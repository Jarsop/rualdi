use crate::common;
use anyhow::Result;

#[test]
fn empty() -> Result<()> {
    let mut rad = common::create_rad("list-env");
    let output = rad.cmd.output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "");
    Ok(())
}

#[test]
fn filled() -> Result<()> {
    let mut rad = common::create_rad("list-env");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
        [environment]
        test = "TEST"
    ]);
    let output = rad.cmd.output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "test TEST\n");
    Ok(())
}
