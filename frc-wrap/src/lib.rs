use std::path::PathBuf;
use pyo3::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Read;
use frc_sys::test;

pub fn wrap_test() {
    test();
}