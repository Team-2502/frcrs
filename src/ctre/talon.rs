

use j4rs::{Instance, InvocationArg, Jvm};
use uom::si::f64::Angle;
use crate::ctre::TalonInvertType;

use super::talon_encoder_tick;

use ctre_sys::{self, talonfx_wrapper_follow, talonfx_wrapper_invert, talonfx_wrapper_get_velocity, talonfx_wrapper_stop, talonfx_wrapper_get_position, talonfx_wrapper_play_tone};

pub struct Kraken {
    can_id: i32,
    motor: *mut ctre_sys::ctre_phoenix6_hardware_TalonFX,
}

pub enum ControlMode {
    Percent,
    Velocity,
    Position
}

impl Kraken {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {

        let motor = unsafe {
            if let Some(bus) = can_loop {
                ctre_sys::talonfx_wrapper_bind_talon_with_bus(can_id, bus.into_bytes().as_mut_ptr())
            } else {
                ctre_sys::talonfx_wrapper_bind_talon(can_id)
                
            }
        };

        Self { can_id, motor }
    }

    pub fn set(&mut self, amount: f64) {
        unsafe {
            ctre_sys::talonfx_wrapper_set_speed(self.motor,amount);
        }
    }

    pub fn follow(&self, master: Kraken, reverse: bool) {
        unsafe { 
            talonfx_wrapper_follow(self.motor, master.can_id, reverse) 
        }
    }

    pub fn set_inverted(&self, inverted: bool) {
        unsafe {
            talonfx_wrapper_invert(self.motor, inverted);
        }
    }

    /// rotations per second
    pub fn get_speed(&self) -> f64 {
        unsafe {
            talonfx_wrapper_get_velocity(self.motor)
        }
    }

    pub fn stop(&self) {
        unsafe {
            talonfx_wrapper_stop(self.motor);
        }
    }

    /// rotations
    pub fn get_position(&self) -> f64 {
        unsafe {
            talonfx_wrapper_get_position(self.motor)
        }
    }

    /// rotations per second
    pub fn get_velocity(&self) -> f64 {
        unsafe {
            talonfx_wrapper_get_velocity(self.motor)
        }
    }

    pub fn play_tone(&self, hertz: f64) {
        unsafe {
            talonfx_wrapper_play_tone(self.motor, hertz)
        }
    }
}

pub struct Falcon {
    can_id: i32,
    pub(crate) instance: Instance
}

impl Falcon {
    pub fn new(can_id: i32, can_loop: Option<String>) -> Self {
        let jvm = Jvm::attach_thread().unwrap();

        let instance = jvm.create_instance(
            "com.ctre.phoenix.motorcontrol.can.TalonFX",
            &[InvocationArg::try_from(can_id)
                .unwrap().into_primitive().unwrap(),
                InvocationArg::try_from(can_loop.unwrap_or("rio".to_owned())).unwrap()
            ]).unwrap();

        Self { can_id, instance }
    }

    pub fn set(&self, control_mode: ControlMode, amount: f64) {
        set(&self.instance, control_mode, amount)
    }

    pub fn follow(&self, master: Falcon) {
        follow(&self.instance, master.instance)
    }

    pub fn set_inverted(&self, talon_invert_type: TalonInvertType) {
        set_inverted(&self.instance, talon_invert_type)
    }

    pub fn stop(&self) {
        stop(&self.instance)
    }

    pub fn get(&self) -> Angle {
        get(&self.instance)
    }
}

fn set(instance: &Instance, control_mode: ControlMode, amount: f64) {
    let jvm = Jvm::attach_thread().unwrap();

    match control_mode {
        ControlMode::Percent => {
            let control = jvm.invoke_static(
                "frc.robot.Wrapper",
                "ctrePercent",
                &Vec::new()).unwrap();

            jvm.invoke(
                instance,
                "set",
                &[
                    InvocationArg::from(control),
                    InvocationArg::try_from(amount)
                    .unwrap()
                    .into_primitive()
                    .unwrap()],
            ).unwrap();
        }
        ControlMode::Velocity => {
            let control = jvm.invoke_static(
                "frc.robot.Wrapper",
                "ctreVelocity",
                &Vec::new()
            ).unwrap();

            jvm.invoke(
                instance,
                "set",
                &[
                    InvocationArg::from(control),
                    InvocationArg::try_from(amount)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
            ).unwrap();
        }
        ControlMode::Position => {
            let control = jvm.invoke_static(
                "frc.robot.Wrapper",
                "ctrePosition",
                &Vec::new()
            ).unwrap();

            jvm.invoke(
                instance,
                "set",
                &[
                    InvocationArg::from(control),
                    InvocationArg::try_from(amount)
                        .unwrap()
                        .into_primitive()
                        .unwrap()],
            ).unwrap();
        }
    }
}

fn follow(instance: &Instance, master: Instance) {
    let jvm = Jvm::attach_thread().unwrap();

    jvm.invoke(
        instance,
        "follow",
        &[InvocationArg::try_from(master)
            .unwrap()
            .into_primitive()
            .unwrap()],
    ).unwrap();
}

fn set_inverted(instance: &Instance, talon_invert_type: TalonInvertType) {
    let jvm = Jvm::attach_thread().unwrap();

    let invert_type = jvm
        .invoke_static("frc.robot.Wrapper", talon_invert_type.as_str(), &Vec::new())
        .unwrap();

    jvm.invoke(
        instance,
        "setInverted",
        &[
            InvocationArg::try_from(invert_type).unwrap(),
        ],
    )
        .unwrap();
}

fn stop(instance: &Instance) {
    let jvm = Jvm::attach_thread().unwrap();
    jvm.invoke(
        instance,
        "stopMotor",
        &Vec::new(),
    ).unwrap();
}

fn get(instance: &Instance) -> Angle {
    let jvm = Jvm::attach_thread().unwrap();

    let returned: f64 = jvm.to_rust(jvm.invoke(
        instance,
        "getSelectedSensorPosition",
        &Vec::new(),
    ).unwrap()).unwrap();

    Angle::new::<talon_encoder_tick>(returned)
}

pub fn get_speed(instance: &Instance) -> f64 {
    let jvm = Jvm::attach_thread().unwrap();

    let returned: f64 = jvm.to_rust(jvm.invoke(
        instance,
        "get",
        &Vec::new(),
    ).unwrap()).unwrap();

    returned
}
