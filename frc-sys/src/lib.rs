use std::path::PathBuf;
use pyo3::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Read;

pub fn test() {
    pyo3::prepare_freethreaded_python();
    let gil = Python::with_gil(|py| -> PyResult<()> {
        println!("Version: {:?}", py.version_info());
        let wpilib = PyModule::import(py, "wpilib")?;

        wpilib.getattr("SmartDashboard.putData")?.call1(("test", "data"));
        Ok(())
    });
    teleop();
}

fn teleop() {
    pyo3::prepare_freethreaded_python();
    loop {
        let gil = Python::with_gil(|py| -> PyResult<()> {
            let wpilib = PyModule::import(py, "wpilib")?;

            wpilib.getattr("SmartDashboard.putData")?.call1(("test", "data"));
            Ok(())
        });
    }
}