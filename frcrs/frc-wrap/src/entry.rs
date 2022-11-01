pub trait Entry {
    fn init();
}

#[cfg(feature = "java")]
pub mod entry {
    use j4rs::prelude::*;
    use lazy_static::lazy_static;
    // This is a hack, it is intended to make the 
    // transition to c++ bindings smoother
    pub struct FrcJvm { 
        jvm: j4rs::Jvm
    }
    unsafe impl Sync for FrcJvm {}
    
    //impl FrcJvm {
    //    unsafe fn take(&mut self) -> j4rs::Jvm {
    //        let jvm = replace(&mut self.jvm, None);
    //        jvm.unwrap()
    //    }
    //}
    //pub static mut FRC_JVM: FrcJvm = FrcJvm { jvm: None };
    lazy_static! {
        /// global JVM instance
        static ref FRC_JVM: FrcJvm =  FrcJvm{jvm:Jvm::attach_thread().unwrap()};
    }


    /// Binds global JVM instance, call before any JNI calls
    //pub fn init() -> Result<(), Box<dyn Error>>{
    //    let mut jvm = FRC_JVM.0.lock()?;
    //    *jvm = Some(FrcJvm(Jvm::attach_thread()?));
    //    Ok(())
    //}

    use super::Entry;
    pub use j4rs_derive::call_from_java as entrypoint;

    pub struct Java {  }
    impl Entry for Java {
        /// Call before anything else
        fn init() {
            //init().unwrap(); // setup JNI
            
            let jvm = &FRC_JVM.jvm;

            // ping driver's station
            jvm.invoke_static("edu.wpi.first.hal.HAL", "observeUserProgramStarting", &Vec::new()).unwrap();
        }
    }
}
