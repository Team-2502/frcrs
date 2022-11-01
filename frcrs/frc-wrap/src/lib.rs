pub mod entry;

pub use entry::entry::entrypoint;



#[cfg(feature = "java")]
mod frc_wrap {
    use std::sync::Mutex;
    use j4rs::prelude::*;
    use std::error::Error;

    // This is a hack, it is intended to make the 
    // transition to c++ bindings smoother
    pub struct FrcJvm(j4rs::Jvm);
    unsafe impl Send for FrcJvm {}
    /// global JVM instance
    pub static FRC_JVM: Mutex<Option<FrcJvm>> = Mutex::new(None);

    pub fn init() -> Result<(), Box<dyn Error>>{
        let mut jvm = FRC_JVM.lock()?;
        *jvm = Some(FrcJvm(Jvm::attach_thread()?));
        Ok(())
    }
}

pub use frc_wrap::*;
