use std::{
    env,
    fs::{self, File},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use anyhow::bail;
use artifact::Artifact;
use reqwest::Client;
use tempfile::TempDir;

use crate::zip::{get_zip, write_archive_to_path};

pub mod artifact;
pub mod zip;

static CLIENT: OnceLock<Client> = OnceLock::new();
pub const WPI_VERSION: &str = "2023.4.3";

pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

pub async fn build(artifacts: &[Artifact], allow: &str, path: &Path, ) -> anyhow::Result<()> {
    let tempdir = TempDir::new()?;
    let include_path = tempdir.path().join("include");

    fs::create_dir_all(&include_path)?;

    for header in artifacts.iter().filter(|artifact| artifact.has_headers()) {
        write_archive_to_path(&include_path, get_zip(&header.get_header_url()).await?)?;
    }

    if let Ok(host) = env::var("HOST") {
        env::set_var("TARGET", host);
    }

    let result = bindgen::Builder::default()
        .clang_args([
            "-xc++",
            "-std=c++20",
            &format!("--include-directory={}", include_path.to_str().unwrap()),
        ])
        .header(include_path.join(path).to_str().unwrap())
        .allowlist_type(allow)
        .allowlist_function(allow)
        .allowlist_var(allow)
        .generate()?;

    if let Some(out_str) = env::var_os("OUT_DIR") {
        let out_dir = Path::new(&out_str);

        result.write_to_file(out_dir.join("bindings.rs"))?;
    }

    let Some(libs_dir) = env::var_os("OUT_DIR").map(|dir| PathBuf::from(dir).join("lib")) else {
        bail!("Unable to find libs dir");
    };

    let out_dir = env::var("LIBS_OUT_DIR")
        .map(|dir| PathBuf::from(dir).join("lib"))
        .ok();

    fs::create_dir_all(&libs_dir)?;
    if let Some(out_dir) = &out_dir {
        fs::create_dir_all(out_dir)?;
    }

    println!(
        "cargo:rustc-link-search=native={}",
        libs_dir.to_str().unwrap()
    );

    for lib in artifacts.iter().filter(|artifact| artifact.get_lib_name().is_some()) {
        let mut archive = get_zip(&lib.get_lib_url()).await?;

        let mut zip_file = lib.find_lib_in_zip(&mut archive)?;

        let mut fs_file =
            File::create(libs_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

        std::io::copy(&mut zip_file, &mut fs_file)?;

        if let Some(out_dir) = &out_dir {
            if lib.should_deploy() {
                let mut fs_file =
                    File::open(libs_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

                let mut out_file =
                    File::create(out_dir.join(format!("lib{}.so", lib.get_lib_name().unwrap())))?;

                std::io::copy(&mut fs_file, &mut out_file)?;
            }
        }

        println!("cargo:rustc-link-lib=dylib={}", lib.get_lib_name().unwrap());
    }

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}
