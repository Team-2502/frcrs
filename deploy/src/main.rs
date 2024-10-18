use std::{env, fs};
use std::io::Write;
use std::net::{IpAddr, SocketAddr, SocketAddrV4, TcpStream};
use std::path::Path;
use std::process::{Command, exit};
use std::time::Duration;
use tracing::{error, info};
use ssh2::Session;
use deploy::{find_rio, send_file, send_string};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Args {
    deploy: Option<DeployArgs>
}

#[derive(Deserialize, Debug)]
struct DeployArgs {
    team_number: usize,

    executable: String,

    lib: Option<String>
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let args = env::args().collect::<Vec<String>>();

    if args[1] == "-b" {
        info!("Building robotcode");

        let output = Command::new("sh")
            .args(["-c", "cargo build --release --target arm-unknown-linux-gnueabi"])
            .output()
            .expect("Failed to build robotcode");

        std::io::stdout().write_all(&output.stdout).unwrap();
    }

    let toml_str = fs::read_to_string("Cargo.toml").unwrap();

    let mut args: DeployArgs = toml::from_str::<Args>(&*toml_str).unwrap().deploy.unwrap();

    let mut ssh = Session::new()?;

    let addr = find_rio(args.team_number);

    let addr = match addr {
        Ok(ipv4) => {
            info!("Rio address parsed: {:?}", ipv4);
            ipv4
        }
        Err(e) => {
            error!("Failed to parse team number: {:?}", e);
            exit(1)
        }
    };

    let rio = TcpStream::connect_timeout(&SocketAddr::new(IpAddr::from(addr), 22), Duration::from_secs_f64(4.));
    let rio = match rio {
        Ok(stream) => {
            info!("Connected to rio");
            stream
        }
        Err(e) => {
            error!("Failed to connect to to rio: {}", e);
            exit(1)
        }
    };
    ssh.set_tcp_stream(rio);
    ssh.handshake()?;

    ssh.userauth_password("admin", "")?;
    assert!(ssh.authenticated());

    let mut channel = ssh.channel_session().unwrap();

    channel.exec("/usr/local/frc/bin/frcKillRobot.sh").unwrap();

    channel.send_eof().unwrap();

    channel = ssh.channel_session().unwrap();
    channel.exec(format!("rm {}", Path::new(&args.executable).file_name().unwrap().to_str().unwrap()).as_str()).unwrap();

    info!("Deploying executable");
    send_file(Path::new(&args.executable), Path::new("/home/lvuser"), &ssh).unwrap();

    if let Some(lib) = args.lib {
        info!("Deploying lib");
        send_file(Path::new(&lib), Path::new("/home/lvuser"), &ssh).unwrap();
    }

    info!("Writing to robotCommand");
    send_string(format!(    "JAVA_HOME=/usr/local/frc/JRE /home/lvuser/{}\n",
                        Path::new(&args.executable).file_name().unwrap().to_str().unwrap()),
                Path::new("/home/lvuser/robotCommand"),
                &ssh).unwrap();

    Ok(())
}
