use crate::common;
use anyhow::Result;

#[test]
fn not_existing_alias() -> Result<()> {
    let mut rad = common::create_rad("remove");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        actual,
        r#"Error: [alias] Failed to remove: test

Caused by:
    alias 'test' not exists
"#
    );
    Ok(())
}

#[test]
fn existing_alias() -> Result<()> {
    let mut rad = common::create_rad("remove");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "[alias] Removed: test\n");
    Ok(())
}
