use crate::common;
use anyhow::Result;

#[test]
fn not_existing_alias() -> Result<()> {
    let mut rad = common::create_rad("remove");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    let expected = String::from(
        r#"Error: fail to remove alias 'test'

Caused by:
    alias 'test' not exists
"#,
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn existing_alias() -> Result<()> {
    let mut rad = common::create_rad("remove");
    rad.use_config(toml::toml!(test = "test"));
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = String::from("alias 'test' removed\n");
    assert_eq!(actual, expected);
    Ok(())
}
