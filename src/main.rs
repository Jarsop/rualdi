mod comp_helper;
mod config;
mod error;
#[cfg(test)]
mod fixture;
// mod fzf;
#[macro_use]
mod macros;
mod subcommand;
mod utils;

use crate::error::SilentExit;
use anyhow::Result;
use clap::AppSettings;
use std::process;
use structopt::StructOpt;
use subcommand::RadSubCmdRunnable;

const ENV_VAR_HELP: &str = "\x1b[0;31mENVIRONMENT VARIABLES\x1b[0m
    \x1b[0;35m_RAD_ALIASES_DIR\x1b[0m        Directory where configuration is stored
    \x1b[0;35m_RAD_NO_ECHO\x1b[0m            Whether or not to print directory name before cd'ing to it
    \x1b[0;35m_RAD_RESOLVE_SYMLINKS\x1b[0m   Whether symlinks should be resolved";

#[derive(Debug, StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    after_help = ENV_VAR_HELP,
    global_settings = &[
        AppSettings::ColoredHelp,
        AppSettings::ColorAlways,
        AppSettings::DisableHelpSubcommand,
        AppSettings::VersionlessSubcommands,
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
