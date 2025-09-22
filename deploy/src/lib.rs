use std::fs;
use std::process::{exit, Command};

use anyhow::{anyhow, Context};
use ssh2::Session;
use std::io::Write;
use std::net::Ipv4Addr;
use std::path::Path;
use tracing::error;

pub fn find_rio(team: usize) -> anyhow::Result<Ipv4Addr> {
    if !(team / 100 <= 255) {
        error!("Invalid team number");
        exit(1);
    }

    for address in [
        Ipv4Addr::new(10, (team / 100) as u8, (team % 100) as u8, 2),
        Ipv4Addr::new(172, 22, 11, 2),
    ] {
        return Ok(address);
    }

    Err(anyhow!("rio not found"))
}

pub fn send_file(path: &Path, remote: &Path, session: &Session) -> anyhow::Result<()> {
    let file = fs::read(path)?;
    let mut remote = remote.to_path_buf();
    remote.push(path.file_name().context("no file")?);
    let mut remote_file = session.scp_send(&remote, 0o744, file.len() as u64, None)?;
    remote_file.write_all(&file)?;
    remote_file.send_eof()?;
    remote_file.wait_eof()?;
    remote_file.close()?;
    remote_file.wait_close()?;

    Ok(())
}

pub fn send_string(data: String, remote: &Path, session: &Session) -> anyhow::Result<()> {
    let remote = remote.to_path_buf();
    let mut remote_file = session.scp_send(&remote, 0o744, data.len() as u64, None)?;

    remote_file.write_all((&data).as_ref())?;
    remote_file.send_eof()?;
    remote_file.wait_eof()?;
    remote_file.close()?;
    remote_file.wait_close()?;

    Ok(())
}

pub fn deploy_executable(path: String, team: String) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("scp {} lvuser@roborio-{}-frc.local:.", path, team))
        .output()
        .expect("failed to execute process");

    std::io::stdout().write_all(&output.stdout).unwrap();
}

pub fn deploy_lib(path: String, team: String) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("scp {} lvuser@roborio-{}-frc.local:.", path, team))
        .spawn()
        .expect("failed to execute process");
}

pub fn robot_command(executeable: String, team: String) {
    let name = Path::new(&executeable)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let command = format!("JAVA_HOME {}", name);

    Command::new("sh")
        .arg("-c")
        .arg(format!("ssh lvuser@roborio-{}-frc.local", team))
        .arg(format!("echo {} > robotCommand", command))
        .spawn()
        .expect("failed to execute process");
}
