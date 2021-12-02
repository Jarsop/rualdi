use crate::common;
use anyhow::Result;

#[test]
fn empty() -> Result<()> {
    let mut rad = common::create_rad("list");
    let output = rad.cmd.output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "No aliases found\n");
    Ok(())
}

#[test]
fn filled() -> Result<()> {
    let mut rad = common::create_rad("list");
    rad.use_config(toml::toml![
        [aliases]
        test = "test"
    ]);
    let output = rad.cmd.output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    assert_eq!(actual, "=\nALIASES\n=\ntest         => test\n");
    Ok(())
}
