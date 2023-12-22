#![allow(async_fn_in_trait)]

use std::{
    cell::RefCell,
    io::Write,
    ops::DerefMut,
    pin::Pin,
    task::{Poll, Waker},
};

use hal_sys::{HAL_ObserveUserProgramDisabled, HAL_ObserveUserProgramTeleop, HAL_SendConsoleLine};
use tracing_subscriber::fmt::MakeWriter;

use hal_sys::HAL_ObserveUserProgramStarting;
use hal_sys::HAL_Initialize;

pub fn hal_initialize(timeout: i32, mode: i32) -> i32 {
    unsafe { HAL_Initialize(timeout, mode) }
}

pub fn observe_user_program_starting() {
    unsafe { HAL_ObserveUserProgramStarting() };
}

pub mod ds;
pub mod error;
pub mod joystick;

struct DsTracingWriter {}

impl<'a> MakeWriter<'a> for DsTracingWriter {
    type Writer = DsTracingWriter;

    fn make_writer(&'a self) -> Self::Writer {
        Self {}
    }

    fn make_writer_for(&'a self, _meta: &tracing::Metadata<'_>) -> Self::Writer {
        Self {}
    }
}

impl Write for DsTracingWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut data = buf.to_vec();

        data.push(0);

        let error_code = unsafe { HAL_SendConsoleLine(data[..].as_ptr()) };

        if error_code == 0 {
            Ok(buf.len())
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                crate::error::Error::HalError(error::HalError(error_code)),
            ))
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
