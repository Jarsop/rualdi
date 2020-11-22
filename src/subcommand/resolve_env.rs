use crate::config;
#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::StructOpt;

/// Resolve enironment variable from alias
#[derive(Debug, StructOpt)]
pub struct ResolveEnv {
    /// Alias for which to find environment variable
    pub alias: String,
}

impl RadSubCmdRunnable for ResolveEnv {
    fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir().with_context(|| {
            format!(
                "fail to resolve environment variable for alias '{}'",
                self.alias
            )
        })?;
        let aliases = Aliases::open(aliases_dir).with_context(|| {
            format!(
                "fail to resolve environment variable for alias '{}'",
                self.alias
            )
        })?;

        let resolved_var = aliases.get_env(&self.alias).with_context(|| {
            format!(
                "fail to resolve environment variable for alias '{}'",
                self.alias
            )
        })?;
        Ok(format!("{}\n", resolved_var))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn existing_var() {
        let mut subcmd = fixture::create_subcmd(ResolveEnv {
            alias: String::from("test"),
        });
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
            [environment]
            test = "TEST"
        ]);
        let res = subcmd.run();
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "TEST\n");
    }

    #[test]
    #[serial]
    fn not_existing_var() {
        let mut subcmd = fixture::create_subcmd(ResolveEnv {
            alias: String::from("test"),
        });
        subcmd.use_config(toml::toml![
            [aliases]
            test = "test"
        ]);
        let res = subcmd.run();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "fail to resolve environment variable for alias 'test'"
        );
    }
}
