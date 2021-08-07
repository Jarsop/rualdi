//! Module to parse rad config file in TOML format
use anyhow::{anyhow, Context, Result};
use serde_derive::{Deserialize, Serialize};
use shellexpand::{tilde, full, LookupError};
use terminal_size::terminal_size;

use std::{
    collections::BTreeMap,
    fs,
    env,
    path::{Path, PathBuf},
    io::prelude::*,
    borrow::Cow
};

// A hash that keeps its order
use indexmap::IndexMap;

#[cfg(test)]
use std::{
    fs::File,
    io::Write,
};

#[cfg(test)]
use tempfile::{Builder, TempDir};
use colored::*;
use regex::{Regex, Captures};

/// Contain aliases and assiociated path
/// ```
/// use serde_derive::{Serialize,Deserialize};
/// use std::collections::BTreeMap;
///
/// #[derive(Serialize, Deserialize, Debug)]
/// pub struct Aliases {
///     aliases: Option<BTreeMap<String, String>>,
/// }
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Aliases {
    pub aliases: Option<BTreeMap<String, String>>,
    #[serde(rename = "environment")]
    pub vars: Option<BTreeMap<String, String>>,
    pub colors: Option<BTreeMap<String, String>>,
    pub alias_hash: Option<BTreeMap<String, String>>,
    #[serde(skip)]
    modified: bool,
    #[serde(skip)]
    aliases_file: PathBuf,
}

/// Get alias from rad TOML structure
/// ```
/// # use anyhow::Result;
/// # fn main() -> Result<()> {
/// # use rualdlib::Aliases;
/// let home = std::env::var("HOME").unwrap();
/// let aliases: Aliases = toml::from_str(r#"
/// [aliases]
/// Home = '~'
/// workdir = '~/workdir'
/// local = '/usr/local'
/// "#).unwrap();
///
/// let alias = "Home";
/// let pointed = aliases.get(alias);
///
/// assert_eq!(pointed, Some(home));
/// # Ok(())
/// # }
/// ```
impl Aliases {
    /// Open rualdi aliases file from default aliases directory,
    /// default directory can be configured by _RAD_ALIASES_DIR
    /// env variable.
    pub fn open(aliases_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&aliases_dir).with_context(|| {
            format!(
                "unable to create config directory: '{}'",
                aliases_dir.display()
            )
        })?;

        let path = Self::get_path(&aliases_dir);

        if !path.is_file() {
            let default_file = r#"# Rualdi aliases configuration file
# DO NOT EDIT VARIABLES AND ALIASES
[colors]
alias = "yellow"
separator = "bright cyan"
path = "magenta"
"#;
            let mut aliases_file: fs::File = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .with_context(|| format!("could not create alias file: '{}'", path.display()))?;

            aliases_file
                .write_all(default_file.as_bytes())
                .with_context(|| format!("could not create alias file: '{}'", path.display()))?;
            aliases_file.flush()?;
        }

        let mut aliases_file = fs::File::open(&path)
            .with_context(|| format!("could not open alias file: '{}'", path.display()))?;

        let mut content = String::new();

        aliases_file.read_to_string(&mut content)?;

