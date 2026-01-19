use std::process::Command;

fn main() -> anyhow::Result<()> {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("./examples/example-grid.json")
        .status()?;

    Ok(())
}
