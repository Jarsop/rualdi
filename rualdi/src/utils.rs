use anyhow::{bail, Context, Result};
use std::env;
use std::path::{Component, Path, PathBuf};

pub fn resolve_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path = path.as_ref();
    let base_path;

    let mut components = path.components().peekable();
    let mut stack = Vec::new();

    // initialize root
    match components.peek() {
        Some(Component::RootDir) => {
            let root = components.next().unwrap();
            stack.push(root);
        }
        _ => {
            base_path = get_current_dir()?;
            stack.extend(base_path.components());
        }
    }

    for component in components {
        match component {
            Component::Normal(_) => stack.push(component),
            Component::CurDir => (),
            Component::ParentDir => {
                if stack.last() != Some(&Component::RootDir) {
                    stack.pop();
                }
            }
            Component::Prefix(_) | Component::RootDir => unreachable!(),
        }
    }

    let result = stack.iter().collect::<PathBuf>();
    if !result.is_dir() {
        bail!("could not resolve path: {}", result.display());
    }
    Ok(result)
}

pub fn get_current_dir() -> Result<PathBuf> {
    env::current_dir().context("could not get current path")
}

pub fn path_to_str<P: AsRef<Path>>(path: &P) -> Result<&str> {
    let path = path.as_ref();
    path.to_str()
        .with_context(|| format!("invalid utf-8 sequence in path: {}", path.display()))
}
