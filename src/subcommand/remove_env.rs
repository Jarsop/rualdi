use crate::config;
use crate::ctype_exp;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;
use colored::*;

/// Remove environment variable for a provided alias
#[derive(Debug, StructOpt)]
pub struct RemoveEnv {
    /// Alias for which to remove the environment variable
    pub alias: String,
}

impl RadSubCmdRunnable for RemoveEnv {
    fn run(&self) -> Result<String> {
        // "fail to remove environment variable for alias '{}'",
        // TODO: add color here
        let aliases_dir = config::rad_aliases_dir().with_context(|| {
            format!(
                "[{}] Failed to remove for [{}] {}",
                ctype_exp!("env"),
                ctype_exp!("alias"),
                self.alias.red().bold()
            )
        })?;
        let mut aliases = Aliases::open(aliases_dir).with_context(|| {
            format!(
                "[{}] Failed to remove for [{}] {}",
                ctype_exp!("env"),
                ctype_exp!("alias"),
                self.alias.red().bold()
            )
        })?;

        aliases.remove_env(self.alias.to_owned()).with_context(|| {
            format!(
                "[{}] Failed to remove for [{}] {}",
                ctype_exp!("env"),
                ctype_exp!("alias"),
                self.alias.red().bold()
            )
        })?;

        // "environment variable for alias '{}' removed\n",
        Ok(format!(
            "[{}] Removed: for [{}] {}",
            ctype_exp!("env"),
            ctype_exp!("alias"),
            self.alias.red().bold()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn existing_var() {
        let mut subcmd = fixture::create_subcmd(RemoveEnv {
            alias: String::from("test"),
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            [environment]
            test = "TEST"
        ));
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            format!(
                "[{}] Removed for [{}] {}",
                ctype_exp!("env"),
                ctype_exp!("alias"),
                "test".red().bold()
            )
        );
    }

    #[test]
    #[serial]
    fn not_existing_alias() {
        let subcmd = fixture::create_subcmd(RemoveEnv {
            alias: String::from("test"),
        });
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            format!(
                "[{}] Failed to remove for [{}] {}",
                ctype_exp!("env"),
                ctype_exp!("alias"),
                "test".red().bold()
            )
        );
    }

    #[test]
    #[serial]
    fn not_existing_var() {
        let mut subcmd = fixture::create_subcmd(RemoveEnv {
            alias: String::from("test"),
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            [environment]
        ));
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            format!(
                "[{}] Failed to remove for [{}] {}",
                ctype_exp!("env"),
                ctype_exp!("alias"),
                "test".red().bold()
            )
        );
    }
}
