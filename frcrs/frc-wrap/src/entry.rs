pub trait Entry {
    fn init();
}

#[cfg(feature = "java")]
pub mod entry {
    use std::sync::Mutex;
    use j4rs::prelude::*;
    use std::error::Error;
    // This is a hack, it is intended to make the 
    // transition to c++ bindings smoother
    pub struct FrcJvm(j4rs::Jvm);
    unsafe impl Send for FrcJvm {}
    pub struct FrcJvmWrapper(Mutex<Option<FrcJvm>>);
    /// global JVM instance
    pub static FRC_JVM: FrcJvmWrapper = FrcJvmWrapper(Mutex::new(None));
    macro_rules! frc_jvm {
        () => {
            {
                &FRC_JVM.0.lock().unwrap().as_ref()
            }
            
        };
    }

    /// Binds global JVM instance, call before any JNI calls
    pub fn init() -> Result<(), Box<dyn Error>>{
        let mut jvm = FRC_JVM.0.lock()?;
        *jvm = Some(FrcJvm(Jvm::attach_thread()?));
        Ok(())
    }

    use super::Entry;
    pub use j4rs_derive::call_from_java as entrypoint;

    pub struct Java {  }
    impl Entry for Java {
        /// Call before anything else
        fn init() {
            init().unwrap(); // setup JNI
            
            let mut jvm = FRC_JVM.0.lock().unwrap();
            let jvm = (jvm.unwrap()).0;

            jvm.static_class("");
            //let jvm2 = &FRC_JVM.0.lock().unwrap();
            //let jvm2 = jvm2.as_ref().unwrap().0;
            //let jvm2 = jvm2.unwrap().0;


            // ping driver's station
            //FRC_JVM.invoke_static("edu.wpi.first.hal.HAL", "observeUserProgramStarting", &Vec::new()).unwrap();
        }
    }
}
