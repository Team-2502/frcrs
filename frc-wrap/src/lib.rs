use std::path::PathBuf;
use pyo3::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Read;
use glob::glob;

pub fn test() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();
    let gil = Python::acquire_gil();
    let py = gil.python();
    println!("Version: {:?}", py.version_info());
    let wpilib = PyModule::import(py, "wpilib")?;

    Ok(())
}