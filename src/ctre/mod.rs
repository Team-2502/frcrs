mod talon;
mod cancoder;

pub use talon::*;
pub use cancoder::*;

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