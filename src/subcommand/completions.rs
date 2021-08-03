use crate::{
    comp_helper,
    config,
    subcommand::RadSubCmdRunnable
};

// env,
// str::FromStr,

use std::{io::Cursor, str::FromStr};

#[cfg(test)]
use crate::fixture;
use anyhow::{anyhow, Context, Result};
use rualdlib::Aliases;
#[cfg(test)]
use serial_test::serial;
use structopt::{
    StructOpt,
    clap::{arg_enum, Shell},
};

use crate::Rad;
// use colored::*;

// TODO: It'd be much easier to mix enum variants
// vec![&CompType::variants, &Shell::variants()]

/// Print aliases to be used for completions
#[derive(Debug, StructOpt)]
pub struct Completions {
    /// Variable type to list for help with completions
    #[structopt(
        possible_values = &CompType::variants(),
        case_insensitive = true,
        value_name = "completion_type",
    )]
    comp_type: CompType,
    /// Shell to be used for completions (can only be used with shell)
    #[structopt(
        possible_values = &ShellType::variants(),
        required_if("comp_type", "shell"),
    )]
    shell: Option<ShellType>
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    enum CompType {
        alias,
        env,
        shell
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    enum ShellType {
        zsh,
        bash
    }
}

impl RadSubCmdRunnable for Completions {
    fn run(&self) -> Result<String> {
        fn replace(
            haystack: &mut String,
            needle: &str,
            replacement: &str
        ) -> Result<(), anyhow::Error> {
            if let Some(index) = haystack.find(needle) {
                haystack.replace_range(index..index + needle.len(), replacement);
                Ok(())
            } else {
                Err(Error::from_chain(
                "Failed to find text:\n{}\n…in completion script:\n{}",
                needle, haystack
                ))
            }
        }

        let aliases_dir =
            config::rad_aliases_dir().with_context(||
                "failed to list variables for alias completions")?;
        let aliases =
            Aliases::open(aliases_dir).with_context(||
                "failed to list variables for alias completions")?;

        let res = match self.comp_type {
            CompType::alias => aliases.list_alias_completions().unwrap_or_else(|| "None".into()),
            CompType::env => aliases.list_env_completions().unwrap_or_else(|| "None".into()),
            CompType::shell => {
                let mut app = Rad::clap();

                // FIX: Send errors up to StructOpt
                let shell = Shell::from_str(
                    &self.shell
                    .as_ref()
                    .unwrap_or_else(|| &ShellType::zsh)
                    .to_string())
                    .unwrap();

                let buffer = Vec::new();
                let mut cursor = Cursor::new(buffer);
                app.gen_completions_to(env!("CARGO_PKG_NAME"),
                    shell,
                    &mut cursor);
                let buffer = cursor.into_inner();
                let mut script = String::from_utf8(buffer)
                    .expect("Clap completion not UTF-8");

                match shell {
                    Shell::Zsh =>
                        for (needle, replacement) in comp_helper::ZSH_COMPLETION_REP {
                            replace(&mut script, needle, replacement)?;
                    },
                    _ => println!(),
                }

                format!("{}", script.trim())
            }
        };

        Ok(res)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     #[serial]
//     fn no_vars() {
//         let subcmd = fixture::create_subcmd(ListEnv {});
//         let res = subcmd.run();
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), "");
//     }
//
//     #[test]
//     #[serial]
//     fn var() {
//         let mut subcmd = fixture::create_subcmd(ListEnv {});
//         subcmd.use_config(toml::toml![
//             [aliases]
//             test = "test"
//             [environment]
//             test = "TEST"
//         ]);
//         let res = subcmd.run();
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), "test TEST\n");
//     }
//
//     #[test]
//     #[serial]
//     fn vars() {
//         let mut subcmd = fixture::create_subcmd(ListEnv {});
//         subcmd.use_config(toml::toml![
//             [aliases]
//             test = "test"
//             test2 = "test2"
//             [environment]
//             test = "TEST"
//             test2 = "TEST2"
//         ]);
//         let res = subcmd.run();
//         assert!(res.is_ok());
//         assert_eq!(res.unwrap(), "test TEST\ntest2 TEST2\n");
//     }
// }
