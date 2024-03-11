use package::Package;

pub mod package;

pub const WPI_MAVEN: Package<'static> = Package {
    maven_url: "frcmaven.wpi.edu/artifactory/release",
    version: "2024.2.1",
    ..EMPTY
};

pub const EMPTY: Package<'static> = Package {
    maven_url: "",
    version: "",
    path: "",
    name: ""
};
