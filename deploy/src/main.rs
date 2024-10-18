use std::{env, fs};
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream};
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

    lib: Option<String>,
    frontend: Option<String>,
    frontend_dest: Option<String>
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let call_args = env::args().collect::<Vec<String>>();

    let toml_str = fs::read_to_string("Cargo.toml").unwrap();

    let mut args: DeployArgs = toml::from_str::<Args>(&*toml_str).unwrap().deploy.unwrap();

    let mut ssh = Session::new()?;

    let mut addr = find_rio(args.team_number);

    if call_args.contains(&"-u".to_owned()) {
        addr = Ok(Ipv4Addr::new(172, 22, 11, 2))
    }

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

    if call_args.contains(&"-b".to_owned()) {
        info!("Building robotcode");

        let output = Command::new("cargo")
            .args(["build --release --target arm-unknown-linux-gnueabi"])
            .output()
            .expect("Failed to build robotcode");

        std::io::stdout().write_all(&output.stdout).unwrap();
    }

    if call_args.contains(&"-f".to_owned()) {
        info!("Deploying frontend");

        let frontend_dir = if args.frontend.as_ref().is_some() {
            args.frontend.as_ref().unwrap()
        } else {
            error!("Frontend directory not specified");
            exit(1);
        };

        let frontend_dest = if args.frontend_dest.as_ref().is_some() {
            args.frontend_dest.as_ref().unwrap()
        } else {
            error!("Frontend destination not specified");
            exit(1);
        };

        for entry in fs::read_dir(frontend_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                info!("Deploying frontend file: {:?}", path.display());

                let remote_dest = Path::new(frontend_dest).join(path.file_name().unwrap());
                send_file(&path, &remote_dest, &ssh)?;
            }
        }
    }

    if call_args.contains(&"-l".to_owned()) {
        if let Some(lib) = args.lib {
            info!("Deploying lib");
            send_file(Path::new(&lib), Path::new("/home/lvuser"), &ssh).unwrap();
        } else {
            error!("Library file not specified");
            exit(1);
        }
    }

    let mut channel = ssh.channel_session().unwrap();

    channel.exec("/usr/local/frc/bin/frcKillRobot.sh").unwrap();

    channel.send_eof().unwrap();

    channel = ssh.channel_session().unwrap();
    channel.exec(format!("rm {}", Path::new(&args.executable).file_name().unwrap().to_str().unwrap()).as_str()).unwrap();

    info!("Deploying executable");
    send_file(Path::new(&args.executable), Path::new("/home/lvuser"), &ssh).unwrap();

    info!("Writing to robotCommand");
    send_string(format!(    "JAVA_HOME=/usr/local/frc/JRE /home/lvuser/{}\n",
                        Path::new(&args.executable).file_name().unwrap().to_str().unwrap()),
                Path::new("/home/lvuser/robotCommand"),
                &ssh).unwrap();

    Ok(())
}
