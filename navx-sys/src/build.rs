use std::env;

use maven::{package::Package, EMPTY, WPI_MAVEN, get_wpilib_toolchain_location};

const KAUAI_MAVEN: Package<'static> = Package {
    maven_url: "dev.studica.com/maven/release/2024",
    path: "com.kauailabs.navx.frc",
    version: "2024.1.0",
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
        Package {
            path: "edu.wpi.first.wpilibc",
            name: "wpilibc-cpp",
            ..WPI_MAVEN
        },
        Package {
            path: "edu.wpi.first.wpinet",
            name: "wpinet-cpp",
            ..WPI_MAVEN
        },
    ];

    dependencies[0].download_lib("wpiHal")?;
    dependencies[1].download_lib("wpiutil")?;
    dependencies[2].download_lib("wpimath")?;
    dependencies[3].download_lib("wpilibc")?;
    dependencies[4].download_lib("wpinet")?;

    let navx_frc = Package {
        name: "navx-frc-cpp",
        ..KAUAI_MAVEN
    };

    let ntcore = Package {
        path: "edu.wpi.first.ntcore",
        name: "ntcore-cpp",
        ..WPI_MAVEN
    };


    for package in dependencies.iter().chain([&ntcore,&navx_frc].into_iter()) {
        package.download_headers()?;
    }

    navx_frc.download_lib("NavX").unwrap();
    ntcore.download_lib("ntcore").unwrap();

    let out = env::var("OUT_DIR").unwrap();

    let toolchain = get_wpilib_toolchain_location()?;

    bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}/include", out))
        .clang_arg(format!("--sysroot={toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot"))
        .clang_arg(format!("-I{toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include"))
        .clang_arg(format!("-I{toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12"))
        .clang_arg(format!("-I{toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12/arm-nilrt-linux-gnueabi"))
        .header("src/wrapper.cpp")
        .allowlist_file("wrapper.cpp")
        .allowlist_file("AHRS.h")
        .allowlist_item("navx_wrapper.*")
        .opaque_type(".*strong_ordering.*")
        .opaque_type(".*Rb_tree.*")
        .opaque_type(".*Temporary_value.*")
        .generate()?
        .write_to_file(format!("{}/bindings.rs", out))?;

    cc::Build::new()
        .cpp(true)
        .std("c++20")
        .cpp_link_stdlib("stdc++")
        .flag(&format!("--sysroot={toolchain}/roborio/arm-nilrt-linux-gnueabi/sysroot"))
        .include(format!("{}/include", out))
        .file("src/wrapper.cpp")
        .compile("navx-wrapper");

    //println!("cargo:rustc-link-search=native={}/{}", out, "libs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/build.rs");
    Ok(())
}
