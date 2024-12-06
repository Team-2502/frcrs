use crate::{com_ctre_phoenix_motorcontrol_can_TalonSRX, com_ctre_phoenix_motorcontrol_TalonSRXControlMode, java};

pub struct SRX {
    motor: com_ctre_phoenix_motorcontrol_can_TalonSRX
}

impl SRX {
    pub fn new(id: i32) -> Self {
        Self {
            motor: com_ctre_phoenix_motorcontrol_can_TalonSRX::new(id).unwrap()
        }
    }

    pub fn set(&self, value: f64) {
        let str = java().new_string("PercentOutput").unwrap();
        let control = com_ctre_phoenix_motorcontrol_TalonSRXControlMode::valueOf(&str);
        com_ctre_phoenix_motorcontrol_can_TalonSRX::set(&self.motor.inner(), &control.unwrap(), value).unwrap();
    }

    pub fn stop(&self) {
        self.set(0.0);
    }
}