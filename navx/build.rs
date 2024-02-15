use std::path::Path;

use anyhow::Result;
use build_utils::{
    artifact::Artifact,
    build,
};

const WPI_MAVEN: &str = "https://frcmaven.wpi.edu/artifactory/release/";
const NAVX_MAVEN: &str = "https://dev.studica.com/maven/release/2024/";

#[tokio::main]
async fn main() -> Result<()> {
    // Always rerun
    //println!("cargo:rerun-if-changed=NULL");

    let headers = vec![
        /*Artifact::builder()
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
            .build()?,*/
        Artifact::builder()
            .group_id("edu.wpi.first.ntcore".to_owned())
            .artifact_id("ntcore-cpp".to_owned())
            .version(build_utils::WPI_VERSION.to_owned())
            .maven_url(WPI_MAVEN.to_owned())
            .lib_name("ntcore".to_owned())
            .build()?,
        Artifact::builder()
            .group_id("com.kauailabs.navx.frc".to_owned())
            .artifact_id("navx-frc-cpp".to_owned())
            .version("2024.1.0".to_owned())
            .maven_url(NAVX_MAVEN.to_owned())
            .lib_name("NavX".to_owned())
            .build()?,
    ];

    build(&headers, "CreateNavX", &Path::new("NavxWrapper.h")).await
    //build(&headers, "AHRS", &Path::new("AHRS.h")).await
}
