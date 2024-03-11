use std::{env, path::{Path, PathBuf}, io::Cursor, str::FromStr};

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
}

impl<'a> Artifact<'a> {
    fn url(&self) -> String {
        format!("{}/{}-{}-{}.zip", self.package.url(), self.package.name, self.package.version, self.name)
    }

    pub fn download(&self, path: &Path) -> anyhow::Result<()> {
        let data = reqwest::blocking::get(self.url())?.bytes()?;
        zip_extract::extract(Cursor::new(data), &path, true)?;

        Ok(())
    }
}


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
