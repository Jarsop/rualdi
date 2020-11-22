use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;

/// Print environment variables in format <alias var>
#[derive(Debug, StructOpt)]
pub struct ListEnv {}

impl RadSubCmdRunnable for ListEnv {
    fn run(&self) -> Result<String> {
        let aliases_dir =
            config::rad_aliases_dir().with_context(|| "fail to list environment variables")?;
        let aliases =
            Aliases::open(aliases_dir).with_context(|| "fail to list environment variables")?;

        let res = aliases.list_env();

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn no_vars() {
        let subcmd = fixture::create_subcmd(ListEnv {});
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "");
    }

    #[test]
    #[serial]
    fn var() {
        let mut subcmd = fixture::create_subcmd(ListEnv {});
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
            [environment]
            test = "TEST"
        ]);
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "test TEST\n");
    }

    #[test]
    #[serial]
    fn vars() {
        let mut subcmd = fixture::create_subcmd(ListEnv {});
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
            test2 = "test2"
            [environment]
            test = "TEST"
            test2 = "TEST2"
        ]);
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "test TEST\ntest2 TEST2\n");
    }
}
