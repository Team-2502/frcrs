use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::Primitive::{Int, Void};
use jni::signature::ReturnType;
use uom::si::{f64::Angle, angle::radian};
use nalgebra::Vector2;
use crate::call::{call, call_static};
use crate::java;

pub struct SmartDashboard;

impl SmartDashboard {
    pub fn put_number(key: String, data: f64) {
        let key = java().new_string(key).unwrap();

        call_static!(
            "edu/wpi/first/wpilibj/smartdashboard/SmartDashboard",
            "putNumber",
            "(Ljava/lang/String;D)V",
            &[JValue::Object(&JObject::from_raw(key.into_raw())).as_jni(), JValue::Double(data).as_jni()],
            ReturnType::Primitive(Void)
        ).v().unwrap()
    }

    pub fn put_bool(key: String, data: bool) {
        let key = java().new_string(key).unwrap();

        call_static!(
            "edu/wpi/first/wpilibj/smartdashboard/SmartDashboard",
            "putBoolean",
            "(Ljava/lang/String;Z)V",
            &[JValue::Object(&JObject::from_raw(key.into_raw())).as_jni(), JValue::Bool(data).as_jni()],
            ReturnType::Primitive(Void)
        ).v().unwrap()
    }
}

pub struct Chooser<T> {
    options: Vec<T>,
    instance: GlobalRef,
}

impl<T> Chooser<T> {
    pub fn new() -> Self {
        let instance = call_static!(
            "frc/robot/Wrapper",
            "createIntegerSendableChooser",
            "()Ledu/wpi/first/wpilibj/smartdashboard/SendableChooser;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap();

        Self {
            options: Vec::new(),
            instance: java().new_global_ref(instance).unwrap()
        }
    }

    pub fn add(&mut self, name: &str, option: T) {
        self.options.push(option);
        let idx = self.options.len();

        let string = java().new_string(name).unwrap();

        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/smartdashboard/SendableChooser",
            "addOption",
            "(Ljava/lang/String;Ljava/lang/Object;)V",
            &[JValue::Object(&JObject::from_raw(string.into_raw())).as_jni(),
                JValue::Int(idx as i32).as_jni()],
            ReturnType::Primitive(Void)
        ).v().unwrap();
    }

    pub fn get(&self) -> i32 {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/smartdashboard/SendableChooser",
            "getSelected",
            "()Ljava/lang/Object;",
            &Vec::new(),
            ReturnType::Primitive(Int)
        ).i().unwrap()
    }

}

/*
pub fn set_position(position: Vector2<f64>, angle: Angle) {
    let jvm = Jvm::attach_thread().unwrap();

    let angle = angle.get::<radian>();

    jvm.invoke_static(
        "frc.robot.Wrapper",
        "setPosition",
        &[
            InvocationArg::try_from(position.x).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(position.y).unwrap().into_primitive().unwrap(),
            InvocationArg::try_from(angle).unwrap().into_primitive().unwrap(),
        ]
    )
    .unwrap();
}
*/