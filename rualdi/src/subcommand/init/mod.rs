mod bash;
mod zsh;

use anyhow::{Context, Result};
use structopt::clap::arg_enum;
use structopt::StructOpt;

use std::io;

/// Generates shell configuration
#[derive(Debug, StructOpt)]
#[structopt()]
pub struct Init {
    #[structopt(possible_values = &Shell::variants(), case_insensitive = true)]
    shell: Shell,

    /// Renames the 'rad' command and corresponding aliases
    #[structopt(long, alias = "rad-cmd", default_value = "rad")]
    cmd: String,
}

impl Init {
    pub fn run(&self) -> Result<String> {
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
