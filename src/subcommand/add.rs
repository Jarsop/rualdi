use crate::config;
use crate::ctype_exp;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use crate::utils;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use std::path::PathBuf;
#[cfg(test)]
use std::str::FromStr;
use structopt::StructOpt;
use colored::*;

/// Add new path alias
#[derive(Debug, StructOpt)]
pub struct Add {
    /// Alias to path
    pub alias: String,
    /// Path to aliasing, if not provided current directory is used
    pub path: Option<PathBuf>,
}

impl RadSubCmdRunnable for Add {
    fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir()
            .with_context(|| format!("[{}] Failed to add: {}", ctype_exp!("alias"), self.alias.red().bold()))?;
        let mut aliases = Aliases::open(aliases_dir)
            .with_context(|| format!("[{}] Failed to add: {}", ctype_exp!("alias"), self.alias.red().bold()))?;

        let path = self.path.to_owned().unwrap_or(utils::get_current_dir()?);
        let path = utils::resolve_path(path)
            .with_context(|| format!("[{}] Failed to add: {}", ctype_exp!("alias"), self.alias.red().bold()))?;

        aliases
            .add(self.alias.to_owned(), utils::path_to_str(&path)?.into())
            .with_context(|| format!("[{}] Failed to add: {}", ctype_exp!("alias"), self.alias.red().bold()))?;

        Ok(format!("[{}] Added: {}\n", ctype_exp!("alias"), self.alias.red().bold()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn not_existing_alias() {
        let subcmd = fixture::create_subcmd(Add {
            alias: String::from("test"),
            path: None,
        });
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(),
            format!("[{}] Added: {}\n",
                ctype_exp!("alias"),
                "test".to_string().red().bold()
            )
        );
    }

    #[test]
    #[serial]
    fn not_existing_path() {
        let subcmd = fixture::create_subcmd(Add {
            alias: String::from("test"),
            path: Some(PathBuf::from_str("not-existing-path").unwrap()),
        });
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
            format!("[{}] Failed to add: {}",
                ctype_exp!("alias"),
                "test".to_string().red().bold()
            )
        );
    }

    #[test]
    #[serial]
    fn existing_alias() {
        let mut subcmd = fixture::create_subcmd(Add {
            alias: String::from("test"),
            path: None,
        });
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
        ]);
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(),
            format!("[{}] Added: {}\n",
                ctype_exp!("alias"),
                "test".to_string().red().bold()
            )
        );
    }
}
