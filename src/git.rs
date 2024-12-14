use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;
use tracing::debug;

pub struct Git {
    pub url: String,
    pub outdir: PathBuf,
}

impl Git {
    pub fn new<P: AsRef<Path>>(url: String, outdir: P) -> Self {
        Git {
            url,
            outdir: outdir.as_ref().to_path_buf(),
        }
    }

    pub async fn download(&self) -> Result<()> {
        println!("Cloning {} into {}", self.url, self.outdir.display());
        self.pull().await?;
        Ok(())
    }

    async fn pull(&self) -> Result<()> {
        debug!(url = self.url, "Clonning...");

        Command::new("git")
            .arg("clone")
            .arg("--depth=1")
            .arg(self.url.as_str())
            .arg(self.outdir.as_os_str())
            .output()?;

        Ok(())
    }
}
