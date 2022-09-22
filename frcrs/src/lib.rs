use std::convert::TryFrom;
use std::result::Result;

use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;

use std::time::SystemTime;

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() { // called on rio boot
    let jvm = Jvm::attach_thread().unwrap();

    //jvm.invoke_static(class_name, method_name, inv_args)

    // Show "robot code" on driver's station
    jvm.invoke_static("edu.wpi.first.hal.HAL", "observeUserProgramStarting", &Vec::new()).unwrap();

    let ds = jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "getInstance", &Vec::new()).unwrap();

    let motor = jvm.create_instance("com.revrobotics.CANSparkMax", &[
        InvocationArg::try_from(1).unwrap().into_primitive().unwrap(),
        InvocationArg::try_from({
                let motor_types = jvm.static_class("com.revrobotics.CANSparkMaxLowLevel.MotorType").unwrap();
                let k_brushless = jvm.field(&motor_types, "kBrushless").unwrap();
                k_brushless
            }).unwrap()
    ]).unwrap();

    loop {
        let teleop: bool = jvm.to_rust(jvm.invoke(&ds, "isTeleop",
            &Vec::new()).unwrap()).unwrap();


        match teleop {
            true => {
                jvm.invoke(&motor, "set", &[
                    InvocationArg::try_from(0.5).unwrap().into_primitive().unwrap(),
                ]);
            }
            false => {
                jvm.invoke(&motor, "stopMotor", &Vec::new()).unwrap();
            }
        };


        //jvm.invoke(&ds, "reportWarning",
        //    &[InvocationArg::try_from(format!("test warn")).unwrap(),
        //    InvocationArg::try_from(false).unwrap().into_primitive().unwrap()]);

        
    }
}
