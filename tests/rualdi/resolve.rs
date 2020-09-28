use crate::common;
use anyhow::Result;

#[test]
fn existing_alias() -> Result<()> {
    let current_dir = std::env::current_dir().unwrap();
    let mut rad = common::create_rad("resolve");
    rad.use_config(toml::toml!(test = "not-existing-path"));
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    let expected = format!(
        "Error: could not resolve path: {}/not-existing-path\n",
        current_dir.to_str().unwrap()
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn existing_path_without_alias() -> Result<()> {
    let mut rad = common::create_rad("resolve");
    let current_dir = std::env::current_dir().unwrap();
    let output = rad.cmd.arg(&current_dir).output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("{}\n", current_dir.to_str().unwrap());
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn not_existing_path_without_alias() -> Result<()> {
    let mut rad = common::create_rad("resolve");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    let expected = format!(
        "Error: could not resolve path: {}/test\n",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn tild_alias() -> Result<()> {
    let mut rad = common::create_rad("resolve");
    rad.use_config(toml::toml!(home = "~"));
    let output = rad.cmd.arg("home").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = format!("{}\n", std::env::var("HOME").unwrap());
    assert_eq!(actual, expected);
    Ok(())
}
