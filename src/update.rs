use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};

const REPOSITORY: &str = "https://github.com/Dexter9532/devnav.git";

pub fn run() -> Result<()> {
    println!("Updating DevNav");

    let status = update_command()
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("failed to run Cargo; make sure Rust and Cargo are installed")?;

    if !status.success() {
        bail!("DevNav update failed");
    }

    println!("DevNav updated");
    Ok(())
}

fn update_command() -> Command {
    let mut command = Command::new("cargo");
    command.args(["install", "--git", REPOSITORY, "--force"]);
    command
}

#[cfg(test)]
mod tests {
    use super::{REPOSITORY, update_command};

    #[test]
    fn update_installs_latest_version_from_repository() {
        let command = update_command();
        let arguments: Vec<_> = command.get_args().collect();

        assert_eq!(command.get_program(), "cargo");
        assert_eq!(arguments, ["install", "--git", REPOSITORY, "--force"]);
    }
}
