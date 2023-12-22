use std::path::Path;

use anyhow::Result;
use build_utils::{
    artifact::Artifact,
    build,
};

const WPI_MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";
const CTRE_MAVEN: &str = "https://maven.ctr-electronics.com/release/";

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
            .group_id("com.ctre.phoenix".to_owned())
            .artifact_id("wpiapi-cpp".to_owned())
            .version("5.31.0".to_owned())
            .maven_url(CTRE_MAVEN.to_owned())
            .lib_name("CTRE_Phoenix_WPI".to_owned())
            .build()?,
        /*Artifact::builder()
            .group_id("com.ctre.phoenix".to_owned())
            .artifact_id("cci".to_owned())
            .version("5.30.3".to_owned())
            .maven_url(CTRE_MAVEN.to_owned())
            .lib_name("CTRE_PhoenixCCI".to_owned())
            .build()?,*/
        Artifact::builder()
            .group_id("com.ctre.phoenix".to_owned())
            .artifact_id("api-cpp".to_owned())
            .version("5.30.3".to_owned())
            .maven_url(CTRE_MAVEN.to_owned())
            .lib_name("CTRE_Phoenix".to_owned())
            .build()?,
        Artifact::builder()
            .group_id("com.ctre.phoenixpro".to_owned())
            .artifact_id("tools".to_owned())
            .version("23.0.10".to_owned())
            .maven_url(CTRE_MAVEN.to_owned())
            .lib_name("CTRE_PhoenixTools".to_owned())
            .build()?,
    ];

    build(&headers, ".*", &Path::new("ctre/phoenix/motorcontrol/can/TalonFX.h")).await
    //build(&headers, "c_MotController_.*", &Path::new("ctre/phoenix/cci/MotController_CCI.h")).await
    //build(&headers, "WPI_.*", &Path::new("ctre/phoenix/motorcontrol/can/WPI_TalonFX.h")).await
}
