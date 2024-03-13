use hal_sys::{HAL_GetDIO, HAL_InitializeDIOPort, HAL_PortHandle, HAL_Handle, HAL_FreeDIOPort};
use j4rs::{Jvm, InvocationArg, Instance};

pub struct DIO {
    handle: i32,
}

impl DIO {
    pub fn new(port: i32) -> Self { 
        let mut _ret = 0;
        let handle = unsafe {HAL_InitializeDIOPort(port, 1, std::ptr::null(), &mut _ret)};
        Self { handle } 
    }

    pub fn get(&self) -> bool {
        let mut _ret = 0;
        unsafe{HAL_GetDIO(self.handle, &mut _ret) == 1}
    }
}

impl Drop for DIO {
    fn drop(&mut self) {
        unsafe{HAL_FreeDIOPort(self.handle)}
    }
}
