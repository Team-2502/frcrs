use j4rs::{InvocationArg, Jvm};

pub struct SmartDashboard;

impl SmartDashboard {
    pub fn put_number(key: String, data: f64) {
        let jvm = Jvm::attach_thread().unwrap();

        jvm.invoke_static(
            "edu.wpi.first.wpilibj.smartdashboard.SmartDashboard",
            "putNumber",
            &[
                InvocationArg::try_from(key).unwrap(),
                InvocationArg::try_from(data).unwrap().into_primitive().unwrap(),
            ]
        )
        .unwrap();
    }
}
