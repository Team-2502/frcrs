pub trait Entry {
    fn hal_start();
}

#[cfg(feature = "java")]
pub mod entry {
    use j4rs::prelude::*;
    use lazy_static::lazy_static;
    pub struct FrcJvm { jvm: j4rs::Jvm }
    unsafe impl Sync for FrcJvm {}
    
    // This is a hack, it is intended to make the 
    // transition to c++ bindings smoother
    lazy_static! {
        /// global JVM instance
        static ref FRC_JVM: FrcJvm =  FrcJvm{ jvm: Jvm::attach_thread().unwrap() };
    }

    use super::Entry;
    pub use j4rs_derive::call_from_java as entrypoint;

    pub struct Java {  }
    impl Entry for Java {
        /// Call before anything else
        fn hal_start() {
            let jvm = &FRC_JVM.jvm;

            // ping driver's station
            jvm.invoke_static("edu.wpi.first.hal.HAL", "observeUserProgramStarting", &Vec::new()).unwrap();
        }
    }
}
