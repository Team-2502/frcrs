use crate::call::{call, call_static};
use crate::java;
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};

pub struct NetworkTable {
    instance: GlobalRef,
}

impl NetworkTable {
    pub fn init() {
        call_static!(
            "frc/robot/Wrapper",
            "startNetworkTables",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        )
        .v()
        .unwrap()
    }

    pub fn get_table(name: &str) -> Self {
        let instance = call_static!(
            "edu/wpi/first/networktables/NetworkTableInstance",
            "getDefault",
            "()Ledu/wpi/first/networktables/NetworkTableInstance;",
            &Vec::new(),
            ReturnType::Object
        )
        .l()
        .unwrap();

        let java_str = java().new_string(name).unwrap();
        let java_obj: JObject = java_str.into();

        let table = call!(
            &instance,
            "edu/wpi/first/networktables/NetworkTableInstance",
            "getTable",
            "Ljava/lang/String;)Ledu/wpi/first/networktables/NetworkTable",
            &[JValue::Object(&java_obj).as_jni()],
            ReturnType::Object
        )
        .l()
        .unwrap();

        Self {
            instance: java().new_global_ref(table).unwrap(),
        }
    }

    pub fn get_entry(&self, name: &str) -> NetworkTableEntry {
        let java_str = java().new_string(name).unwrap();
        let java_obj: JObject = java_str.into();

        let instance = call!(
            &self.instance,
            "edu/wpi/first/networktables/NetworkTable",
            "getEntry",
            "(Ljava/lang/String;)Ledu/wpi/first/networktables/NetworkTableEntry;",
            &[JValue::Object(&java_obj).as_jni()],
            ReturnType::Object
        )
        .l()
        .unwrap();

        NetworkTableEntry {
            instance: java().new_global_ref(instance).unwrap(),
        }
    }
}

pub struct NetworkTableEntry {
    instance: GlobalRef,
}

impl NetworkTableEntry {
    pub fn set_float(&self, value: f64) {
        call!(
            &self.instance,
            "edu/wpi/first/networktables/NetworkTableEntry",
            "setDouble",
            "(D)V",
            &[JValue::Double(value).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        )
        .v()
        .unwrap()
    }

    pub fn set_string(&self, value: &str) {
        let java_str = java().new_string(value).unwrap();
        let java_obj: JObject = java_str.into();

        call!(
            &self.instance,
            "edu/wpi/first/networktables/NetworkTableEntry",
            "setString",
            "(Ljava/lang/String;)V",
            &[JValue::Object(&java_obj).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        )
        .v()
        .unwrap()
    }

    pub fn get_float(&self) -> f64 {
        call!(
            &self.instance,
            "edu/wpi/first/networktables/NetworkTableEntry",
            "getDouble",
            "(D)D",
            &[JValue::Double(0.).as_jni()],
            ReturnType::Primitive(Primitive::Double)
        )
        .d()
        .unwrap()
    }

    pub fn get_string(&self) -> String {
        let default_str = java().new_string("").unwrap();
        let default_obj: JObject = default_str.into();

        let result = call!(
            &self.instance,
            "edu/wpi/first/networktables/NetworkTableEntry",
            "getString",
            "(Ljava/lang/String;)Ljava/lang/String;",
            &[JValue::Object(&default_obj).as_jni()],
            ReturnType::Object
        )
        .l()
        .unwrap();

        java().get_string((&result).into()).unwrap().into()
    }
}
