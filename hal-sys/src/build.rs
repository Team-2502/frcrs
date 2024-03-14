use std::env;

use maven::{package::Package, WPI_MAVEN};

const NI_MAVEN: Package<'static> = Package {
    path: "edu.wpi.first.ni-libraries",
    ..WPI_MAVEN
};

fn main() -> anyhow::Result<()> {

    let headers = [
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

    headers[0].download_lib("wpiHal")?;
    headers[1].download_lib("wpiutil")?;
    headers[2].download_lib("wpimath")?;
    headers[3].download_lib("wpilibc")?;

    let runtime = Package {
        name: "runtime",
        ..NI_MAVEN
    };

    let chip_object = Package {
        name: "chipobject",
        ..NI_MAVEN
    };

    let net_comm = Package {
        name: "netcomm",
        ..NI_MAVEN
    };

    let visa = Package {
        name: "visa",
        ..NI_MAVEN
    };

    runtime.download_lib("embcanshim")?;
    println!("cargo:rustc-link-lib=dylib=fpgalvshim");
    chip_object.download_lib("RoboRIO_FRC_ChipObject")?;
    net_comm.download_lib("FRC_NetworkCommunication")?;
    visa.download_lib("visa")?;

    for package in headers.iter() {
        package.download_headers()?;
    }

    let out = env::var("OUT_DIR").unwrap();

    bindgen::Builder::default()
        .clang_arg("-xc++")
        .clang_arg("-std=c++20")
        .clang_arg(format!("-I{}/include", out))
        .clang_arg(format!("--sysroot={}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot", env!("HOME")))
        .clang_arg(format!("-I{}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include", env!("HOME")))
        .clang_arg(format!("-I{}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12", env!("HOME")))
        .clang_arg(format!("-I{}/.gradle/toolchains/frc/2024/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12/arm-nilrt-linux-gnueabi", env!("HOME")))
        .header("src/wrapper.h")
        .allowlist_item(".*HAL_.*")
        //.allowlist_file("wrapper.cpp")
        .opaque_type(".*strong_ordering.*")
        .opaque_type(".*Rb_tree.*")
        .opaque_type(".*Temporary_value.*")
        .generate()?
        .write_to_file(format!("{}/bindings.rs", out))?;

    //println!("cargo:rustc-link-search=native={}/{}", out, "libs");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/wrapper.cpp");
    println!("cargo:rerun-if-changed=src/build.rs");
    Ok(())
}
