use crate::common;
use anyhow::Result;

#[test]
fn empty() -> Result<()> {
    let mut rad = common::create_rad("list");
    let output = rad.cmd.output()?;
    let actual = String::from_utf8(output.stdout).unwrap();
    let expected = String::from("No aliases found\n");
    assert_eq!(actual, expected);
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
    let expected = String::from("Aliases:\n\n\t'test' => 'test'\n");
    assert_eq!(actual, expected);
    Ok(())
}
