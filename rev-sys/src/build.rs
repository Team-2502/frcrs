use std::env;

use maven::{package::Package, EMPTY, WPI_MAVEN, get_wpilib_toolchain_location};

const REV_MAVEN: Package<'static> = Package {
    maven_url: "maven.revrobotics.com",
    path: "com.revrobotics.frc",
    version: "2024.1.1",
    ..EMPTY
};

fn main() -> anyhow::Result<()> {

    let dependencies = [
        Package {
            path: "edu.wpi.first.hal",
            name: "hal-cpp",
            ..WPI_MAVEN
        },
        Package {
            path: "edu.wpi.first.wpiutil",
            name: "wpiutil-cpp",
            ..WPI_MAVEN
        },
        Package {
            path: "edu.wpi.first.wpimath",
            name: "wpimath-cpp",
            ..WPI_MAVEN
        },
    ];

    dependencies[0].download_lib("wpiHal")?;
    dependencies[1].download_lib("wpiutil")?;
    dependencies[2].download_lib("wpimath")?;

    let revlib = Package {
        name: "REVLib-driver",
        ..REV_MAVEN
    };

    for package in dependencies.iter().chain([&revlib].into_iter()) {
        package.download_headers()?;
    }

    revlib.download_lib("REVLibDriver")?;

    let out = env::var("OUT_DIR")?;

    let toolchain = get_wpilib_toolchain_location()?;

    bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}/include", out))
        .clang_arg(format!("--sysroot={toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot"))
        .clang_arg(format!("-I{toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include"))
        .clang_arg(format!("-I{toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12"))
        .clang_arg(format!("-I{toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12/arm-nilrt-linux-gnueabi"))
        .header(format!("{out}/include/rev/CANSparkMaxDriver.h"))
        .allowlist_item(".*SparkMax.*")
        .allowlist_item(".*REVLib.*")
        .opaque_type(".*strong_ordering.*")
        .opaque_type(".*Rb_tree.*")
        .opaque_type(".*Temporary_value.*")
        .generate()?
        .write_to_file(format!("{}/bindings.rs", out))?;

    //println!("cargo:rustc-link-search=native={}/{}", out, "libs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    Ok(())
}
