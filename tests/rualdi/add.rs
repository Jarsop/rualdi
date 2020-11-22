use crate::common;
use anyhow::Result;

#[test]
fn not_existing_alias() -> Result<()> {
    let mut rad = common::create_rad("add");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = String::from("alias 'test' added\n");
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn not_existing_path() -> Result<()> {
    let current_dir = std::env::current_dir().unwrap();
    let mut rad = common::create_rad("add");
    let output = rad.cmd.arg("test").arg("not-existing-path").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    let expected = format!(
        r#"Error: fail to add alias 'test'

Caused by:
    could not resolve path: {}/not-existing-path
"#,
        current_dir.to_str().unwrap()
    );
    assert_eq!(actual, expected);
    Ok(())
}

#[test]
fn existing_alias() -> Result<()> {
    let mut rad = common::create_rad("add");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    let expected = String::from(
        r#"Error: fail to add alias 'test'

Caused by:
    alias 'test' already exists
"#,
    );
    assert_eq!(actual, expected);
    Ok(())
}
