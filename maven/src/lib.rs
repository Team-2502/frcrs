#![feature(fs_try_exists)]
use std::{os, fs, path::Path};

use anyhow::{Result, anyhow};
use package::Package;

pub mod package;

pub const WPI_MAVEN: Package<'static> = Package {
    maven_url: "frcmaven.wpi.edu/artifactory/release",
    version: "2024.2.1",
    ..EMPTY
};

pub const EMPTY: Package<'static> = Package {
    maven_url: "",
    version: "",
    path: "",
    name: ""
};

pub fn get_wpilib_toolchain_location() -> Result<String> {
    let gradle = format!("{}/.gradle/toolchains/frc/2024", env!("HOME"));
    let wpi = format!("{}/wpilib/2024", env!("HOME"));
    if fs::try_exists(&gradle)? {
        Ok(gradle)
    } else if fs::try_exists(&wpi)? {
        Ok(wpi)
    } else {
        Err(anyhow!("frc toolchain not found"))
    }
}
