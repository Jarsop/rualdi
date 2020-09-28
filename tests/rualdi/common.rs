use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::{Builder, TempDir};

const EXE_PATH: &str = "./target/debug/rualdi";

pub struct TmpConfig {
    pub tmp_dir: TempDir,
    pub tmp_file: fs::File,
}

impl TmpConfig {
    pub fn create_dir() -> Self {
        let tmp_dir = Builder::new().prefix("test_rualdi").tempdir().unwrap();
        let file_path = tmp_dir.path().join(".default");
        let tmp_file = fs::File::create(file_path).unwrap();
        TmpConfig { tmp_dir, tmp_file }
    }

    pub fn with_base(&mut self) {
        let file_path = self.tmp_dir.path().join("rualdi.toml");
        self.tmp_file = fs::File::create(file_path).unwrap();
        writeln!(
            self.tmp_file,
            "# Rualdi aliases configuration file\n[aliases]\n"
        )
        .unwrap();
    }

    pub fn with_content(&mut self, toml: toml::value::Value) {
        writeln!(self.tmp_file, "{}", toml.to_string()).unwrap();
    }
}

pub struct TestCommand {
    pub cmd: Command,
    pub tmp: TmpConfig,
}

impl TestCommand {
    pub fn use_config(&mut self, toml: toml::value::Value) {
        self.tmp.with_content(toml);
    }
}

pub fn create_rad(subcmd: &str) -> TestCommand {
    let binary = fs::canonicalize(EXE_PATH).unwrap();
    let mut cmd = Command::new(binary);
    let mut tmp = TmpConfig::create_dir();
    tmp.with_base();

    cmd.arg(subcmd)
        .env_clear()
        .env("_RAD_ALIASES_DIR", tmp.tmp_dir.path().as_os_str());

    TestCommand { cmd, tmp }
}
