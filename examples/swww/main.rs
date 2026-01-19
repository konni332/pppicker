use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() -> anyhow::Result<()> {
    let old_cwd = std::env::current_dir()?;
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples/swww");
    std::env::set_current_dir(dir)?;

    let mut script = Command::new("./fetch_wallpapers.sh")
        .stdout(Stdio::piped())
        .spawn()?;

    let script_stdout = script.stdout.take().expect("stdout missing");

    let mut cargo = Command::new("cargo")
        .args(["run", "--"])
        .stdin(Stdio::from(script_stdout))
        .stdout(Stdio::inherit())
        .spawn()?;

    let status = cargo.wait()?;
    script.wait()?;
    assert!(status.success());

    std::env::set_current_dir(old_cwd)?;
    Ok(())
}
