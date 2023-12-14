use std::path::Path;

use anyhow::Result;
use build_utils::{
    artifact::Artifact,
    build,
};

const WPI_MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";
const REV_MAVEN: &str = "https://maven.revrobotics.com/";

#[tokio::main]
async fn main() -> Result<()> {
    let headers = vec![
        Artifact::builder()
            .group_id("edu.wpi.first.hal".to_owned())
            .artifact_id("hal-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpiutil".to_owned())
            .artifact_id("wpiutil-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("edu.wpi.first.wpimath".to_owned())
            .artifact_id("wpimath-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .build()?,
        Artifact::builder()
            .group_id("com.revrobotics.frc".to_owned())
            .artifact_id("REVLib-driver".to_owned())
            .version("2023.1.3".to_owned())
            .maven_url(REV_MAVEN.to_owned())
            .lib_name("REVLibDriver".to_owned())
            .build()?,
    ];

    build(&headers, "c_(SparkMax|REVLib)_.*", &Path::new("rev/CANSparkMaxDriver.h")).await
}