        let mut aliases: Aliases = toml::from_str(&content)
            .with_context(|| format!("could not open alias file: '{}'", path.display()))?;
        aliases.modified = false;
        aliases.aliases_file = path;
        Ok(aliases)
    }

    /// Save rualdi aliases file in default aliases directory,
    /// default directory can be configured by _RAD_ALIASES_DIR
    /// env variable.
    pub fn save(&self) -> Result<()> {
        if !self.modified {
            Ok(())
        } else {
            let mut aliases_file: fs::File = fs::OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.aliases_file)
                .with_context(|| {
                    format!(
                        "could not save alias file: '{}'",
                        &self.aliases_file.display()
                    )
                })?;
            aliases_file.sync_data()?;

            let mut content = String::new();
            content.push_str("# Rualdi aliases configuration file\n# DO NOT EDIT VARIABLES AND ALIASES\n");

            let data = toml::to_string(&self).with_context(|| "fail to encode aliases in toml")?;
            content.push_str(data.as_str());
            aliases_file
                .write_all(content.as_bytes())
                .with_context(|| {
                    format!(
                        "fail to save aliases file '{}'",
                        self.aliases_file.display()
                    )
                })?;
            aliases_file.sync_all()?;
            Ok(())
        }
    }

    /// Add alias on path in aliase configuration file, raise an error if alias
    /// already exists.
    pub fn add(&mut self, alias: String, path: String) -> Result<()> {
        let mut aliases = self.selfmatch(self.aliases.to_owned());
        let colors = self.selfmatch(self.colors.to_owned());
        let alias_hash = self.selfmatch(self.alias_hash.to_owned());

        if aliases.contains_key(&alias) {
            return Err(anyhow!("alias '{}' already exists", alias));
        }

        aliases.insert(alias, path);

        self.aliases = Some(aliases);
        self.alias_hash = Some(alias_hash);
        self.colors = Some(colors);
        self.modified = true;
        Ok(())
    }

    /// Add environment variable assiociated with an alias configuration file
    /// to load it in shell environment, raise an error if environment variable
    /// already exists.
    pub fn add_env(&mut self, alias: String, var_name: String) -> Result<()> {
        let mut vars = self.selfmatch(self.vars.to_owned());
        let colors = self.selfmatch(self.colors.to_owned());
        let alias_hash = self.selfmatch(self.alias_hash.to_owned());

        if vars.contains_key(&alias) {
            return Err(anyhow!(
                "alias '{}' has already a environment variable assiociated",
                alias
            ));
        }

        let values: Vec<String> = vars.values().cloned().collect();
        if values.contains(&var_name) {
            return Err(anyhow!(
                "environment variable '{}' for alias '{}' already exists",
                var_name,
                alias
            ));
        }

        vars.insert(alias, var_name);

        self.vars = Some(vars);
        self.alias_hash = Some(alias_hash);
        self.colors = Some(colors);
        self.modified = true;
        Ok(())
    }

    /// Remove alias on path in aliase configuration file, raise an error if alias
    /// not exists.
    pub fn remove(&mut self, alias: String) -> Result<()> {
        let mut aliases = self.selfmatch(self.aliases.to_owned());
        let colors = self.selfmatch(self.colors.to_owned());
        let alias_hash = self.selfmatch(self.alias_hash.to_owned());

        if !aliases.contains_key(&alias) {
            return Err(anyhow!("alias '{}' not exists", alias));
        }

        aliases.remove(&alias);

        self.aliases = Some(aliases);
        self.alias_hash = Some(alias_hash);
        self.colors = Some(colors);
        self.modified = true;
        Ok(())
    }

    /// Remove environment variable associated to an alias
    /// in aliase configuration file, raise an error if variable
    /// not exists.
    pub fn remove_env(&mut self, alias: String) -> Result<()> {
        let mut vars = self.selfmatch(self.vars.to_owned());
        let colors = self.selfmatch(self.colors.to_owned());
        let alias_hash = self.selfmatch(self.alias_hash.to_owned());

        if !vars.contains_key(&alias) {
            return Err(anyhow!(
                "no such environment variable for alias '{}'",
                alias
            ));
        }

        vars.remove(&alias);

        self.vars = Some(vars);
        self.alias_hash = Some(alias_hash);
        self.colors = Some(colors);
        self.modified = true;
        Ok(())
    }

    /// Get a formatted String conaining aliases/paths
    /// found in configuration file
    pub fn list(&self) -> Option<String> {
        if let Some(aliases) = &self.aliases {
            if aliases.is_empty() {
                None
            } else {
                // TODO: test invalid environment var
                // TODO: test no environment vars with header section
                // TODO: test no environment vars without header section
                // If default is off and there are no aliases listed, insert the home directory
                // mapping with itself to prevent errors
                let alias_hash = self.build_alias_hash()
                    .unwrap_or_else(|| {
                        let mut tmp = IndexMap::new();
                        tmp.insert(
                            dirs::home_dir().unwrap_or(PathBuf::from("INVALID_HOME_DIR"))
                                .into_os_string().into_string().unwrap(),
                            dirs::home_dir().unwrap_or(PathBuf::from("INVALID_HOME_DIR"))
                                .into_os_string().into_string().unwrap()
                        );
                        tmp
                    }
                );

                let mut reg = Vec::new();
                for (k, _) in alias_hash.iter() {
                    reg.push(k.to_string());
                }

                let width = terminal_size().map(|(w, _)| w.0 as usize).unwrap_or(0);
                let mut res = String::new();

                res.push_str(format!("{}\n{: ^width$}\n{}\n",
                        "=".repeat(width).green().bold(),
                        "ALIASES".red().bold(),
                        "=".repeat(width).green().bold(),
                        width = width - 1)
                    .as_str()
                    );

                let color_alias = self.get_colors("alias").unwrap_or(Color::Yellow);
                let color_separator = self.get_colors("separator").unwrap_or(Color::BrightCyan);
                let color_path = self.get_colors("path").unwrap_or(Color::Magenta);

                let re = Regex::new(format!(r"({})", reg.join("|")).as_str()).unwrap();
                for (alias, path) in aliases.iter() {
                    let new_path = if re.is_match(path) {
                        re.replace(path, |caps: &Captures| {
                            alias_hash.get(caps.get(1).unwrap().as_str()).unwrap()
                        })
                    } else {
                        Cow::from(path)
                    };
                    res.push_str(format!("{:<12} {:<2} {}\n",
                            alias.color(color_alias).bold(),
                            "=>".color(color_separator).bold(),
                            new_path.color(color_path))
                        .as_str()
                        );
                }

                if let Some(vars) = &self.vars {
                    if !vars.is_empty() {
                        res.push_str(format!("{}\n{: ^width$}\n{}\n",
                                "=".repeat(width).green().bold(),
                                "ENVIRONMENT VARIABLES".red().bold(),
                                "=".repeat(width).green().bold(),
                                width = width - 1)
                            .as_str()
                            );
                            for (alias, var) in vars.iter() {
                                res.push_str(format!("{:<12} {:<2} {}\n",
                                        var.color(color_alias).bold(),
                                        "=>".color(color_separator).bold(),
                                        alias.color(color_path))
                                    .as_str()
                                    );
                        }
                    }
                }
                Some(res)
            }
        } else {
            None
        }
    }

    /// Get a String conaining aliases/vars
    /// found in configuration file
    pub fn list_env(&self) -> String {
        let mut vars_found = String::new();
        if let Some(vars) = &self.vars {
            if vars.is_empty() {
                vars_found
            } else {
                for (alias, var) in vars.iter() {
                    vars_found.push_str(format!(
                            "{} {} {}\n",
                            alias.yellow(),
                            "=>".bright_cyan(),
                            var.magenta())
                        .as_str()
                    );
                }
                vars_found
            }
        } else {
            vars_found
        }
    }

    pub fn list_alias(&self) -> Option<String> {
        if let Some(aliases) = &self.aliases {
            if aliases.is_empty() {
                None
            } else {
                let mut res = String::new();
                for (alias, path) in aliases.iter() {
                    res.push_str(format!("{} => {}\n", alias, path).as_str());
                }
                Some(res)
            }
        } else {
            None
        }
    }

    pub fn list_alias_completions(&self) -> Option<String> {
        if let Some(aliases) = &self.aliases {
            if aliases.is_empty() {
                None
            } else {
                let mut res = String::new();
                for (alias, _) in aliases.iter() {
                    res.push_str(format!("{}\n", alias).as_str());
                }
                Some(res)
            }
        } else {
            None
        }
    }

    pub fn list_env_completions(&self) -> Option<String> {
        if let Some(vars) = &self.vars {
            if vars.is_empty() {
                None
            } else {
                let mut res = String::new();
                for (_, var) in vars.iter() {
                    res.push_str(format!("{}\n", var).as_str());
                }
                Some(res)
            }
        } else {
            None
        }
    }

    /// Get rualdi configuration path with rualdi configuration
    /// file name concatenate
    fn get_path<P: AsRef<Path>>(aliases_dir: P) -> PathBuf {
        aliases_dir.as_ref().join("rualdi.toml")
    }

    /// Helper function to prevent having  to type match statement
    fn selfmatch(&mut self, matching: Option<BTreeMap<String, String>>
    ) -> BTreeMap<String, String>
    {
        match matching {
            Some(matching) => matching,
            _ => {
                self.modified = true;
                BTreeMap::new()
            }
        }
    }

    /// Build an alias-directory mapping hash to shorten the lengths of paths
    fn build_alias_hash(&self) -> Option<IndexMap<String, String>> {
        /// Helper function to get different directories for macOS specifically
        /// Example: `cache_dir()` returns `$HOME/Library/Caches`, when this will return `$HOME/.cache`
        fn macos_dirs(dir_func: Option<PathBuf>, joined: &str) -> PathBuf {
            if env::consts::OS == "macos" {
                PathBuf::from(env!("HOME")).join(joined)
            } else {
                dir_func.unwrap_or(
                    PathBuf::from(format!(
                            "INVALID_{}_DIR",
                            joined.to_uppercase()
                            ))
                    )
            }
        }

        if let Some(alias_hash) = &self.alias_hash {
            if alias_hash.is_empty() {
                None
            } else {
                let mut new_alias_hash = IndexMap::new();

                // Configuration set variables need to be parsed first to set the variables before
                // the default does. The IndexMap will keep the order
                for short in alias_hash.keys() {
                    new_alias_hash.insert(
                        PathBuf::from(full(
                                alias_hash.get(short).unwrap()
                            ).unwrap_or_else(|_| {
                                Cow::from(LookupError {
                                    var_name: "UNKNOWN_ENVIRONMENT_VARIABLE".into(),
                                    cause: env::VarError::NotPresent
                                }.to_string())
                            }).to_string()
                            ).into_os_string().into_string().unwrap(),
                        format!("%{}", short)
                    );
                }

                if Regex::new(r"on|yes|1").unwrap().is_match(
                    alias_hash.get("use_default")
                        .unwrap_or(&"N/A".to_string())
                        .as_str())
                {
                    // Should match on every system
                    // Also, the INVALID_... is so:
                        // 1) There is not a duplicate key
                        // 2) I'm hoping that there is not a path containing that name
                    new_alias_hash.insert(
                        dirs::audio_dir().unwrap_or(PathBuf::from("INVALID_AUDIO_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%AUDIO_HOME".to_string()
                    );

                    new_alias_hash.insert(
                        macos_dirs(dirs::cache_dir(), ".cache")
                            .into_os_string().into_string().unwrap(),
                        "%CACHE_HOME".to_string()
                    );

                    new_alias_hash.insert(
                        macos_dirs(dirs::config_dir(), ".config")
                            .into_os_string().into_string().unwrap(),
                        "%CONFIG_HOME".to_string()
                    );

                    new_alias_hash.insert(
                        macos_dirs(dirs::data_dir(), ".local/share")
                            .into_os_string().into_string().unwrap(),
                        "%DATA_HOME".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::desktop_dir().unwrap_or(PathBuf::from("INVALID_DESKTOP_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%DESKTOP".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::document_dir().unwrap_or(PathBuf::from("INVALID_DOCUMENT_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%DOCUMENTS".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::download_dir().unwrap_or(PathBuf::from("INVALID_DOWNLOAD_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%DOWNLOADS".to_string()
                    );

                    new_alias_hash.insert(
                        macos_dirs(dirs::executable_dir(), ".local/bin")
                            .into_os_string().into_string().unwrap(),
                        "%BIN_HOME".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::font_dir().unwrap_or(PathBuf::from("INVALID_FONT_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%FONTS".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::picture_dir().unwrap_or(PathBuf::from("INVALID_PICTURE_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%PICTURES".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::public_dir().unwrap_or(PathBuf::from("INVALID_PUBLIC_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%PUBLIC".to_string()
                    );

                    new_alias_hash.insert(
                        macos_dirs(dirs::runtime_dir(), ".local/tmp")
                            .into_os_string().into_string().unwrap(),
                        "%RUNTIME_DIR".to_string()
                    );

                    new_alias_hash.insert(
                        macos_dirs(dirs::template_dir(), "Templates")
                            .into_os_string().into_string().unwrap(),
                        "%TEMPLATE_DIR".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::video_dir().unwrap_or(PathBuf::from("INVALID_VIDEO_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%VIDEO_DIR".to_string()
                    );

                    new_alias_hash.insert(
                        dirs::home_dir().unwrap_or(PathBuf::from("INVALID_HOME_DIR"))
                            .into_os_string().into_string().unwrap(),
                        "%HOME".to_string()
                    );
                }

                Some(new_alias_hash)
            }
        } else {
            None
        }
    }

    /// Get colors from configuration to colorize output
    fn get_colors(&self, colored: &str) -> Option<Color> {
        if let Some(colors) = &self.colors {
            if colors.is_empty() {
                None
            } else if let Some(color) = colors.get(colored) {
                // Unwraps to white if invalid
                Some(Color::from(color.as_str()))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Search alias in rualdi aliases configuration file,
    /// return None if alias not found
    pub fn get(&self, alias: &str) -> Option<String> {
        if let Some(aliases) = &self.aliases {
            if aliases.is_empty() {
                None
            } else if let Some(path) = aliases.get(alias) {
                Some(tilde(path).to_string())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Search environment variable associated to alias in
    /// rualdi aliases configuration file,
    /// return None if variable not found
    pub fn get_env(&self, alias: &str) -> Result<String> {
        if let Some(vars) = &self.vars {
            if vars.is_empty() {
                Err(anyhow!(format!("no variable for alias '{}'", alias)))
            } else if let Some(var) = vars.get(alias) {
                Ok(var.to_owned())
            } else {
                Err(anyhow!(format!("no variable for alias '{}'", alias)))
            }
        } else {
            Err(anyhow!(format!("no variable for alias '{}'", alias)))
        }
    }
}

impl Drop for Aliases {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("{:#}", e);
        }
    }
}

#[cfg(test)]
pub struct MockAliases;

#[cfg(test)]
impl MockAliases {
    pub fn open() -> Aliases {
        let mut aliases: BTreeMap<String, String> = BTreeMap::new();
        aliases.insert("test".into(), "/test/haha".into());
        aliases.insert("Home".into(), "~".into());

        let mut colors: BTreeMap<String, Color> = BTreeMap::new();
        colors.insert("name".into(), "red".into());
        colors.insert("separator".into(), "cyan".into());
        colors.insert("path".into(), "green".into());

        Aliases {
            aliases: Some(aliases),
            vars: None,
            colors: Some(colors),
            alias_hash: None,
            modified: false,
            aliases_file: PathBuf::new(),
        }
    }

    pub fn open_with_env() -> Aliases {
        let mut aliases: BTreeMap<String, String> = BTreeMap::new();
        aliases.insert("test".into(), "/test/haha".into());
        aliases.insert("Home".into(), "~".into());

        let mut vars: BTreeMap<String, String> = BTreeMap::new();
        vars.insert("test".into(), "TEST".into());

        let mut colors: BTreeMap<String, Color> = BTreeMap::new();
        colors.insert("name".into(), "red".into());
        colors.insert("separator".into(), "bright cyan".into());
        colors.insert("path".into(), "green".into());

        Aliases {
            aliases: Some(aliases),
            vars: Some(vars),
            colors: Some(colors),
            alias_hash: None,
            modified: false,
            aliases_file: PathBuf::new(),
        }
    }

    pub fn open_with_vars() -> Aliases {
        let mut aliases: BTreeMap<String, String> = BTreeMap::new();
        aliases.insert("test".into(), "/test/haha".into());
        aliases.insert("test2".into(), "/test2/haha".into());
        aliases.insert("Home".into(), "~".into());

        let mut vars: BTreeMap<String, String> = BTreeMap::new();
        vars.insert("test".into(), "TEST".into());
        vars.insert("test2".into(), "TEST2".into());

        let mut colors: BTreeMap<String, Color> = BTreeMap::new();
        colors.insert("name".into(), "red".into());
        colors.insert("separator".into(), "cyan".into());
        colors.insert("path".into(), "green".into());

        Aliases {
            aliases: Some(aliases),
            vars: Some(vars),
            colors: Some(colors),
            alias_hash: None,
            modified: false,
            aliases_file: PathBuf::new(),
        }
    }

    pub fn open_no_aliases() -> Aliases {
        let aliases: BTreeMap<String, String> = BTreeMap::new();
        let vars: BTreeMap<String, String> = BTreeMap::new();

        let mut colors: BTreeMap<String, Color> = BTreeMap::new();
        colors.insert("name".into(), "red".into());
        colors.insert("separator".into(), "cyan".into());
        colors.insert("path".into(), "green".into());

        Aliases {
            aliases: Some(aliases),
            vars: Some(vars),
            colors: Some(colors),
            alias_hash: None,
            modified: false,
            aliases_file: PathBuf::new(),
        }
    }

    pub fn open_no_colors() -> Aliases {
        let mut aliases: BTreeMap<String, String> = BTreeMap::new();
        aliases.insert("test".into(), "/test/haha".into());
        aliases.insert("test2".into(), "/test2/haha".into());
        aliases.insert("Home".into(), "~".into());

        let mut vars: BTreeMap<String, String> = BTreeMap::new();
        vars.insert("test".into(), "TEST".into());
        vars.insert("test2".into(), "TEST2".into());

        let mut colors: BTreeMap<String, Color> = BTreeMap::new();

        Aliases {
            aliases: Some(aliases),
            vars: Some(vars),
            colors: None,
            alias_hash: None,
            modified: false,
            aliases_file: PathBuf::new(),
        }
    }

    pub fn open_empty() -> Aliases {
        Aliases {
            aliases: None,
            vars: None,
            colors: None,
            alias_hash: None,
            modified: false,
            aliases_file: PathBuf::new(),
        }
    }
}

#[cfg(test)]
pub struct TmpConfig {
    pub tmp_dir: TempDir,
    pub tmp_file: File,
}

#[cfg(test)]
impl TmpConfig {
    pub fn create_dir() -> Result<Self> {
        let tmp_dir = Builder::new().prefix("test_rualdi").tempdir()?;
        let file_path = tmp_dir.path().join(".default");
        let tmp_file = File::create(file_path)?;
        Ok(TmpConfig { tmp_dir, tmp_file })
    }

    pub fn with_empty(mut self) -> Result<Self> {
        let file_path = self.tmp_dir.path().join("rualdi.toml");
        self.tmp_file = File::create(file_path)?;
        Ok(self)
    }

    pub fn with_base(mut self) -> Result<Self> {
        let file_path = self.tmp_dir.path().join("rualdi.toml");
        self.tmp_file = File::create(file_path)?;
        writeln!(
            self.tmp_file,
            "# Rualdi aliases configuration file\n# DO NOT EDIT VARIABLES AND ALIASES\n"
        )?;
        Ok(self)
    }

    pub fn with_content(mut self, toml: toml::value::Value) -> Result<Self> {
        self.tmp_file.write_all(toml.to_string().as_bytes())?;
        self.tmp_file.flush()?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests_get {
    use super::*;

    #[test]
    fn simple() {
        let alias = "test";
        let aliases = MockAliases::open();
        let pointed = aliases.get(alias);
        assert_eq!(pointed, Some("/test/haha".into()));
    }

    #[test]
    fn tild() {
        let alias = "Home";
        let home = std::env::var("HOME").unwrap();
        let aliases = MockAliases::open();
        let pointed = aliases.get(alias);
        assert_eq!(pointed, Some(home));
    }

    #[test]
    fn not_existing() {
        let alias = "should_fail";
        let aliases = MockAliases::open();
        let pointed = aliases.get(alias);
        assert_eq!(pointed, None);
    }

    #[test]
    fn from_no_aliases() {
        let alias = "should_fail";
        let aliases = MockAliases::open_no_aliases();
        let pointed = aliases.get(alias);
        assert_eq!(pointed, None);
    }

    #[test]
    fn from_empty_aliases() {
        let alias = "should_fail";
        let aliases = MockAliases::open_empty();
        let pointed = aliases.get(alias);
        assert_eq!(pointed, None);
    }
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn to_empty_aliases() {
        let alias = String::from("test");
        let path = String::from("/test");
        let mut expected_aliases: BTreeMap<String, String> = BTreeMap::new();

        expected_aliases.insert(alias.to_owned(), path.to_owned());

        let mut aliases = MockAliases::open_no_aliases();
        let res = aliases.add(alias, path);
        assert!(res.is_ok());
        assert_eq!(aliases.aliases, Some(expected_aliases));
    }

    #[test]
    fn to_none_aliases() {
        let alias = String::from("test");
        let path = String::from("/test");
        let mut expected_aliases: BTreeMap<String, String> = BTreeMap::new();

        expected_aliases.insert(alias.to_owned(), path.to_owned());

        let mut aliases = MockAliases::open_empty();
        let res = aliases.add(alias, path);
        assert!(res.is_ok());
        assert_eq!(aliases.aliases, Some(expected_aliases));
    }

    #[test]
    fn to_filled_aliases() {
        let alias = String::from("test2");
        let path = String::from("/test");
        let mut expected_aliases: BTreeMap<String, String> = BTreeMap::new();

        expected_aliases.insert(alias.to_owned(), path.to_owned());
        expected_aliases.insert("test".into(), "/test/haha".into());
        expected_aliases.insert("Home".into(), "~".into());

        let mut aliases = MockAliases::open();
        let res = aliases.add(alias, path);
        assert!(res.is_ok());
        assert_eq!(aliases.aliases, Some(expected_aliases));
    }

    #[test]
    fn existing() {
        let alias = String::from("test");
        let path = String::from("/test");
        let mut aliases = MockAliases::open();
        let res = aliases.add(alias, path);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod tests_remove {
    use super::*;

    #[test]
    fn existing() {
        let alias = String::from("test");
        let mut aliases = MockAliases::open();
        let res = aliases.remove(alias);
        assert!(res.is_ok());
    }

    #[test]
    fn not_existing() {
        let alias = String::from("not_exsting");
        let mut aliases = MockAliases::open_no_aliases();
        let res = aliases.remove(alias);
        assert!(res.is_err());
    }

    #[test]
    fn from_empty_aliases() {
        let alias = String::from("not_exsting");
        let mut aliases = MockAliases::open_empty();
        let res = aliases.remove(alias);
        assert!(res.is_err());
    }

    #[test]
    fn from_none_aliases() {
        let alias = String::from("not_exsting");
        let mut aliases = MockAliases::open_no_aliases();
        let res = aliases.remove(alias);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod tests_get_env {
    use super::*;

    #[test]
    fn existing() {
        let alias = "test";
        let aliases = MockAliases::open_with_env();
        let ret = aliases.get_env(alias);
        assert!(ret.is_ok());
        assert_eq!(ret.unwrap(), String::from("TEST"));
    }

    #[test]
    fn not_existing() {
        let alias = "NOPE";
        let aliases = MockAliases::open_with_env();
        let ret = aliases.get_env(alias);
        assert!(ret.is_err());
    }

    #[test]
    fn from_no_env() {
        let alias = "should_fail";
        let aliases = MockAliases::open_no_aliases();
        let ret = aliases.get_env(alias);
        assert!(ret.is_err());
    }

    #[test]
    fn from_empty_env() {
        let alias = "should_fail";
        let aliases = MockAliases::open_empty();
        let ret = aliases.get_env(alias);
        assert!(ret.is_err());
    }
}

#[cfg(test)]
mod tests_add_env {
    use super::*;

    #[test]
    fn not_existing_var() {
        let mut aliases = MockAliases::open();
        let alias = String::from("Home");
        let var = String::from("MY_HOME");
        let mut expected_vars: BTreeMap<String, String> = BTreeMap::new();

        expected_vars.insert(alias.to_owned(), var.to_owned());

        let res = aliases.add_env(alias, var);
        assert!(res.is_ok());
        assert_eq!(aliases.vars, Some(expected_vars));
    }

    #[test]
    fn existing_var() {
        let mut aliases = MockAliases::open_with_env();
        let alias = String::from("test1");
        let var = String::from("TEST");
        let res = aliases.add_env(alias, var);
        assert!(res.is_err());
    }

    #[test]
    fn existing_alias() {
        let mut aliases = MockAliases::open_with_env();
        let alias = String::from("test");
        let var = String::from("TEST1");
        let res = aliases.add_env(alias, var);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod tests_remove_env {
    use super::*;

    #[test]
    fn existing() {
        let alias = String::from("test");
        let mut aliases = MockAliases::open_with_env();
        let res = aliases.remove_env(alias);
        assert!(res.is_ok());
    }

    #[test]
    fn not_existing() {
        let alias = String::from("not_exsting");
        let mut aliases = MockAliases::open_with_env();
        let res = aliases.remove_env(alias);
        assert!(res.is_err());
    }

    #[test]
    fn from_empty_env() {
        let alias = String::from("not_exsting");
        let mut aliases = MockAliases::open_empty();
        let res = aliases.remove_env(alias);
        assert!(res.is_err());
    }

    #[test]
    fn from_none_env() {
        let alias = String::from("not_exsting");
        let mut aliases = MockAliases::open_no_aliases();
        let res = aliases.remove_env(alias);
        assert!(res.is_err());
    }
}

#[cfg(test)]
mod tests_list {
    use super::*;

    #[test]
    fn list_filled() {
        let aliases = MockAliases::open();
        let output = aliases.list();
        let expected_output = "Aliases:\n\n\t'Home' => '~'\n\t'test' => '/test/haha'\n";
        assert!(output.is_some());
        assert_eq!(output.unwrap(), expected_output);
    }

    #[test]
    fn list_filled_env() {
        let aliases = MockAliases::open_with_env();
        let output = aliases.list();
        let expected_output = "Aliases:\n\n\t'Home' => '~'\n\t'test' => '/test/haha'\n\nEnvironment variables:\n\n\t'TEST' => 'test'\n";
        assert!(output.is_some());
        assert_eq!(output.unwrap(), expected_output);
    }

    #[test]
    fn list_empty() {
        let aliases = MockAliases::open_empty();
        let output = aliases.list();
        assert!(output.is_none());
    }

    #[test]
    fn list_no_aliases() {
        let aliases = MockAliases::open_no_aliases();
        let output = aliases.list();
        assert!(output.is_none());
    }
}

#[cfg(test)]
mod tests_list_env {
    use super::*;

    #[test]
    fn list_filled_var() {
        let aliases = MockAliases::open_with_env();
        let output = aliases.list_env();
        let expected_output = "test TEST\n";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn list_filled_vars() {
        let aliases = MockAliases::open_with_vars();
        let output = aliases.list_env();
        let expected_output = "test TEST\ntest2 TEST2\n";
        assert_eq!(output, expected_output);
    }

    #[test]
    fn list_empty() {
        let aliases = MockAliases::open_empty();
        let output = aliases.list_env();
        assert_eq!(output, "");
    }
}

#[cfg(test)]
mod test_open {
    use super::*;

    #[test]
    fn open_config_not_existing() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?;
        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        let expected_aliases = MockAliases::open_empty();
        assert!(aliases.is_ok());
        assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
        Ok(())
    }

    #[test]
    fn open_config_empty_aliases() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?.with_base()?;

        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        let expected_aliases = MockAliases::open_empty();
        assert!(aliases.is_ok());
        assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
        Ok(())
    }

    #[test]
    fn open_config_empty_file() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?.with_empty()?;
        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        let expected_aliases = MockAliases::open_empty();
        assert!(aliases.is_ok());
        assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
        Ok(())
    }

    #[test]
    fn open_config_filled() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?
            .with_base()?
            .with_content(toml::toml![
                [aliases]
                test = "/test/haha"
                Home = "~"
            ])?;

        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        let expected_aliases = MockAliases::open();
        assert!(aliases.is_ok());
        assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
        Ok(())
    }

    #[test]
    #[should_panic]
    fn open_dir_not_existing() {
        Aliases::open(PathBuf::from("/no-existant-path")).unwrap();
    }
}

#[cfg(test)]
mod test_save {
    use super::*;

    #[test]
    fn should_saved() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?
            .with_base()?
            .with_content(toml::toml![
                [aliases]
                test = "/test/haha"
                Home = "~"
            ])?;

        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        assert!(aliases.is_ok());
        let saved = aliases.unwrap().save();
        assert!(saved.is_ok());
        Ok(())
    }

    #[test]
    fn modified_should_saved() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?
            .with_base()?
            .with_content(toml::toml![
                [aliases]
                test = "/test/haha"
                Home = "~"
            ])?;

        let alias = String::from("saved");
        let path = String::from("/saved");

        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        assert!(aliases.is_ok());
        let mut aliases = aliases.unwrap();
        aliases.add(alias, path)?;
        let saved = aliases.save();
        assert!(saved.is_ok());
        Ok(())
    }

    #[test]
    fn should_not_opened() -> Result<()> {
        let aliases_file = TmpConfig::create_dir()?
            .with_base()?
            .with_content(toml::toml![
                [aliases]
                test = "/test/haha"
                Home = "~"
            ])?;

        let aliases = Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
        assert!(aliases.is_ok());
        let mut aliases = aliases.unwrap();
        aliases.aliases_file = Path::new("/not/existing/path").to_path_buf();
        aliases.modified = true;
        let saved = aliases.save();
        assert!(saved.is_err());
        Ok(())
    }
}
