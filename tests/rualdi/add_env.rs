use crate::common;
use anyhow::Result;

#[test]
fn not_existing_alias() -> Result<()> {
    let mut rad = common::create_rad("add-env");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        actual,
        "Error: [alias] test doesn't exist. Cannot add [env] TEST\n"
    );
    Ok(())
}

#[test]
fn not_existing_var() -> Result<()> {
    let mut rad = common::create_rad("add-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "[env] TEST added for [alias] test");
    Ok(())
}

#[test]
fn not_existing_named_var_cap() -> Result<()> {
    let mut rad = common::create_rad("add-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.arg("test").arg("TOTO").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "[env] TOTO added for [alias] test");
    Ok(())
}

#[test]
fn not_existing_named_var_no_cap() -> Result<()> {
    let mut rad = common::create_rad("add-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.arg("test").arg("toto").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "[env] TOTO added for [alias] test");
    Ok(())
}

#[test]
fn existing_var() -> Result<()> {
    let mut rad = common::create_rad("add-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
        [environment]
        test = "TEST"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        actual,
        r#"Error: Failed to add: [env] TEST for [alias] test

Caused by:
    alias 'test' has already a environment variable assiociated
"#
    );
    Ok(())
}
