pub trait Entry {
    fn acknowledge_start();
}

#[cfg(feature = "java")]
pub mod entry {
    use crate::entry::Entry;
    pub use j4rs_derive::call_from_java as entrypoint;

    pub struct Java {  }
    impl Entry for Java {
        fn acknowledge_start() {
            
            
        }
    }
}
