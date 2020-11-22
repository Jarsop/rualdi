use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;

/// Print aliases with their path and environment variable associated
#[derive(Debug, StructOpt)]
pub struct List {}

impl RadSubCmdRunnable for List {
    fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir().with_context(|| "fail to list aliases")?;
        let aliases = Aliases::open(aliases_dir).with_context(|| "fail to list aliases")?;

        let res = aliases
            .list()
            .unwrap_or_else(|| "No aliases found\n".into());

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn no_aliases() {
        let subcmd = fixture::create_subcmd(List {});
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "No aliases found\n");
    }

    #[test]
    #[serial]
    fn alias() {
        let mut subcmd = fixture::create_subcmd(List {});
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
        ]);
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Aliases:\n\n\t'test' => 'test'\n");
    }

    #[test]
    #[serial]
    fn aliases() {
        let mut subcmd = fixture::create_subcmd(List {});
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
            test2 = "test2"
        ]);
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            "Aliases:\n\n\t'test' => 'test'\n\t'test2' => 'test2'\n"
        );
    }

    #[test]
    #[serial]
    fn vars() {
        let mut subcmd = fixture::create_subcmd(List {});
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
            test2 = "test2"
            [environment]
            test = "TEST"
        ]);
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(
            res.unwrap(),
            "Aliases:\n\n\t'test' => 'test'\n\t'test2' => 'test2'\n\nEnvironment variables:\n\n\t'TEST' => 'test'\n"
        );
    }
}
