use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn rad_aliases_dir() -> Result<PathBuf> {
    let aliases_dir = match env::var_os("_RAD_ALIASES_DIR") {
        Some(data_osstr) => PathBuf::from(data_osstr),
        None => match dirs::data_local_dir() {
            Some(mut aliases_dir) => {
                aliases_dir.push("rualdi");
                aliases_dir
            }
            None => bail!("could not find config directory, please set _RAD_ALIASES_DIR manually"),
        },
    };

    // This will fail when `aliases_dir` points to a file or a broken symlink, but
    // will no-op on a valid symlink (to a directory), or an actual directory.
    fs::create_dir_all(&aliases_dir).with_context(|| {
        format!(
            "could not create data directory: '{}'",
            aliases_dir.display()
        )
    })?;

    Ok(aliases_dir)
}

pub fn rad_no_echo() -> bool {
    match env::var_os("_RAD_NO_ECHO") {
        Some(var) => var == "1",
        None => false,
    }
}

pub fn rad_resolve_symlinks() -> bool {
    match env::var_os("_RAD_RESOLVE_SYMLINKS") {
        Some(var) => var == "1",
        None => false,
    }
}
