mod spark;

pub use spark::*;
pub use spark::Spark as SparkFlex;
pub use spark::Spark as SparkMax;

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
    Velocity,
    Speed,
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
