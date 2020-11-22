use crate::subcommand::RadSubCmdRunnable;
use anyhow::Result;
use std::fs;
use std::io::Write;
use tempfile::{Builder, TempDir};

pub struct TmpConfig {
    pub tmp_dir: TempDir,
    pub tmp_file: fs::File,
}

impl TmpConfig {
    pub fn create_dir() -> Self {
        let tmp_dir = Builder::new().prefix("test_rualdi").tempdir().unwrap();
        let file_path = tmp_dir.path().join("rualdi.toml");
        let tmp_file = fs::File::create(file_path).unwrap();
        TmpConfig { tmp_dir, tmp_file }
    }

    pub fn with_base(&mut self) {
        self.tmp_file
            .write_all(b"# Rualdi aliases configuration file\n# DO NOT EDIT\n")
            .unwrap();
        self.tmp_file.flush().unwrap();
    }

    pub fn with_content(&mut self, toml: toml::value::Value) {
        self.tmp_file
            .write_all(toml.to_string().as_bytes())
            .unwrap();
        self.tmp_file.flush().unwrap();
    }
}

pub struct TestSubCmd<T> {
    subcmd: T,
    pub tmp: TmpConfig,
}

impl<T> TestSubCmd<T>
where
    T: RadSubCmdRunnable,
{
    pub fn use_config(&mut self, toml: toml::value::Value) {
        self.tmp.with_content(toml);
    }

    pub fn run(&self) -> Result<String> {
        self.subcmd.run()
    }
}

pub fn create_subcmd<T>(subcmd: T) -> TestSubCmd<T>
where
    T: RadSubCmdRunnable,
{
    let mut tmp = TmpConfig::create_dir();
    tmp.with_base();

    std::env::set_var("_RAD_ALIASES_DIR", tmp.tmp_dir.path().as_os_str());

    TestSubCmd { subcmd, tmp }
}
