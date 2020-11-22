use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use crate::utils;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
#[cfg(test)]
use std::str::FromStr;
use structopt::StructOpt;

/// Resolve alias
#[derive(Debug, StructOpt)]
pub struct Resolve {
    /// Path to resolve alias
    pub path: PathBuf,
}

impl RadSubCmdRunnable for Resolve {
    fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir()
            .with_context(|| format!("fail to resolve alias path '{}'", self.path.display()))?;
        let aliases = Aliases::open(aliases_dir)
            .with_context(|| format!("fail to resolve alias for path '{}'", self.path.display()))?;

        let path;

        if self.path.is_absolute() {
            path = utils::resolve_path(&self.path)?;
        } else {
            let resolved_path = resolve_alias(&self.path, aliases).with_context(|| {
                format!("fail to resolve alias for path '{}'", self.path.display())
            })?;
            path = utils::resolve_path(&resolved_path)?;
        }
        Ok(format!("{}\n", path.display()))
    }
}

/// Get path radical and search it in config file,
/// return original path if no alias found
fn resolve_alias<P: AsRef<Path>>(path: P, aliases: Aliases) -> Result<PathBuf> {
    let path = path.as_ref();

    let mut components = path.components().peekable();
    let component = components.next().unwrap().as_os_str();
    let to_find = component
        .to_str()
        .with_context(|| format!("invalid utf-8 sequence in alias part: {:?}", component))?;

    let alias = aliases.get(to_find);
    let result = match alias {
        Some(alias) => {
            let mut resolved: Vec<&OsStr> = Vec::new();
            resolved.push(OsStr::new(&alias));
            resolved.extend(
                components
                    .map(|comp| comp.as_os_str())
                    .collect::<Vec<&OsStr>>(),
            );
            let result = resolved.iter().collect::<PathBuf>();
            result
        }
        None => path.to_path_buf(),
    };
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn existing_alias() {
        let current_dir = std::env::current_dir().unwrap();
        let mut subcmd = fixture::create_subcmd(Resolve {
            path: PathBuf::from_str("test").unwrap(),
        });
        subcmd.use_config(toml::toml![
            [aliases]
            test = "not-existing-path"
        ]);
        let res = subcmd.run();
        let expected = format!(
            "could not resolve path: {}/not-existing-path",
            current_dir.to_str().unwrap()
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected);
    }

    #[test]
    #[serial]
    fn existing_path_without_alias() {
        let current_dir = std::env::current_dir().unwrap();
        let subcmd = fixture::create_subcmd(Resolve {
            path: PathBuf::from_str(current_dir.to_str().unwrap()).unwrap(),
        });
        let res = subcmd.run();
        let expected = format!("{}\n", current_dir.to_str().unwrap());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }

    #[test]
    #[serial]
    fn not_existing_path_without_alias() {
        let current_dir = std::env::current_dir().unwrap();
        let subcmd = fixture::create_subcmd(Resolve {
            path: PathBuf::from_str("test").unwrap(),
        });
        let res = subcmd.run();
        let expected = format!(
            "could not resolve path: {}/test",
            current_dir.to_str().unwrap()
        );
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), expected);
    }

    #[test]
    #[serial]
    fn tild_alias() {
        let home_dir = std::env::var("HOME").unwrap();
        let mut subcmd = fixture::create_subcmd(Resolve {
            path: PathBuf::from_str("home").unwrap(),
        });
        subcmd.use_config(toml::toml![
            [aliases]
            home = "~"
        ]);
        let res = subcmd.run();
        let expected = format!("{}\n", home_dir);
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), expected);
    }
}
