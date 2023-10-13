mod spark;

pub use spark::*;

pub enum MotorType {
    Brushed,
    Brushless
}

impl MotorType {
    pub fn as_str(&self) -> &str {
        match &self {
            MotorType::Brushed => "kBrushed",
            MotorType::Brushless => "kBrushless"
        }
    }
}