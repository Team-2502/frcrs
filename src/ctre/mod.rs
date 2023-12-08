mod talon;

pub use talon::*;

pub enum TalonInvertType {
    CounterClockwise,
}

impl TalonInvertType {
    pub fn as_str(&self) -> &str {
        match self {
            TalonInvertType::CounterClockwise => "TalonFXCounterClockwise",
        }
    }
}