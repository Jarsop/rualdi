use crate::config;
use anyhow::{Context, Result};
use rualdlib::Aliases;
use structopt::StructOpt;

/// Remove alias
#[derive(Debug, StructOpt)]
pub struct Remove {
    /// Alias to remove
    pub alias: Vec<String>,
}

impl Remove {
    pub fn run(&self) -> Result<String> {
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
