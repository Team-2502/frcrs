pub mod rev;
pub mod robot;

use j4rs_derive::call_from_java;

use std::convert::TryFrom;
use j4rs::InvocationArg;
use j4rs::prelude::*;
use crate::rev::{MotorType, Spark};

//static JVM: Rc<Mutex<Option<Jvm>>> = Rc::new(Mutex::new(None));
/*
#[call_from_java("frc.robot.Main.rustentry")]
pub fn entrypoint() {
    let jvm = Jvm::attach_thread().unwrap();
    //JVM.get_or_init(|| Mutex::new(Jvm::attach_thread().unwrap()));

    // Show "robot code" on driver's station
    //let ds = initialize_driverstation();

    jvm.invoke_static(
        "edu.wpi.first.hal.HAL",
        "observeUserProgramStarting",
        &Vec::new(),
    )
        .unwrap();

    let ds = jvm
        .invoke_static(
            "edu.wpi.first.wpilibj.DriverStation",
            "getInstance",
            &Vec::new(),
        )
        .unwrap();

    let spark = Spark::new(1, MotorType::Brushless);

    loop {
        let teleop: bool = jvm
            .to_rust(jvm.invoke(&ds, "isTeleop", &Vec::new()).unwrap())
            .unwrap();

        match teleop {
            true => {
                spark.set(0.25);
            }
            false => {
                spark.stop();
            }
        };

        //jvm.invoke(&ds, "reportWarning",
        //    &[InvocationArg::try_from(format!("test warn")).unwrap(),
        //    InvocationArg::try_from(false).unwrap().into_primitive().unwrap()]);
    }
}

fn initialize_driverstation() -> Instance {
    let jvm = Jvm::attach_thread().unwrap();

    jvm.invoke_static(
        "edu.wpi.first.hal.HAL",
        "observeUserProgramStarting",
        &Vec::new(),
    )
        .unwrap();

    jvm
        .invoke_static(
            "edu.wpi.first.wpilibj.DriverStation",
            "getInstance",
            &Vec::new(),
        )
        .unwrap()
}*/

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() { // called on rio boot
    let jvm = Jvm::attach_thread().unwrap();

    //jvm.invoke_static(class_name, method_name, inv_args)

    // Show "robot code" on driver's station
    jvm.invoke_static("edu.wpi.first.hal.DriverStationJNI", "observeUserProgramStarting", &Vec::new()).unwrap();

    // init hal
    //jvm.invoke_static("edu.wpi.first.hal.HAL", "initialize", &[InvocationArg::try_from(500).unwrap()]).unwrap();
    //jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "refreshData", &Vec::new()).unwrap();

    //let ds = jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "getInstance", &Vec::new()).unwrap();

    //let k_brushless = jvm.static_class_field("com.revrobotics.CANSparkMaxLowLevel.MotorType", "kBrushless").unwrap();
    /*let k_brushless = jvm.invoke_static("com.revrobotics.CANSparkMaxLowLevel.MotorType", "fromId", &[
        InvocationArg::try_from(1).unwrap().into_primitive().unwrap()
    ]).unwrap();

    //let k_brushless = jvm.invoke_static("frc.robot.Wrapper", "getBrushless", &Vec::new()).unwrap();
    */
    let k_brushless = jvm.invoke_static("frc.robot.Wrapper", "getBrushless", &Vec::new()).unwrap();

    let motor = jvm.create_instance("com.revrobotics.CANSparkMax", &[
        InvocationArg::try_from(5).unwrap().into_primitive().unwrap(),
        InvocationArg::try_from(k_brushless).unwrap()
    ]).unwrap();

    loop {
        //let teleop: bool = jvm.to_rust(jvm.invoke(&ds, "isTeleop", &Vec::new()).unwrap()).unwrap();
        let teleop: bool = jvm.to_rust(jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "isTeleop", &Vec::new()).unwrap()).unwrap();

        match teleop {
            true => {
                /*jvm.invoke_static(" edu.wpi.first.wpilibj.DriverStation", "reportWarning",
                                  &[InvocationArg::try_from(format!("test warn")).unwrap(),
                                      InvocationArg::try_from(false).unwrap().into_primitive().unwrap()]).expect("TODO: panic message");*/
                jvm.invoke(&motor, "set", &[
                    InvocationArg::try_from(0.1).unwrap().into_primitive().unwrap(),
                ]).expect("TODO: panic message");
            }
            false => {
                jvm.invoke(&motor, "stopMotor", &Vec::new()).unwrap();
            }
        };
    }
}
