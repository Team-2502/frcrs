use std::{fs, net::{Ipv4Addr, TcpStream, SocketAddrV4}, time::Duration, path::{PathBuf, Path}, str::FromStr, io::Write};

use anyhow::{anyhow, Context, Ok};
use glob::glob;
use locate_cargo_manifest::locate_manifest;
use ping::{ping, Error};
use serde::Deserialize;
use ssh2::Session;

#[derive(Deserialize)]
struct Config {
    team: usize,
    package: String,
    #[serde(default)]
    profile: String,
    #[serde(default)]
    server_port: usize,
    #[serde(default)]
    server_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self { team: Default::default(), package: Default::default(), profile: "release".to_owned(), server_port: 8085, server_path: Default::default() }
    }
}

pub fn deploy() -> anyhow::Result<()> {
    let mut config = locate_manifest()?;
    config.pop();
    config.push("frcrs.toml");
    let config: Config = toml::from_str(&fs::read_to_string(config)?)?;

    bootstrap(find_rio(config.team)?, &config)?;



    Ok(())
}

/// upload without the client library, using ssh and scp
fn bootstrap(rio: Ipv4Addr, config: &Config) -> anyhow::Result<()> {
    let profile = &config.profile;
    let package = &config.package;

    let mut ssh = Session::new()?;

    let rio = TcpStream::connect(SocketAddrV4::new(rio,22))?;
    ssh.set_tcp_stream(rio);
    ssh.handshake()?;

    ssh.userauth_password("admin", "")?;
    assert!(ssh.authenticated());

    let remote_lib_dir = PathBuf::from_str("/usr/local/frc/third-party/lib")?;

    for lib in libs()? {
        println!("uploading: {}", lib.file_name().unwrap().to_str().unwrap());
        send_file(&lib, &remote_lib_dir, &ssh)?;
    }

    let mut user_program = locate_manifest()?;
    user_program.pop();
    user_program.push(format!("target/arm-unknown-linux-gnueabi/{profile}/{package}"));

    send_file(&user_program, &PathBuf::from("/home/lvuser"), &ssh)?;

    Ok(())
}

fn send_file(path: &Path, remote: &Path, session: &Session) -> anyhow::Result<()> {
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

pub fn find_rio(team: usize) -> anyhow::Result<Ipv4Addr> {
    let rio_timeout = Duration::from_millis(100);
    assert!(team/100 <= 255);
    for address in [
        Ipv4Addr::new(10, (team/100) as u8, (team%100) as u8, 2),
        Ipv4Addr::new(172, 22, 11, 2), // usb
    ] {
        if ping(std::net::IpAddr::V4(address), Some(rio_timeout), None, None, None, None).is_ok() {
            return Ok(address)
        }
    }

    Err(anyhow!("rio not found"))
}

pub fn libs() -> anyhow::Result<Vec<PathBuf>> {
    let profile = "release";
    let mut project = locate_manifest()?;
    project.pop(); // Cargo.toml
    project.push(format!("target/arm-unknown-linux-gnueabi/{profile}/build"));

    let mut names = Vec::new();
    let mut paths = Vec::new();

    for path in glob(&format!("{}/*-sys-*/out/libs/*.so", project.to_str().context("bad character in path")?))? {
        let path = path?;
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        if names.contains(&name) {
            continue;
        }
        names.push(name);
        paths.push(path);
    }

    Ok(paths)
}
