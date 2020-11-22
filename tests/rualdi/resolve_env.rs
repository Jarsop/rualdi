use crate::common;
use anyhow::Result;

#[test]
fn existing_var() -> Result<()> {
    let mut rad = common::create_rad("resolve-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
        [environment]
        test = "TEST"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "TEST\n");
    Ok(())
}

#[test]
fn not_existing_var() -> Result<()> {
    let mut rad = common::create_rad("resolve-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        actual,
        r#"Error: fail to resolve environment variable for alias 'test'

Caused by:
    no variable for alias 'test'
"#,
    );
    Ok(())
}
