use crate::config;
use anyhow::{Context, Result};
use structopt::StructOpt;

/// Add new path alias
#[derive(Debug, StructOpt)]
pub struct List {}

impl List {
    pub fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir().with_context(|| "fail to list aliases")?;
        let aliases = rualdi::Aliases::open(aliases_dir).with_context(|| "fail to list aliases")?;

        let res = aliases
            .list()
            .unwrap_or_else(|| "No aliases found\n".into());

        Ok(res)
    }
}
