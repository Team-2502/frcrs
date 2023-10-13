mod spark;

pub use spark::*;

pub enum MotorType {
    Brushed,
    Brushless
}

pub enum IdleMode {
    Brake,
    Coast
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
