use crate::call::{call, create};
use crate::java;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use nalgebra::{Rotation3, Vector3};

pub struct Pigeon {
    instance: GlobalRef,
}

impl Pigeon {
    pub fn new(id: i32, can: Option<String>) -> Self {
        let string = java().new_string(can.unwrap_or("rio".to_string())).unwrap();

        let instance = create!(
            "com/ctre/phoenix6/hardware/Pigeon2",
            "(ILjava/lang/String;)V",
            &[
                JValue::Int(id).as_jni(),
                JValue::Object(&JObject::from(string)).as_jni()
            ]
        );

        Self {
            instance: java().new_global_ref(instance).unwrap(),
        }
    }

    pub fn get_angle(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/Pigeon2",
            "getAngle",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    pub fn get_rotation(&self) -> Vector3<f64> {
        let rotation = call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/Pigeon2",
            "getRotation3d",
            "()Ledu/wpi/first/math/geometry/Rotation3d;",
            &Vec::new(),
            ReturnType::Object
        )
        .l()
        .unwrap();

        let x = call!(
            rotation.as_ref(),
            "edu/wpi/first/math/geometry/Rotation3d",
            "getX",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap();

        let y = call!(
            rotation.as_ref(),
            "edu/wpi/first/math/geometry/Rotation3d",
            "getY",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap();

        let z = call!(
            rotation.as_ref(),
            "edu/wpi/first/math/geometry/Rotation3d",
            "getZ",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap();

        Vector3::new(x, y, z)
    }

    pub fn reset(&self) {
        call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/Pigeon2",
            "reset",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        );
    }

    pub fn set_yaw(&self, yaw: f64) -> bool {

        call!(
            self.instance.as_obj(),
            "com/ctre/phoenix6/hardware/core/CorePigeon2",
            "setYaw",
            "(D)Z",
            &[JValue::Double(yaw).as_jni()],
            ReturnType::Primitive(Primitive::Boolean)
        ).z().unwrap()
    }
}
