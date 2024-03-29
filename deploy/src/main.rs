use std::net::{SocketAddrV4, TcpStream};
use std::path::Path;
use clap::Parser;
use ssh2::Session;
use deploy::{find_rio, send_file, send_string};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    team_number: usize,

    #[arg(short, long)]
    executable: String,

    #[arg(short, long)]
    lib: Option<String>
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut ssh = Session::new()?;

    let addr = find_rio(args.team_number).unwrap();

    let rio = TcpStream::connect(SocketAddrV4::new(addr, 22))?;
    ssh.set_tcp_stream(rio);
    ssh.handshake()?;

    ssh.userauth_password("admin", "")?;
    assert!(ssh.authenticated());

    println!("Target set to {}", addr);

    println!("Deploying executable");
    send_file(Path::new(&args.executable), Path::new("/home/lvuser"), &ssh).unwrap();

    if let Some(lib) = args.lib {
        println!("Deploying lib");
        send_file(Path::new(&lib), Path::new("/home/lvuser"), &ssh).unwrap();
    }

    println!("Writing to robotCommand");
    send_string(format!("JAVA_HOME=/usr/local/frc/JRE /home/lvuser/{}",
                        Path::new(&args.executable).file_name().unwrap().to_str().unwrap()),
                Path::new("/home/lvuser/robotCommand"),
                &ssh).unwrap();

    Ok(())
}
