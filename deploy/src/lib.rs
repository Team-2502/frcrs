#![feature(async_fn_in_trait)]

use std::{
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf}, fs,
};

use tokio::process::Command as AsyncCommand;

const LIB_DEPLOY_DIR: &str = "/usr/local/frc/third-party/lib";

pub struct TeamNumber(u32);

impl TeamNumber {
    pub fn new(num: u32) -> Self {
        assert!(num < 10000);

        Self(num)
    }
}

async fn find_rio_address(team_number: TeamNumber) -> Option<IpAddr> {
    let addr = IpAddr::V4(Ipv4Addr::new(
        10,
        (team_number.0 / 100) as u8,
        (team_number.0 % 100) as u8,
        2,
    ));

    if check_addr(addr).await {
        return Some(addr);
    }

    None
}

async fn check_addr(addr: IpAddr) -> bool {
    let payload = [0; 8];

    surge_ping::ping(addr, &payload).await.is_ok()
}

pub struct RioTarget(IpAddr);

pub async fn create_target(team_number: TeamNumber) -> Option<RioTarget> {
    let addr = find_rio_address(team_number).await?;

    Some(RioTarget(addr))
}

impl RioTarget {
    pub async fn execute(&mut self, command: &str) {
        AsyncCommand::new("ssh")
            .arg(format!("admin@{}", self.0))
            .arg("-t")
            .arg(command)
            .output()
            .await
            .expect("Could not execute command on target");
    }

    pub async fn execute_many<'a, I: Iterator<Item = &'a str>>(&mut self, commands: I) {
        AsyncCommand::new("ssh")
            .arg(format!(
                "admin@{}<<EOF\n{}\nEOF",
                self.0,
                commands.collect::<Vec<_>>().join("\n")
            ))
            .output()
            .await
            .expect("Could not execute command on target");
    }

    pub async fn put(&mut self, local: &Path, remote: &Path) {
        // Remove the old file (assuming it has the same name) otherwise will not overwrite
        AsyncCommand::new("ssh")
            .arg(format!(
                "admin@{}<<EOF\n{}\nEOF",
                self.0,
                format!("rm {}", local.file_name().unwrap().to_str().unwrap())
            ))
            .output()
            .await
            .expect("Could not execute command on target");

        AsyncCommand::new("scp")
            .args([
                local.to_str().unwrap(),
                &format!("admin@{}:{}", self.0, remote.to_str().unwrap()),
            ])
            .output()
            .await
            .expect("Could not copy file to target");
    }

    pub async fn run<A: Action>(&mut self, action: &mut A) {
        action.execute(self).await;
    }
}

pub trait Action {
    async fn execute(&mut self, target: &mut RioTarget);
}

pub struct ProgramKill;

impl Action for ProgramKill {
    async fn execute(&mut self, target: &mut RioTarget) {
        target
            .execute_many(
                [
                    ". /etc/profile.d/natinst-path.sh",
                    "/usr/local/frc/bin/frcKillRobot.sh -t 2> /dev/null",
                ]
                .into_iter(),
            )
            .await;
    }
}

pub struct ProgramStart;

impl Action for ProgramStart {
    async fn execute(&mut self, target: &mut RioTarget) {
        target
            .execute_many(
                [
                    "sync",
                    ". /etc/profile.d/natinst-path.sh",
                    "/usr/local/frc/bin/frcKillRobot.sh -t -r 2> /dev/null",
                ]
                .into_iter(),
            )
            .await;
    }
}

pub struct ProgramRun<'a> {
    pub local: &'a Path,
}

impl<'a> Action for ProgramRun<'a> {
    async fn execute(&mut self, target: &mut RioTarget) {
        let remote =
            PathBuf::from("/home/lvuser/").join(self.local.file_name().unwrap().to_str().unwrap());
        target.put(self.local, &remote).await;
        target
            .execute(&format!(
                "echo \"{}\" > /home/lvuser/robotCommand",
                remote.to_str().unwrap()
            ))
            .await;
        target
            .execute("chmod +x /home/lvuser/robotCommand; chown lvuser /home/lvuser/robotCommand")
            .await;

        let remote = remote.to_str().unwrap();

        target
            .execute_many(
                [
                    format!("chmod +x \"{}\"", remote).as_str(),
                    format!("chown lvuser \"{}\"", remote).as_str(),
                    format!("setcap cap_sys_nice+eip \"{}\"", remote).as_str(),
                ]
                .into_iter(),
            )
            .await;
    }
}

pub struct ConfigureLibs;

impl Action for ConfigureLibs {
    async fn execute(&mut self, target: &mut RioTarget) {
        target.execute_many([
            format!("chmod -R 777 \"{}\" || true", LIB_DEPLOY_DIR).as_str(),
            format!("chown -R lvuser:ni \"{}\"", LIB_DEPLOY_DIR).as_str(),
            "ldconfig"
        ].into_iter()).await;
    }
}

pub struct DeployLibs<'a> {
    pub dir: &'a Path
}

impl<'a> Action for DeployLibs<'a> {
    async fn execute(&mut self, target: &mut RioTarget) {
        for lib in fs::read_dir(self.dir).expect("Could not find files in libs dir") {
            let lib = lib.expect("Error getting directory entry");

            target.put(&lib.path(), &Path::new(LIB_DEPLOY_DIR).join(lib.file_name())).await;
        }
    }
}
