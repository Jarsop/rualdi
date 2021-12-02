use crate::common;
use anyhow::Result;

#[test]
fn not_existing_alias() -> Result<()> {
    let mut rad = common::create_rad("add");
    let output = rad.cmd.arg("test").output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "[alias] Added: test\n");
    Ok(())
}

#[test]
fn not_existing_path() -> Result<()> {
    let current_dir = std::env::current_dir().unwrap();
    let mut rad = common::create_rad("add");
    let output = rad.cmd.arg("test").arg("not-existing-path").output()?;
    let actual = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        actual,
        format!(
            r#"Error: [alias] Failed to add: test

Caused by:
    could not resolve path: {}/not-existing-path
"#,
            current_dir.to_str().unwrap()
        )
    );
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
    assert_eq!(
        actual,
        r#"Error: [alias] Failed to add: test

Caused by:
    alias 'test' already exists
"#,
    );
    Ok(())
}
