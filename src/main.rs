mod config;
mod comp_helper;
mod error;
#[cfg(test)]
mod fixture;
#[macro_use]
mod macros;
mod subcommand;
mod utils;

use crate::error::SilentExit;
use anyhow::Result;
use std::process;
use structopt::StructOpt;
use clap::{AppSettings};
use subcommand::RadSubCmdRunnable;

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    global_settings = &[
        AppSettings::ColoredHelp,
        AppSettings::ColorAlways
    ]
)]
struct Rad {
    #[structopt(flatten)]
    radsubcmd: RadSubCmd,
}

#[derive(Debug, StructOpt)]
pub enum RadSubCmd {
    #[structopt(alias = "a")]
    Add(subcommand::Add),
    #[structopt(alias = "ax")]
    AddEnv(subcommand::AddEnv),
    #[structopt(alias = "i")]
    Init(subcommand::Init),
    #[structopt(alias = "l")]
    List(subcommand::List),
    #[structopt(alias = "la")]
    ListAlias(subcommand::ListAlias),
    #[structopt(alias = "lx")]
    ListEnv(subcommand::ListEnv),
    #[structopt(alias = "r")]
    Remove(subcommand::Remove),
    #[structopt(alias = "rx")]
    RemoveEnv(subcommand::RemoveEnv),
    #[structopt(alias = "res")]
    Resolve(subcommand::Resolve),
    #[structopt(alias = "resx")]
    ResolveEnv(subcommand::ResolveEnv),

    #[structopt(alias = "comp")]
    Completions(subcommand::Completions),
}

fn rad_main() -> Result<String> {
    let opt = Rad::from_args();

    let res = match opt.radsubcmd {
        RadSubCmd::Add(add) => add.run(),
        RadSubCmd::AddEnv(add_env) => add_env.run(),
        RadSubCmd::Completions(completions) => completions.run(),
        RadSubCmd::Init(init) => init.run(),
        RadSubCmd::List(list) => list.run(),
        RadSubCmd::ListAlias(list_alias) => list_alias.run(),
        RadSubCmd::ListEnv(list_env) => list_env.run(),
        RadSubCmd::Remove(remove) => remove.run(),
        RadSubCmd::RemoveEnv(remove_env) => remove_env.run(),
        RadSubCmd::Resolve(resolve) => resolve.run(),
        RadSubCmd::ResolveEnv(resolve_env) => resolve_env.run(),
    };

    if res.is_ok() {
        print!("{}", res.as_ref().unwrap());
        // print!("{:?}", clap::App::);
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
