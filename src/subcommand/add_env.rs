use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{anyhow, Context, Result};
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
                "fail to add environment variable '{}' for alias '{}'",
                var, self.alias
            )
        })?;
        let mut aliases = Aliases::open(aliases_dir).with_context(|| {
            format!(
                "fail to add environment variable '{}' for alias '{}'",
                var, self.alias
            )
        })?;

        if aliases.get(&self.alias).is_none() {
            return Err(anyhow!(format!(
                "cannot add environment variable '{}', no such alias '{}'",
                var, self.alias
            )));
        }
        aliases
            .add_env(self.alias.to_owned(), var.to_owned())
            .with_context(|| {
                format!(
                    "fail to add environment variable '{}' for alias '{}'",
                    var, self.alias
                )
            })?;

        Ok(format!(
            "environment variable '{}' for alias '{}' added\n",
            var, self.alias
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
        assert_eq!(
            res.unwrap_err().to_string(),
            "cannot add environment variable 'TEST', no such alias 'test'"
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
        assert_eq!(
            res.unwrap_err().to_string(),
            "cannot add environment variable 'PROVIDED', no such alias 'test'"
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
        assert_eq!(
            res.unwrap(),
            "environment variable 'TEST' for alias 'test' added\n"
        );
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
        assert_eq!(
            res.unwrap(),
            "environment variable 'PROVIDED' for alias 'test' added\n"
        );
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
            "fail to add environment variable 'PROVIDED' for alias 'test'"
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
            "fail to add environment variable 'TEST' for alias 'test2'"
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
        assert_eq!(
            res.unwrap(),
            "environment variable 'PROVIDED' for alias 'test' added\n"
        );
    }
}
