use crate::call::{self, call, call_static, create};
use crate::java;
use crate::rev::{ControlType, IdleMode, MotorType, Spark, SparkPIDController};
use j4rs::{Instance, InvocationArg, Jvm};
use jni::objects::{JObject, JValue};
use jni::signature::{Primitive, ReturnType};
use once_cell::sync::OnceCell;
use uom::si::angle;
use uom::si::angle::revolution;
use uom::si::f64::*;

pub struct JavaSpark<'local> {
    can_id: i32,
    instance: JObject<'local>,
    encoder: Option<JObject<'local>>,
    pid: Option<JObject<'local>>,
}

pub struct SparkFlex<'local> {
    can_id: i32,
    instance: JObject<'local>,
    encoder: Option<JObject<'local>>,
}

pub trait SparkMax {
    fn new(can_id: i32, motor_type: MotorType) -> Self;
    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    fn set(&self, amount: f64);
    fn set_idle_mode(&self, idle_mode: IdleMode);
    fn get_pid(&self) -> SparkPIDController;
    fn follow(&self, master: Spark, invert: bool);
    fn stop(&self);
    fn set_position(&mut self, position: Angle);
}

impl<'local> JavaSpark<'local> {
    fn set_reference(&mut self, value: f64, control_type: ControlType) {
        let mut jvm = java();

        assert!(matches!(ControlType::Position, control_type));
        static POS: OnceCell<JObject>= OnceCell::new();

        let pos = POS.get_or_init(|| {
            jvm.call_static_method("frc/robot/Wrapper", control_type.as_str(), "()Lcom/revrobotics/CANSparkBase$ControlType;", &Vec::new()).unwrap().l().unwrap()
        });

        let controller = self.get_controller();

        call!(controller,
            "com/revrobotics/SparkPIDController",
            "setReference",
            "(DLcom/revrobotics/CANSparkBase$ControlType;)Lcom/revrobotics/REVLibError;",
            &[JValue::Double(value).as_jni(),
              JValue::Object(&JObject::from_raw(pos.as_raw())).as_jni()
            ],
            ReturnType::Object
        ).l().unwrap();
    }

