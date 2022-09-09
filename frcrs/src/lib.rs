use std::convert::TryFrom;
use std::result::Result;

use j4rs::InvocationArg;
use j4rs::prelude::*;
use j4rs_derive::*;
use serde::Deserialize;

#[call_from_java("frc.robot.Main.rustentry")]
fn entrypoint() { // called on rio boot
    let jvm = Jvm::attach_thread().unwrap();

    //jvm.invoke_static(class_name, method_name, inv_args)
    loop {}
}
