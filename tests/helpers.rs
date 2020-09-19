use anyhow::Result;
use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

pub struct TmpConfig {
    pub tmp_dir: TempDir,
    pub tmp_file: File,
}

impl TmpConfig {
    pub fn create_dir() -> Result<Self> {
        let tmp_dir = TempDir::new("test_rualdi")?;
        let file_path = tmp_dir.path().join(".default");
        let tmp_file = File::create(file_path)?;
        Ok(TmpConfig { tmp_dir, tmp_file })
    }

    pub fn with_empty(mut self) -> Result<Self> {
        let file_path = self.tmp_dir.path().join("rualdi.toml");
        self.tmp_file = File::create(file_path)?;
        Ok(self)
    }

    pub fn with_base(mut self) -> Result<Self> {
        let file_path = self.tmp_dir.path().join("rualdi.toml");
        self.tmp_file = File::create(file_path)?;
        writeln!(
            self.tmp_file,
            "# Rualdi aliases configuration file\n[aliases]\n"
        )?;
        Ok(self)
    }

    pub fn with_content(mut self, content: &str) -> Result<Self> {
        writeln!(self.tmp_file, "{}", content)?;
        Ok(self)
    }
}
