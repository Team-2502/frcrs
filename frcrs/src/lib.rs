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
    jvm.invoke_static("edu.wpi.first.hal.HAL", "observeUserProgramStarting", &Vec::new()).unwrap();

    let ds = jvm.invoke_static("edu.wpi.first.wpilibj.DriverStation", "getInstance", &Vec::new()).unwrap();


    let mut now = SystemTime::now();
    let mut delta = 0;

    loop {
        //jvm.invoke_static(
        //    "edu.wpi.first.wpilibj.DriverStation", 
        //    "reportWarning", 
        //    &[InvocationArg::try_from("test warn").unwrap(),
        //    InvocationArg::try_from(false).unwrap()]);
        //jvm.invoke(&ds, "isTeleop",
        //    &Vec::new());


        jvm.invoke(&ds, "reportWarning",
            &[InvocationArg::try_from(format!("test warn, {} ms", &delta)).unwrap(),
            InvocationArg::try_from(false).unwrap().into_primitive().unwrap()]);

        
        delta = now.elapsed().unwrap().as_millis();
        now = SystemTime::now();

    }
}
