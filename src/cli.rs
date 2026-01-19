use std::{
    env, fs,
    io::{self, IsTerminal, Read},
    path::PathBuf,
};

use anyhow::Context;

pub fn read_input() -> anyhow::Result<String> {
    let mut args = env::args().skip(1);

    if let Some(path) = args.next() {
        read_from_file(PathBuf::from(path))
    } else {
        read_from_stdin()
    }
}

fn read_from_file(path: PathBuf) -> anyhow::Result<String> {
    let input = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read file {}", path.display()))?;
    tracing::info!("loaded content from: {}", path.display());
    Ok(input)
}

fn read_from_stdin() -> anyhow::Result<String> {
    if io::stdin().is_terminal() {
        anyhow::bail!("No input provided on stdin (stdin is a terminal)")
    }
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    if buffer.trim().is_empty() {
        anyhow::bail!("No input provided on stdin")
    }
    tracing::info!("loaded content from: stdin");
    Ok(buffer)
}
