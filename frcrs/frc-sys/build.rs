extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs;
use glob::glob;

fn main() {
    let gradle_include_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../cppstub/build/tmp/");

    // read gradle compile options
    let mut compile_option_path;
    let compile_options = fs::read_to_string(
        {
            compile_option_path = gradle_include_path.clone();
            compile_option_path.push("compileFrcUserProgramReleaseExecutableFrcUserProgramCpp");
            compile_option_path.push("options");
            compile_option_path.set_extension("txt");
//            println!("{:?}",compile_option_path.as_os_str());
            compile_option_path.as_path()
        }
        ).unwrap();
    // extract header paths
    let header_paths = parse_header_paths(&compile_options);


    let mut header = String::new(); // generated header to bind

    // include every header
    for header_path in header_paths { 
        println!("cargo:warning={}",header_path);
        // iterate over all headers in paths
        for entry in glob(&(header_path.clone()+"/**/*.h")).unwrap() {
            // generate line of header
            header.push_str(&format!("#include \"{}\"\n", entry.unwrap().as_os_str().to_str().unwrap()));
        }
        for entry in glob(&(header_path+"/**/*.hpp")).unwrap() {
            // generate line of header
            header.push_str(&format!("#include \"{}\"\n", entry.unwrap().as_os_str().to_str().unwrap()));
        }
    }

    // read gradle linker options
    let mut link_option_path;
    let link_options = fs::read_to_string(
        {
            link_option_path = gradle_include_path.clone();
            link_option_path.push("linkFrcUserProgramReleaseExecutable");
            link_option_path.push("options");
            link_option_path.set_extension("txt");
            link_option_path.as_path()
        }
        ).unwrap();


    let link_paths = parse_link_paths(link_options);

    // link wpilib/dep libraries
    for link_path in link_paths {
        println!("cargo:rustc-link-search={}",link_path);
    };

    println!("cargo:warning={}",&header);

    let  bindings = bindgen::Builder::default()
        .header_contents("frcCpp.h", &header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(compile_options.lines())
        .clang_arg("-stdlib=libc++")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).unwrap();
}

/// returns a vec of the header paths in the command line args
fn parse_header_paths(options: &String) -> Vec<String> {
    let mut header_paths: Vec<String> = Vec::new();
    let mut include: bool = false; // keeps track of include paths
    for line in options.lines() {
        if include {
            header_paths.push(line.to_string());
        }
        include = line == "-I"; // mark the next line as a header path
    };
    header_paths
}

/// returns a vec of the linker paths given the command line args
fn parse_link_paths(options: String) -> Vec<String> {
    let mut link_paths: Vec<String> = Vec::new();
    for line in options.lines() {
        // assume a path if first char is a '/' (exceedingly portable)
        if line.chars().nth(0) == Some('/') {
            link_paths.push(line.to_string());
        }
    };
    link_paths
}
