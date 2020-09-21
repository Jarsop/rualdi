mod config;
mod error;
mod subcommand;
mod utils;

use crate::error::SilentExit;
use anyhow::Result;
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rualdi", about = "Rust Aliasing Directory.")]
struct Rad {
    #[structopt(flatten)]
    radsubcmd: RadSubCmd,
}

#[derive(Debug, StructOpt)]
pub enum RadSubCmd {
    Add(subcommand::Add),
    Init(subcommand::Init),
    List(subcommand::List),
    Remove(subcommand::Remove),
    Resolve(subcommand::Resolve),
}

fn rad_main() -> Result<String> {
    let opt = Rad::from_args();

    let res = match opt.radsubcmd {
        RadSubCmd::Add(add) => add.run(),
        RadSubCmd::Init(init) => init.run(),
        RadSubCmd::List(list) => list.run(),
        RadSubCmd::Remove(remove) => remove.run(),
        RadSubCmd::Resolve(resolve) => resolve.run(),
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
