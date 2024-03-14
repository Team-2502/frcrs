use std::{path::{Path, PathBuf}, io::{Cursor, Write}, str::FromStr, fs::{self, OpenOptions}};

use anyhow::{Ok, Context};

#[derive(Default)]
pub struct Package<'a> {
    pub maven_url: &'a str,
    pub path: &'a str,
    pub name: &'a str,
    pub version: &'a str,
}

pub struct Artifact<'a> {
    pub package: &'a Package<'a>,
    pub name: &'a str,
}

impl<'a> Package<'a> {
    fn url(&self) -> String {
        format!("https://{}/{}/{}/{}", self.maven_url, self.path.replace(".", "/"), self.name, self.version)
    }

   pub fn artifact(&'a self, name: &'a str) -> Artifact<'a> {
        Artifact { package: &self, name }
    }

    pub fn download_headers(&self) -> anyhow::Result<()> {

        let mut path = PathBuf::from_str(std::env::var("OUT_DIR")?.as_str())?;
        path.push("include");
        self.artifact("headers").download(&path)?;

        Ok(())
    }

    pub fn download_lib(&self, lib: &str) -> anyhow::Result<()> {
        let mut path = PathBuf::from_str(std::env::var("OUT_DIR")?.as_str())?;
        path.push("libs");
        self.artifact("linuxathena").download(&path)?;

        // rename all so.version files to just so, to make arm-frc2023-linux-gnueabi-gcc happy
        // i'm making the assumption that files will be listed alphabetically 
        // (and that that matches so version order exactly)
        for file in glob::glob(&format!("{}/**/*.so*", path.to_string_lossy()))? { 
            let file = file?;
            let trimmed = file.clone().file_name().unwrap()
                .to_string_lossy()
                .split_once('.').unwrap().0.to_owned();
            let trimmed = format!("{trimmed}.so");

            let mut normalized = path.clone();
            normalized.push(trimmed);
            fs::rename(file,normalized)?;
        }

        println!("cargo:rustc-link-search=native={}", path.to_string_lossy());
        println!("cargo:rustc-link-lib=dylib={}", lib); // TODO: multi file pushes

        Ok(())
    }
}

impl<'a> Artifact<'a> {
    fn url(&self) -> String {
        format!("{}/{}-{}-{}.zip", self.package.url(), self.package.name, self.package.version, self.name)
    }

    pub fn download(&self, path: &Path) -> anyhow::Result<()> {
        if downloaded(&self.url()) {
            return Ok(());
        }

        let data = reqwest::blocking::get(self.url())?.bytes()?;

        let mut path = path.to_owned();

        match self.package.name {
            "hal-cpp" => {
                path = path.join("hal");
            },
            _ => {},
        };
        fs::create_dir_all(&path).unwrap();
        zip_extract::extract(Cursor::new(data), &path, true)?;

        Ok(())
    }
}

fn downloaded(url: &str) -> bool {
    let path = format!("{}/downloaded.txt", std::env::var("OUT_DIR").unwrap());
    if fs::read_to_string(&path).unwrap_or(Default::default()).lines().into_iter().find(|l| **l == *url).is_some() {
        return true;
    }

    let mut file = OpenOptions::new().create(true).append(true).open(path).unwrap();
    file.write(format!("{}\n",url).as_bytes()).unwrap();
    file.flush().unwrap();

    false
}


#[cfg(test)]
mod tests {
    use crate::WPI_MAVEN;

    use super::Package;


    #[test]
    fn download() {
        let hal = Package {
            path: "edu.wpi.first.hal",
            name: "hal-cpp",
            ..WPI_MAVEN
        };

        hal.download_headers().unwrap();
        println!("{}", std::env::var("OUT_DIR").unwrap());
    }

}
