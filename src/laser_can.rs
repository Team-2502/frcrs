use jni::objects::{GlobalRef, JValue};
use jni::signature::{Primitive, ReturnType};
use nalgebra::Vector4;
use crate::java;

pub struct LaserCan {
    instance: GlobalRef,
}

pub enum RangingMode {
    Long,
    Short
}

impl LaserCan {
    pub fn new(id: i32) -> Self {
        let instance = create!(
            "au/grapplerobotics/LaserCan",
            "(I)V",
            &[JValue::Int(id).as_jni()]
        );

        Self {
            instance: java().new_global_ref(instance).unwrap(),
        }
    }

    pub fn set_ranging_mode(&self, ranging_mode: RangingMode) {
        let s = match ranging_mode {
            RangingMode::Long => "LONG",
            RangingMode::Short => "SHORT",
        };

        let mode = call_static!(
            "au/grapplerobotics/LaserCan$RangingMode",
            "valueOf",
            "(Ljava/lang/String;)Lau/grapplerobotics/LaserCan$RangingMode;",
            &[JValue::Object(&java().new_string(s).unwrap()).as_jni()],
            ReturnType::Object
        ).l().unwrap();

        call!(
            self.instance.as_obj(),
            "au/grapplerobotics/LaserCan",
            "setRangingMode",
            "(Lau/grapplerobotics/LaserCan$RangingMode;)V",
            &[JValue::Object(&mode).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        );
    }

    /// Gets the measurement from the laser can in mm
    pub fn get_measurement(&self) -> i32 {
        let ret = call!(
            self.instance.as_obj(),
            "au/grapplerobotics/LaserCan",
            "getMeasurement",
            "()Lau/grapplerobotics/interfaces/LaserCanInterface$Measurement;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        call_static!(
            "frc/robot/Wrapper",
            "laserCanMeasurement",
            "(Lau/grapplerobotics/interfaces/LaserCanInterface$Measurement;)I",
            &[JValue::Object(&ret).as_jni()],
            ReturnType::Primitive(Primitive::Int)
        ).i().unwrap()
    }
}
