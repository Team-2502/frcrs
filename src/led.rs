use jni::{objects::{GlobalRef, JObject, JValue}, signature::{Primitive, ReturnType}};

#[derive(Clone)]
pub struct Led {
    instance: GlobalRef,
    buffer: GlobalRef
}

impl Led {
    pub fn new(port: i32, count: i32) -> Self {
        let instance = create!(
            "edu/wpi/first/wpilibj/AddressableLED",
            "(I)V",
            &[JValue::Int(port).as_jni()]
        );

        call!(
            instance.clone(),
            "edu/wpi/first/wpilibj/AddressableLED",
            "setLength",
            "(I)V",
            &[JValue::Int(count).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        );
        
        let buffer = create!(
            "edu/wpi/first/wpilibj/AddressableLEDBuffer",
            "(I)V",
            &[JValue::Int(count).as_jni()]
        );

        let obj = buffer.as_obj();

         call!(
            &instance,
            "edu/wpi/first/wpilibj/AddressableLED",
            "setData",
            "(Ledu/wpi/first/wpilibj/AddressableLEDBuffer;)V",
            &[JValue::Object(obj).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        );

        call!(
            &instance,
            "edu/wpi/first/wpilibj/AddressableLED",
            "start",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        );


        Self {
            instance,
            buffer
        }
    }

    pub fn set_rgb(&self, idx: i32, r: i32, g: i32, b: i32) {
        call!(
            &self.buffer,
            "edu/wpi/first/wpilibj/AddressableLEDBuffer",
            "setRGB",
            "(IIII)V",
            &[JValue::Int(idx).as_jni(), JValue::Int(r).as_jni(), JValue::Int(g).as_jni(), JValue::Int(b).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        );
    }

    pub fn set_data(&self) {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/AddressableLED",
            "setData",
            "(Ledu/wpi/first/wpilibj/AddressableLEDBuffer;)V",
            &[JValue::Object(self.buffer.as_obj()).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        );
    }

    pub fn start(&self) {
        call!(
            &self.instance,
            "edu/wpi/first/wpilibj/AddressableLED",
            "start",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        );
    }

    pub fn flush(&self) {
        self.set_data();
        self.start();
    }
}

