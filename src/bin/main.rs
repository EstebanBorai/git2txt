use std::env::current_dir;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;

use git2txt::{git::Git, scanner::Scanner};
use tempdir::TempDir;
use tokio::fs::{copy, create_dir, File, OpenOptions};
use tokio::sync::Mutex;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

const AGGREGATE_FILENAME: &str = "aggregate.txt";

#[derive(Debug, Parser)]
#[command(
    name = "git2txt",
    about = "Download Git Repos into TXT Files",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai/git2txt)",
    next_line_help = true
)]
pub struct Cli {
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter_layer)
        .init();

    let args = Cli::parse();
    let outdir = TempDir::new("git2txt-temp")?;
    let git = Git::new(args.url, outdir.path());

    git.download().await?;

    let scanner_output = outdir.path().join("output");
    create_dir(&scanner_output).await?;

    let scanner_aggregate_file = scanner_output.join(AGGREGATE_FILENAME);

    File::create(&scanner_aggregate_file).await?;
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&scanner_aggregate_file)
        .await?;
    let file = Arc::new(Mutex::new(file));

    let scanner = Scanner::new(outdir.path(), Arc::clone(&file));
    scanner.scan().await?;

    copy(&scanner_aggregate_file, current_dir()?.join("output.txt")).await?;

    Ok(())
}
