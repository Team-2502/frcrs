macro_rules! call_static {
    ($path:tt, $method:tt, $sig:tt, $args:expr, $ret:expr) => {{
        use crate::java;
        use jni::objects::{JClass, JStaticMethodID};
        use once_cell::sync::OnceCell;
        static FNPTR: OnceCell<JStaticMethodID> = OnceCell::new();
        static CLASS: OnceCell<JClass> = OnceCell::new();
        let mut java = java();
        let fnptr = FNPTR.get_or_init(|| java.get_static_method_id($path, $method, $sig).unwrap());
        let class = CLASS.get_or_init(|| java.find_class($path).unwrap());

        unsafe {
            java.call_static_method_unchecked(class, fnptr, $ret, $args)
                .unwrap()
        }
    }};
}

macro_rules! call {
    ($obj:expr, $path:tt, $method:tt, $sig:tt, $args:expr, $ret:expr) => {{
        use crate::java;
        use jni::objects::JMethodID;
        use once_cell::sync::OnceCell;
        static FNPTR: OnceCell<JMethodID> = OnceCell::new();
        let mut java = java();
        let fnptr = FNPTR.get_or_init(|| {
            let class = java.find_class($path).unwrap();
            java.get_method_id(class, $method, $sig).unwrap()
        });

        unsafe {
            java.call_method_unchecked($obj, fnptr, $ret, $args)
                .unwrap()
        }
    }};
}

// this one only offers a performance benefit if you construct in a loop,
// the intent is just to homogenize the api
macro_rules! create {
    ($path:tt, $sig:tt, $args:expr) => {{
        use crate::java;
        use jni::objects::{JClass, JMethodID};
        use once_cell::sync::OnceCell;
        static FNPTR: OnceCell<JMethodID> = OnceCell::new();
        static CLASS: OnceCell<JClass> = OnceCell::new();
        let mut java = java();
        let class = CLASS.get_or_init(|| java.find_class($path).unwrap());
        let fnptr = FNPTR.get_or_init(|| java.get_method_id(class, "<init>", $sig).unwrap());

        let obj = unsafe { java.new_object_unchecked(class, *fnptr, $args).unwrap() };
        java.new_global_ref(obj).unwrap()
    }};
}

macro_rules! once {
    ($code:expr) => {{
        static ONCE: OnceCell<JObject> = OnceCell::new();

        ONCE.get_or_init(|| $code)
    }};
}

pub(crate) use {call, call_static, create, once};
