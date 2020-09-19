mod helpers;
use anyhow::Result;

#[test]
fn open_config_not_existing() -> Result<()> {
    let aliases_file = helpers::TmpConfig::create_dir()?;
    let aliases = rualdi::Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
    let expected_aliases = rualdi::MockAliases::open_no_aliases();
    assert!(aliases.is_ok());
    assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
    Ok(())
}

#[test]
fn open_config_empty_aliases() -> Result<()> {
    let aliases_file = helpers::TmpConfig::create_dir()?.with_base()?;

    let aliases = rualdi::Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
    let expected_aliases = rualdi::MockAliases::open_no_aliases();
    assert!(aliases.is_ok());
    assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
    Ok(())
}

#[test]
fn open_config_empty_file() -> Result<()> {
    let aliases_file = helpers::TmpConfig::create_dir()?.with_empty()?;
    let aliases = rualdi::Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
    let expected_aliases = rualdi::MockAliases::open_no_aliases();
    assert!(aliases.is_ok());
    assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
    Ok(())
}

#[test]
fn open_config_filled() -> Result<()> {
    let aliases_file = helpers::TmpConfig::create_dir()?
        .with_base()?
        .with_content(r#"test = "/test/haha""#)?
        .with_content(r#"Home = "~""#)?;

    let aliases = rualdi::Aliases::open(aliases_file.tmp_dir.path().to_path_buf());
    let expected_aliases = rualdi::MockAliases::open();
    assert!(aliases.is_ok());
    assert_eq!(aliases.unwrap().aliases, expected_aliases.aliases);
    Ok(())
}