    pub fn get_current(&self) -> f64 {
        call!(
            &self.instance,
            "com/revrobotics/CANSparkBase",
            "getOutputCurrent",
            "()D",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Double)
        ).d().unwrap()
    }

    fn get_controller(&mut self) -> &JObject {
        if self.pid.is_some() {
            return &self.encoder.as_ref().unwrap();
        }

        self.pid = Some(call!(
            &self.instance,
            "com/revrobotics/CANSparkBase",
            "getPIDController",
            "()Lcom/revrobotics/SparkPIDController;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap());

        &self.pid.as_ref().unwrap()
    }

    fn get_encoder(&mut self) -> &JObject {
        if self.encoder.is_some() {
            return &self.encoder.as_ref().unwrap();
        }

        self.encoder = Some(call!(
            &self.instance,
            "com/revrobotics/CANSparkBase",
            "getEncoder",
            "()Lcom/revrobotics/RelativeEncoder;",
            &Vec::new(),
            ReturnType::Object
        ).l().unwrap());

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
        &self.instance
    }
}

impl<'local> SparkMax for JavaSpark<'local> {
    fn new(can_id: i32, motor_type: MotorType) -> Self {
        let mut jvm = java();
 
        let motortype = jvm.call_static_method("frc/robot/Wrapper", motor_type.as_str(), "()Lcom/revrobotics/CANSparkLowLevel$MotorType", &Vec::new()).unwrap().l().unwrap();

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

    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    fn set(&self, amount: f64) {
        call!(
            &self.instance,
            "com/revrobotics/CANSparkBase",
            "set",
            "(D)V",
            &[JValue::Double(amount).as_jni()],
            ReturnType::Primitive(Primitive::Void)
        ).v().unwrap();
    }

    fn set_idle_mode(&self, idle_mode: IdleMode) {
        let mut jvm = java();

        let mode = jvm.call_static_method("frc/robot/Wrapper", idle_mode.as_str(), "()Lcom/revrobotics/CANSparkBase$IdleMode;", &Vec::new()).unwrap().l().unwrap();

        call!(
            &self.instance,
            "com/revrobotics/CANSparkBase",
            "setIdleMode",
            "(Lcom/revrobotics/CANSparkBase$IdleMode;)Lcom/revrobotics/REVLibError",
            &[JValue::Object(&JObject::from_raw(mode.into_raw())).as_jni()],
            ReturnType::Object
        );
    }

    fn get_pid(&self) -> SparkPIDController {
        // TODO: get values from spark
        SparkPIDController::from(
            &self,
            0.0,
            0.0,
            0.0,
        )
    }

    fn follow(&self, master: Spark, invert: bool) {
        call!(
            &self.instance,
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
    fn stop(&self) {
        call!(
            &self.instance,
            "com/revrobotics/CANSparkBase",
            "stopMotor",
            "()V",
            &Vec::new(),
            ReturnType::Primitive(Primitive::Void)
        ).v().unwrap();
    }

    fn set_position(&mut self, position: Angle) {
        self.set_reference(position.get::<angle::revolution>(), ControlType::Position);
    }
}

impl<'local> SparkFlex<'local> {
    pub(crate) fn instance(&self) -> &JObject {
        &self.instance
    }

    fn set_reference(&self, value: f64, control_type: ControlType) {
        let jvm = Jvm::attach_thread().unwrap();

        let _control_type = jvm
            .invoke_static("frc.robot.Wrapper", control_type.as_str(), &Vec::new())
            .unwrap();

        jvm.invoke(
            &self.get_pid().instance(),
            "setReference",
            &[
                InvocationArg::try_from(value)
                    .unwrap()
                    .into_primitive()
                    .unwrap(),
                InvocationArg::try_from(_control_type).unwrap(),
            ],
        )
            .unwrap();
    }

    fn get_encoder(&mut self) -> &Instance {
        if self.encoder.is_some() {
            return &self.encoder.as_ref().unwrap();
        }

        let jvm = Jvm::attach_thread().unwrap();

        self.encoder = Some(
            jvm.invoke(&self.instance, "getEncoder", &Vec::new()).unwrap(),
        );

        &self.encoder.as_ref().unwrap()

    }

    pub fn get_velocity(&mut self) -> f64 {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.to_rust(jvm.invoke(
            self.get_encoder(),
            "getVelocity",
            &[
            ],
        ).unwrap()).unwrap()
    }
}

impl SparkMax for SparkFlex {
    fn new(can_id: i32, motor_type: MotorType) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let motortype = jvm
            .invoke_static("frc.robot.Wrapper", motor_type.as_str(), &Vec::new())
            .unwrap();

        /*let instance = jvm
            .create_instance(
                "com.revrobotics.CANSparkFlex",
                &[
                    InvocationArg::try_from(can_id)
                        .unwrap()
                        .into_primitive()
                        .unwrap(),
                    InvocationArg::try_from(motortype).unwrap(),
                ],
            )
            .unwrap();*/

        let instance = jvm.invoke_static("frc.robot.Wrapper", "createSparkFlex", &[
            InvocationArg::try_from(can_id).unwrap().into_primitive().unwrap()
        ]).unwrap();

        Self {
            can_id,
            motor_type,
            instance,
            encoder: None
        }
    }

    /// Set the speed of the motor
    ///
    /// `amount` is from -1, 1
    fn set(&self, amount: f64) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(
            &self.instance,
            "set",
            &[InvocationArg::try_from(amount)
                .unwrap()
                .into_primitive()
                .unwrap()],
        )
            .unwrap(); //.expect("Failed to call `set` on motor");
    }

    fn set_idle_mode(&self, idle_mode: IdleMode) {
        let jvm = Jvm::attach_thread().unwrap();

        let mode = jvm
            .invoke_static("frc.robot.Wrapper", idle_mode.as_str(), &Vec::new())
            .unwrap();

        jvm.invoke(
            &self.instance,
            "setIdleMode",
            &[InvocationArg::try_from(mode).unwrap()],
        )
            .unwrap();
    }

    fn get_pid(&self) -> SparkPIDController {
        let jvm = Jvm::attach_thread().unwrap();

        // TODO: get values from spark
        SparkPIDController::from(
            &self.instance,
            jvm.invoke(&self.instance, "getPIDController", &Vec::new())
                .unwrap(),
            0.0,
            0.0,
            0.0,
        )
    }

    fn follow(&self, master: Spark, invert: bool) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke(&self.instance, "follow", &[
            InvocationArg::from(master.instance),
            InvocationArg::try_from(invert).unwrap().into_primitive().unwrap()
        ]).unwrap();
    }

    /// Stop the motor
    fn stop(&self) {
        let jvm = Jvm::attach_thread().unwrap();
        jvm.invoke(&self.instance, "stopMotor", &Vec::new())
            .unwrap();
    }

    fn set_position(&self, position: Angle) {
        self.set_reference(position.get::<angle::revolution>(), ControlType::Position);
    }
}
