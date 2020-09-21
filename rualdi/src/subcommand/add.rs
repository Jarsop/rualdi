use crate::config;
use crate::utils;
use anyhow::{Context, Result};
use rualdlib::Aliases;
use std::path::PathBuf;
use structopt::StructOpt;

/// Add new path alias
#[derive(Debug, StructOpt)]
pub struct Add {
    /// Alias to path
    pub alias: String,
    /// Path to aliasing, if not provided current directory is used
    pub path: Option<PathBuf>,
}

impl Add {
    pub fn run(&self) -> Result<String> {
        let aliases_dir = config::rad_aliases_dir()
            .with_context(|| format!("fail to add alias '{}'", self.alias))?;
        let mut aliases = Aliases::open(aliases_dir)
            .with_context(|| format!("fail to add alias '{}'", self.alias))?;

        let path = self.path.to_owned().unwrap_or(utils::get_current_dir()?);
        let path = utils::resolve_path(path)
            .with_context(|| format!("fail to add alias '{}'", self.alias))?;

        aliases
            .add(self.alias.to_owned(), utils::path_to_str(&path)?.into())
            .with_context(|| format!("fail to add alias '{}'", self.alias))?;

        Ok(format!("alias '{}' added\n", self.alias))
    }
}
