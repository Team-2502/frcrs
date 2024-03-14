use std::env;

use maven::{package::Package, EMPTY, WPI_MAVEN, get_wpilib_toolchain_location};

const CTRE_MAVEN: Package<'static> = Package {
    maven_url: "maven.ctr-electronics.com/release",
    path: "com.ctre.phoenix6",
    version: "24.1.0",
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
    ];

    dependencies[0].download_lib("wpiHal")?;
    dependencies[1].download_lib("wpiutil")?;
    dependencies[2].download_lib("wpimath")?;
    dependencies[3].download_lib("wpilibc")?;


    let ctre_tools = Package {
        name: "tools",
        ..CTRE_MAVEN
    };

    let ctre_wpi = Package {
        name: "wpiapi-cpp",
        ..CTRE_MAVEN
    };


    for package in dependencies.iter().chain([&ctre_wpi,&ctre_tools].into_iter()) {
        package.download_headers()?;
    }

    ctre_tools.download_lib("CTRE_PhoenixTools").unwrap();
    ctre_wpi.download_lib("CTRE_Phoenix6_WPI").unwrap();

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
        .allowlist_item("talonfx_wrapper.*")
        .allowlist_item("cancoder_wrapper.*")
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
        //.flag(&format!("--with-native-headers={}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12", env!("HOME")))
        //.include(format!("{}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include", env!("HOME")))
        //.include(format!("{}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12", env!("HOME")))
        .include(format!("{}/include", out))
        .file("src/wrapper.cpp")
        .compile("ctre-wrapper");

    //println!("cargo:rustc-link-search=native={}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include", env!("HOME"));
    //println!("cargo:rustc-link-search=native={}/{}", out, "libs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    Ok(())
}
