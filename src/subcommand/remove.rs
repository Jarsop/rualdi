use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;

/// Remove alias
#[derive(Debug, StructOpt)]
pub struct Remove {
    /// Alias to remove
    pub alias: Vec<String>,
}

impl RadSubCmdRunnable for Remove {
    fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir().with_context(|| "fail to remove alias")?;
        let mut aliases = Aliases::open(aliases_dir).with_context(|| "fail to remove alias")?;

        for alias in &self.alias {
            aliases
                .remove(alias.to_owned())
                .with_context(|| format!("fail to remove alias '{}'", alias))?;
            println!("alias '{}' removed", alias)
        }

        Ok("".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn not_existing_alias() {
        let subcmd = fixture::create_subcmd(Remove {
            alias: vec![String::from("test")],
        });
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "fail to remove alias 'test'");
    }

    #[test]
    #[serial]
    fn existing_alias() {
        let mut subcmd = fixture::create_subcmd(Remove {
            alias: vec![String::from("test")],
        });
        subcmd.use_config(toml::toml!(test = "test"));
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "");
    }

    #[test]
    #[serial]
    fn existing_aliases() {
        let mut subcmd = fixture::create_subcmd(Remove {
            alias: vec![String::from("test"), String::from("test2")],
        });
        subcmd.use_config(toml::toml!(
            test = "test"
            test2 = "test2"
        ));
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "");
    }
}
