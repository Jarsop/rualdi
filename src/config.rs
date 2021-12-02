use anyhow::{bail, Context, Result};
#[cfg(test)]
use serial_test::serial;
use std::{env, fs, path::PathBuf};

pub fn rad_aliases_dir() -> Result<PathBuf> {
    let aliases_dir = match env::var_os("_RAD_ALIASES_DIR") {
        Some(data_osstr) => PathBuf::from(data_osstr),
        None => {
            #[cfg(target_os = "macos")]
            let config_dir_og = std::env::var_os("XDG_DATA_HOME")
                .map(PathBuf::from)
                .filter(|p| p.is_absolute())
                .or_else(|| dirs_next::home_dir().map(|d| d.join(".local").join("share")));

            #[cfg(not(target_os = "macos"))]
            let config_dir_og = dirs_next::data_local_dir();

            if let Some(conf_dir) = config_dir_og.map(|d| d.join(env!("CARGO_PKG_NAME"))) {
                conf_dir
            } else {
                bail!("could not find config directory, please set _RAD_ALIASES_DIR manually")
            }
        }
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

// pub fn fzf_opts() -> Option<OsString> {
//     env::var_os("_RAD_FZF_OPTS")
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[serial]
    fn echo() {
        assert!(!rad_no_echo());
    }

    #[test]
    #[serial]
    fn no_echo() {
        std::env::set_var("_RAD_NO_ECHO", "1");
        assert!(rad_no_echo());
        std::env::set_var("_RAD_NO_ECHO", "");
    }

    #[test]
    #[serial]
    fn resolve_symlinks() {
        std::env::set_var("_RAD_RESOLVE_SYMLINKS", "1");
        assert!(rad_resolve_symlinks());
        std::env::set_var("_RAD_RESOLVE_SYMLINKS", "0");
    }

    #[test]
    #[serial]
    fn no_resolve_symlinks() {
        assert!(!rad_resolve_symlinks());
    }
}
