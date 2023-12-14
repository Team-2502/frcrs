use std::io::Cursor;

use bytes::Bytes;
use thiserror::Error;
use zip::{read::ZipFile, ZipArchive};

use crate::zip::extract_libs;

#[derive(Debug)]
pub struct Artifact {
    group_id: String,
    artifact_id: String,
    version: String,
    maven_url: String,
    lib_name: Option<String>,
    headers: bool,
    should_deploy: bool,
}

impl Artifact {
    pub fn get_header_url(&self) -> String {
        format!(
            "{}{}/{}/{}/{}-{}-{}.zip",
            self.maven_url,
            self.group_id.replace(".", "/"),
            self.artifact_id,
            self.version,
            self.artifact_id,
            self.version,
            "headers"
        )
    }

    pub fn get_lib_url(&self) -> String {
        format!(
            "{}{}/{}/{}/{}-{}-{}.zip",
            self.maven_url,
            self.group_id.replace(".", "/"),
            self.artifact_id,
            self.version,
            self.artifact_id,
            self.version,
            "linuxathena"
        )
    }

    pub fn get_lib_name(&self) -> Option<&str> {
        self.lib_name.as_deref()
    }

    pub fn find_lib_in_zip<'a>(
        &self,
        archive: &'a mut ZipArchive<Cursor<Bytes>>,
    ) -> anyhow::Result<ZipFile<'a>> {
        let (_, file_number) = extract_libs(archive)?
            .into_iter()
            .find(|(name, _)| name == self.get_lib_name().unwrap())
            .unwrap();

        Ok(archive.by_index(file_number)?)
    }

    pub fn has_headers(&self) -> bool {
        self.headers
    }

    pub fn should_deploy(&self) -> bool {
        self.should_deploy
    }
}

impl Artifact {
    pub fn builder() -> Builder {
        Builder::new()
    }
}

pub struct Builder {
    group_id: Option<String>,
    artifact_id: Option<String>,
    version: Option<String>,
    maven_url: Option<String>,
    artifact_name: Option<String>,
    lib_name: Option<String>,
    headers: bool,
    should_deploy: bool,
}

#[derive(Clone, Copy)]
pub enum Target {
    Headers,
    RoboRio,
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Target::Headers => "headers".to_string(),
            Target::RoboRio => "linuxathena".to_string(),
        }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Missing group id (e.g. `com.google`)")]
    MissingGroupId,
    #[error("Missing artifact id (e.g. `gson`)")]
    MissingArtifactId,
    #[error("Missing version (e.g. `2.10.1`)")]
    MissingVersion,
    #[error("Missing maven url (e.g. `https://repo.maven.apache.org/maven2/`)")]
    MissingMavenUrl,
    #[error("Missing target or artifact name (only one is needed)")]
    MissingArtifactName,
}

impl Builder {
    fn new() -> Self {
        Self {
            group_id: None,
            artifact_id: None,
            version: None,
            maven_url: None,
            artifact_name: None,
            lib_name: None,
            headers: true,
            should_deploy: true,
        }
    }

    pub fn build(&self) -> Result<Artifact, Error> {
        let group_id = self.group_id.to_owned().ok_or(Error::MissingGroupId)?;
        let artifact_id = self
            .artifact_id
            .to_owned()
            .ok_or(Error::MissingArtifactId)?;
        let version = self.version.to_owned().ok_or(Error::MissingVersion)?;
        let maven_url = self.maven_url.to_owned().ok_or(Error::MissingMavenUrl)?;

        Ok(Artifact {
            group_id,
            artifact_id,
            version,
            maven_url,
            lib_name: self.lib_name.to_owned(),
            headers: self.headers,
            should_deploy: self.should_deploy,
        })
    }

    pub fn group_id(&mut self, group_id: String) -> &mut Self {
        self.group_id = Some(group_id);

        self
    }

    pub fn artifact_id(&mut self, artifact_id: String) -> &mut Self {
        self.artifact_id = Some(artifact_id);

        self
    }

    pub fn version(&mut self, version: String) -> &mut Self {
        self.version = Some(version);

        self
    }

    pub fn maven_url(&mut self, maven_url: String) -> &mut Self {
        self.maven_url = Some(maven_url);

        self
    }

    pub fn artifact_name(&mut self, artifact_name: String) -> &mut Self {
        self.artifact_name = Some(artifact_name);

        self
    }

    pub fn lib_name(&mut self, lib_name: String) -> &mut Self {
        self.lib_name = Some(lib_name);

        self
    }

    pub fn no_headers(&mut self) -> &mut Self {
        self.headers = false;

        self
    }

    pub fn no_deploy(&mut self) -> &mut Self {
        self.should_deploy = false;

        self
    }
}
