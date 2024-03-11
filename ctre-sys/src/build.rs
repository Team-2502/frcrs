use std::env;

use maven::{package::Package, EMPTY, WPI_MAVEN};

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

    bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}/include", out))
        .header("src/wrapper.hpp")
        .allowlist_file("wrapper.hpp")
        .allowlist_item("test")
        .allowlist_item("CreateTalonFX")
        .opaque_type(".*strong_ordering.*")
        .opaque_type(".*Rb_tree.*")
        .opaque_type(".*Temporary_value.*")
        .generate()?
        .write_to_file(format!("{}/bindings.rs", out))?;

    Ok(())
}
