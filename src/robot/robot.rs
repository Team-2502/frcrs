use crate::robot::{Command, Subsystem};

pub struct Robot {
    subsystems: Vec<Box<dyn Subsystem>>,
    commands: Vec<Box<dyn Command>>
}

impl Robot {
    pub fn new() -> Self {
        Self {
            subsystems: vec![],
            commands: vec![]
        }
    }
}