use crate::config;
use crate::ctype_exp;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{anyhow, Context, Result};
use colored::*;
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;

/// Add new environment variable for an alias
#[derive(Debug, StructOpt)]
pub struct AddEnv {
    /// Alias to link
    pub alias: String,
    /// Environment variable to link on alias, if not provided alias is used
    pub var: Option<String>,
}

impl RadSubCmdRunnable for AddEnv {
    fn run(&self) -> Result<String> {
        let var = self.var.as_ref().unwrap_or(&self.alias).to_uppercase();

        let aliases_dir = config::rad_aliases_dir().with_context(|| {
            format!(
                "Failed to add: [{}] {} for [{}] {}",
                ctype_exp!("env"),
                var.red().bold(),
                ctype_exp!("alias"),
                self.alias.red().bold()
            )
        })?;
        let mut aliases = Aliases::open(aliases_dir).with_context(|| {
            format!(
                "Failed to add: [{}] {} for [{}] {}",
                ctype_exp!("env"),
                var.red().bold(),
                ctype_exp!("alias"),
                self.alias.red().bold()
            )
        })?;

        if aliases.get(&self.alias).is_none() {
            return Err(anyhow!(format!(
                "[{}] {} doesn't exist. Cannot add [{}] {}",
                ctype_exp!("alias"),
                self.alias.red().bold(),
                ctype_exp!("env"),
                var.red().bold()
            )));
        }
        aliases
            .add_env(self.alias.to_owned(), var.to_owned())
            .with_context(|| {
                format!(
                    "Failed to add: [{}] {} for [{}] {}",
                    ctype_exp!("env"),
                    var.red().bold(),
                    ctype_exp!("alias"),
                    self.alias.red().bold()
                )
            })?;

        Ok(format!(
            "[{}] {} added for [{}] {}",
            ctype_exp!("env"),
            var.red().bold(),
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
    fn not_existing_alias_without_var() {
        let subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test"),
            var: None,
        });
        let res = subcmd.run();
        assert!(res.is_err());
        // "cannot add environment variable 'TEST', no such alias 'test'"
        assert_eq!(
            res.unwrap_err().to_string(),
            "[alias] test doesn't exist. Cannot add [env] TEST",
        );
    }

    #[test]
    #[serial]
    fn not_existing_alias_with_var() {
        let subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test"),
            var: Some(String::from("PROVIDED")),
        });
        let res = subcmd.run();
        assert!(res.is_err());
        // "cannot add environment variable 'PROVIDED', no such alias 'test'"
        assert_eq!(
            res.unwrap_err().to_string(),
            "[alias] test doesn't exist. Cannot add [env] PROVIDED",
        );
    }

    #[test]
    #[serial]
    fn existing_alias_without_var() {
        let mut subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test"),
            var: None,
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            [environment]
        ));
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "[env] TEST added for [alias] test");
    }

    #[test]
    #[serial]
    fn existing_alias_with_var() {
        let mut subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test"),
            var: Some(String::from("PROVIDED")),
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            [environment]
        ));
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "[env] PROVIDED added for [alias] test");
    }

    #[test]
    #[serial]
    fn existing_alias() {
        let mut subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test"),
            var: Some(String::from("PROVIDED")),
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            [environment]
            test = "TEST"
        ));
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Failed to add: [env] PROVIDED for [alias] test"
        );
    }

    #[test]
    #[serial]
    fn existing_var() {
        let mut subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test2"),
            var: Some(String::from("TEST")),
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            test2 = "test"
            [environment]
            test = "TEST"
        ));
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "Failed to add: [env] TEST for [alias] test2"
        );
    }

    #[test]
    #[serial]
    fn var_lower() {
        let mut subcmd = fixture::create_subcmd(AddEnv {
            alias: String::from("test"),
            var: Some(String::from("provided")),
        });
        subcmd.use_config(toml::toml!(
            [aliases]
            test = "test"
            [environment]
        ));
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "[env] PROVIDED added for [alias] test");
    }
}
