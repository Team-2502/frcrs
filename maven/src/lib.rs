use package::Package;

pub mod package;

const WPI_MAVEN: Package<'static> = Package {
    maven_url: "frcmaven.wpi.edu/artifactory/release",
    version: "2024.2.1",
    path: "edu.wpi.first",
    name: ""
};
