mod bash;
mod zsh;

#[cfg(test)]
use crate::fixture;
use crate::subcommand::RadSubCmdRunnable;
use anyhow::{Context, Result};
#[cfg(test)]
use gag::Gag;
#[cfg(test)]
use serial_test::serial;
use structopt::clap::arg_enum;
use structopt::StructOpt;

use std::io;

/// Generates shell configuration
#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Init {
    #[structopt(
        possible_values = &Shell::variants(),
        case_insensitive = true)
    ]
    shell: Shell,

    /// Renames the 'rad' command and corresponding aliases
    #[structopt(long, alias = "rad-cmd", default_value = "rad")]
    cmd: String,
}

impl RadSubCmdRunnable for Init {
    fn run(&self) -> Result<String> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();

        match self.shell {
            Shell::bash => bash::run(&mut handle, self),
            Shell::zsh => zsh::run(&mut handle, self),
        }
        .context("could not initialize rualdi")?;
        Ok("\n".to_string())
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    enum Shell {
        bash,
        zsh,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn zsh() {
        let subcmd = fixture::create_subcmd(Init {
            shell: Shell::zsh,
            cmd: String::from("rad"),
        });
        let _print_gag = Gag::stdout().unwrap();
        let res = subcmd.run();
        assert!(res.is_ok());
    }

    #[test]
    #[serial]
    fn bash() {
        let subcmd = fixture::create_subcmd(Init {
            shell: Shell::bash,
            cmd: String::from("rad"),
        });
        let _print_gag = Gag::stdout().unwrap();
        let res = subcmd.run();
        assert!(res.is_ok());
    }
}
