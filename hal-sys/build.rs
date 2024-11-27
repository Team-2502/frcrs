use std::env;
use std::path::Path;
use auto_jni::call::generate_bindings_file;

// jar xf ..\javastub.jar
// javap -s edu.wpi.first.hal.HAL
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out = env::var("OUT_DIR").unwrap();
    let file = Path::new(&out).join("bindings.rs");
    let class_name = vec![
        "edu.wpi.first.hal.HAL",
        "edu.wpi.first.wpilibj.DriverStation",
        "edu.wpi.first.hal.DriverStationJNI"
    ];
    let class_path = Some("Z:\\frcrs\\hal-sys\\unwrapped".to_string());

    let options = vec![
        "-XX:+UseSerialGC".to_string(),
        "-Djava.lang.invoke.stringConcat=BC_SB".to_string(),
        "-Djava.library.path=/usr/local/frc/third-party/lib".to_string(),
        "-Djava.class.path=/home/lvuser/javastub.jar".to_string(),
    ];

    generate_bindings_file(class_name, class_path, &*file, Some(options)).expect("TODO: panic message");
}