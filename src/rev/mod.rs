mod spark;
mod pid;

pub use pid::*;
pub use spark::*;
pub use spark::JavaSpark as Spark;

pub enum MotorType {
    Brushed,
    Brushless
}

pub enum IdleMode {
    Brake,
    Coast
}

pub enum ControlType {
    Position,
}

impl MotorType {
    pub fn as_str(&self) -> &str {
        match &self {
            MotorType::Brushed => "kBrushed",
            MotorType::Brushless => "kBrushless"
        }
    }
}

impl IdleMode {
    pub fn as_str(&self) -> &str {
        match &self {
            IdleMode::Brake => "kBrake",
            IdleMode::Coast => "kCoast"
        }
    }
}

impl ControlType {
    pub fn as_str(&self)-> &str {
        match &self {
            ControlType::Position => "kPosition"
        }
    }
}
