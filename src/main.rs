mod config;
mod error;
#[cfg(test)]
mod fixture;
mod subcommand;
mod utils;

use crate::error::SilentExit;
use anyhow::Result;
use std::process;
use structopt::StructOpt;
use subcommand::RadSubCmdRunnable;

#[derive(Debug, StructOpt)]
#[structopt(name = "rualdi", about = "Rust Aliasing Directory.")]
struct Rad {
    #[structopt(flatten)]
    radsubcmd: RadSubCmd,
}

#[derive(Debug, StructOpt)]
pub enum RadSubCmd {
    Add(subcommand::Add),
    AddEnv(subcommand::AddEnv),
    Init(subcommand::Init),
    List(subcommand::List),
    ListEnv(subcommand::ListEnv),
    Remove(subcommand::Remove),
    RemoveEnv(subcommand::RemoveEnv),
    Resolve(subcommand::Resolve),
    ResolveEnv(subcommand::ResolveEnv),
}

fn rad_main() -> Result<String> {
    let opt = Rad::from_args();

    let res = match opt.radsubcmd {
        RadSubCmd::Add(add) => add.run(),
        RadSubCmd::AddEnv(add_env) => add_env.run(),
        RadSubCmd::Init(init) => init.run(),
        RadSubCmd::List(list) => list.run(),
        RadSubCmd::ListEnv(list_env) => list_env.run(),
        RadSubCmd::Remove(remove) => remove.run(),
        RadSubCmd::RemoveEnv(remove_env) => remove_env.run(),
        RadSubCmd::Resolve(resolve) => resolve.run(),
        RadSubCmd::ResolveEnv(resolve_env) => resolve_env.run(),
    };

    if res.is_ok() {
        print!("{}", res.as_ref().unwrap());
    }

    res.map_err(|e| match e.downcast::<SilentExit>() {
        Ok(SilentExit { code }) => process::exit(code),
        Err(e) => e,
    })
}

fn main() -> Result<()> {
    match rad_main() {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
