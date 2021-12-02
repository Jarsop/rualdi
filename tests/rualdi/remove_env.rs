use crate::common;
use anyhow::Result;

#[test]
fn not_existing_var() -> Result<()> {
    let mut rad = common::create_rad("remove-env");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    let expected = String::from(
        r#"Error: [env] Failed to remove for [alias] test

Caused by:
    no such environment variable for alias 'test'
"#,
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn existing_var() -> Result<()> {
    let mut rad = common::create_rad("remove-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
        [environment]
        test = "TEST"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "[env] Removed for [alias] test");
    Ok(())
}
