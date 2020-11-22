use crate::common;
use anyhow::Result;

#[test]
fn not_existing_alias() -> Result<()> {
    let mut rad = common::create_rad("add-env");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        actual,
        "Error: cannot add environment variable 'TEST', no such alias 'test'\n"
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
    assert_eq!(
        actual,
        "environment variable 'TEST' for alias 'test' added\n"
    );
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
    assert_eq!(
        actual,
        "environment variable 'TOTO' for alias 'test' added\n"
    );
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
    assert_eq!(
        actual,
        "environment variable 'TOTO' for alias 'test' added\n"
    );
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
    let expected = String::from(
        r#"Error: fail to add environment variable 'TEST' for alias 'test'

Caused by:
    alias 'test' has already a environment variable assiociated
"#,
    );
    assert_eq!(actual, expected);
    Ok(())
}
