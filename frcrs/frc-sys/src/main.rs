extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let gradle_include_path = PathBuf::from("../../../cppstub/build/tmp/");

    // read gradle compile options
    let mut compile_option_path;
    let compile_options = fs::read_to_string(
        {
            compile_option_path = gradle_include_path.clone();
            compile_option_path.push("compileFrcUserProgramReleaseExecutableFrcUserProgramCpp");
            compile_option_path.push("options");
            compile_option_path.set_extension("txt");
            println!("{:?}",compile_option_path.as_os_str());
            compile_option_path.as_path()
        }
        ).unwrap();
    // extract header paths
    let header_paths = parse_header_paths(compile_options);

    for header_path in header_paths {
        println!("{}",header_path);
    }

    /*
    let header = String::new(); // generated header to bind

    for header_path in header_paths { 

    }
    */


    /*
    let linkOptions = 0;
    let link_paths = parse_link_paths(compileOptions);

    // link wpilib/dep libraries
    for link_path in link_paths {
        println!("cargo:rustc-link-search={}",link_path);
    };
    */
}

/// returns a vec of the header paths in the command line args
fn parse_header_paths(options: String) -> Vec<String> {
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
