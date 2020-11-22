use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;

/// Add new environment variable for an alias
#[derive(Debug, StructOpt)]
pub struct RemoveEnv {
    /// Alias for which to remove the environment variable
    pub alias: String,
}

impl RadSubCmdRunnable for RemoveEnv {
    fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir().with_context(|| {
            format!(
                "fail to remove environment variable for alias '{}'",
                self.alias
            )
        })?;
        let mut aliases = Aliases::open(aliases_dir).with_context(|| {
            format!(
                "fail to remove environment variable for alias '{}'",
                self.alias
            )
        })?;

        aliases.remove_env(self.alias.to_owned()).with_context(|| {
            format!(
                "fail to remove environment variable for alias '{}'",
                self.alias
            )
        })?;

        Ok(format!(
            "environment variable for alias '{}' removed\n",
            self.alias
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
            "environment variable for alias 'test' removed\n"
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
            "fail to remove environment variable for alias 'test'"
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
            "fail to remove environment variable for alias 'test'"
        );
    }
}
