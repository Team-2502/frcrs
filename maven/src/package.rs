pub struct Package {
    pub maven_url: String,
    pub group: String,
    pub name: String,
    pub version: String,
}

pub struct Artifact<'a> {
    pub package: &'a Package,
    pub name: String,
}

impl Package {
    fn url(&self) -> String {
        format!("https://{}/{}/{}/{}", self.maven_url, self.group.replace(".", "/"), self.name, self.version)
    }

    fn artifact(&self, name: String) -> Artifact {
        Artifact { package: &self, name }
    }
}

impl<'a> Artifact<'a> {
    fn url(&self) -> String {
        format!("{}/{}-{}-{}.zip", self.package.url(), self.package.name, self.package.version, self.name)
    }

    pub fn download(&self) {
        dbg!(self.url());
    }
}
