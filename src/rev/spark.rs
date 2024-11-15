use crate::call::{self, call, call_static, create, once};
use crate::java;
use crate::rev::{ControlType, IdleMode, MotorType, };
use jni::objects::{GlobalRef, JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use once_cell::sync::OnceCell;
use uom::si::angle;
use uom::si::angle::revolution;
use uom::si::f64::*;

pub struct Spark {
    can_id: i32,
    instance: GlobalRef,
    encoder: Option<GlobalRef>,
    pid: Option<GlobalRef>,
}

impl Spark {

    pub fn set_reference(&mut self,value: f64,control_type: ControlType) {
        self.set_reference_ff(value, control_type, 0.)
    }

    pub fn set_reference_ff(&mut self, value: f64, control_type: ControlType, feed_forward: f64) {
        let mut jvm = java();

        let mut control_type_java = |control_type: &ControlType| {
            jvm.call_static_method(
                "frc/robot/Wrapper",
                control_type.as_str(),
                "()Lcom/revrobotics/CANSparkBase$ControlType;",
                &Vec::new()
            ).unwrap().l().unwrap()
        };

        let control_type = match control_type {
            ControlType::Position => once!(control_type_java(&control_type)),
            ControlType::Velocity => once!(control_type_java(&control_type)),
        };

        let controller = self.get_controller();

        call!(controller,
            "com/revrobotics/SparkPIDController",
            "setReference",
            "(DLcom/revrobotics/CANSparkBase$ControlType;ID)Lcom/revrobotics/REVLibError;",
            &[JValue::Double(value).as_jni(),
              JValue::Object(&JObject::from_raw(control_type.as_raw())).as_jni(),
              JValue::Int(0).as_jni(), // PID 0, TODO: handle multiple
              JValue::Double(feed_forward).as_jni(),
            ],
            ReturnType::Object
        ).l().unwrap();
    }

    pub fn get_current(&self) -> f64 {
        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "getOutputCurrent",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub(crate) fn get_controller(&mut self) -> &JObject {
        if self.pid.is_some() {
            return &self.pid.as_ref().unwrap();
        }

        self.pid = Some(java().new_global_ref(call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "getPIDController",
            "()Lcom/revrobotics/SparkPIDController;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap()).unwrap());

        &self.pid.as_ref().unwrap()
    }

    fn get_encoder(&mut self) -> &JObject {
        if self.encoder.is_some() {
            return &self.encoder.as_ref().unwrap();
        }

        self.encoder = Some(java().new_global_ref(call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "getEncoder",
            "()Lcom/revrobotics/RelativeEncoder;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap()).unwrap());

        &self.encoder.as_ref().unwrap()

    }

    pub fn get_velocity(&mut self) -> f64 {
        let encoder = self.get_encoder();
        call!(
            encoder,
            "com/revrobotics/RelativeEncoder",
            "getVelocity",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    pub fn get_position(&mut self) -> Angle {
        let encoder = self.get_encoder();
        let rots: f64 = call!(
            encoder,
            "com/revrobotics/RelativeEncoder",
            "getPosition",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap();

        Angle::new::<revolution>(rots)
    }

    pub(crate) fn instance(&self) -> &JObject {
        self.instance.as_obj()
    }

    pub fn new(can_id: i32, motor_type: MotorType) -> Self {
        let mut jvm = java();
 
        let motortype = jvm.call_static_method("frc/robot/Wrapper", motor_type.as_str(), "()Lcom/revrobotics/CANSparkLowLevel$MotorType;", &Vec::new()).unwrap().l().unwrap();

        let instance = create!(
            "com/revrobotics/CANSparkMax",
            "(ILcom/revrobotics/CANSparkMaxLowLevel$MotorType;)V",
            &[JValue::Int(can_id).as_jni(),
              JValue::Object(&JObject::from_raw(motortype.into_raw())).as_jni()]
        );
        
        Self {
            can_id,
            instance,
            encoder: None,
            pid: None,
        }
    }

    pub fn flex(can_id: i32) -> Self {
        Self::new(can_id, MotorType::Brushless)
    }

    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    pub fn set(&self, amount: f64) {
        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "set",
            "(D)V",
            &[JValue::Double(amount).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        ).v().unwrap();
    }

    pub fn set_idle_mode(&self, idle_mode: IdleMode) {
        let mut jvm = java();

        let mode = jvm.call_static_method("frc/robot/Wrapper", idle_mode.as_str(), "()Lcom/revrobotics/CANSparkBase$IdleMode;", &Vec::new()).unwrap().l().unwrap();

        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "setIdleMode",
            "(Lcom/revrobotics/CANSparkBase$IdleMode;)Lcom/revrobotics/REVLibError",
            &[JValue::Object(&JObject::from_raw(mode.into_raw())).as_jni()],
            ReturnType::Object
        );
    }

    pub fn follow(&self, master: Spark, invert: bool) {
        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "follow",
            "Lcom/revrobotics/CANSparkBase;Z)Lcom/revrobotics/REVLibError;",
            &[
            JValue::Object(&JObject::from_raw(master.instance().as_raw())).as_jni(),
            JValue::Bool(invert as u8).as_jni()
            ],
            ReturnType::Object
        ).l().unwrap();
    }

    /// Stop the motor
    pub fn stop(&self) {
        call!(
            self.instance.as_obj(),
            "com/revrobotics/CANSparkBase",
            "stopMotor",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        ).v().unwrap();
    }

    pub fn set_position(&mut self, position: Angle) {
        self.set_reference(position.get::<angle::revolution>(), ControlType::Position);
    }

    #[deprecated]
    pub fn get_pid(&mut self) -> &mut Self {
        self
    }

    pub fn set_p(&mut self, p: f64) {
        call!(self.get_controller(),
            "com/revrobotics/SparkPIDController",
            "setP",
            "(D)Lcom/revrobotics/REVLibError;",
            &[JValue::Double(p).as_jni()],
            ReturnType::Object
        ).l().unwrap();
    }

    pub fn set_i(&mut self, i: f64) {
        call!(self.get_controller(),
            "com/revrobotics/SparkPIDController",
            "setI",
            "(D)Lcom/revrobotics/REVLibError;",
            &[JValue::Double(i).as_jni()],
            ReturnType::Object
        ).l().unwrap();
    }

    pub fn set_d(&mut self, d: f64) {
        call!(self.get_controller(),
            "com/revrobotics/SparkPIDController",
            "setD",
            "(D)Lcom/revrobotics/REVLibError;",
            &[JValue::Double(d).as_jni()],
            ReturnType::Object
        ).l().unwrap();
    }

    pub fn set_ff(&mut self, ff: f64) {
        call!(self.get_controller(),
            "com/revrobotics/SparkPIDController",
            "setFF",
            "(D)Lcom/revrobotics/REVLibError;",
            &[JValue::Double(ff).as_jni()],
            ReturnType::Object
        ).l().unwrap();
    }
}
