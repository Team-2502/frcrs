use crate::error::{Error, HalError, Result};
use hal_sys::{HAL_GetJoystickAxes, HAL_GetJoystickButtons, HAL_JoystickAxes, HAL_JoystickButtons};

#[derive(PartialEq, Clone)]
pub struct Joystick {
    id: u32,
}

impl Joystick {
    pub fn new(id: u32) -> Result<Self> {
        if id > 16 {
            Err(Error::JoystickIndexOutOfRange(id))
        } else {
            Ok(Self { id })
        }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_button_data(&self) -> Result<HAL_JoystickButtons> {
        let mut buttons = HAL_JoystickButtons {
            buttons: 0,
            count: 0,
        };

        let status = unsafe { HAL_GetJoystickButtons(self.id as i32, &mut buttons) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(buttons)
    }

    pub fn get_axes_data(&self) -> Result<HAL_JoystickAxes> {
        let mut axes = HAL_JoystickAxes {
            count: 0,
            axes: [0.; 12],
            raw: [0; 12],
        };

        let status = unsafe { HAL_GetJoystickAxes(self.id as i32, &mut axes) };

        if status != 0 {
            return Err(HalError(status).into());
        }

        Ok(axes)
    }

    // TODO: test all on hardware & err handling
    pub fn get_x(&self) -> Result<f32> {
        Ok(self.get_axes_data().unwrap().axes[0usize])
    }

    pub fn get_y(&self) -> Result<f32> {
        Ok(self.get_axes_data().unwrap().axes[1usize])
    }

    pub fn get_z(&self) -> Result<f32> {
        Ok(self.get_axes_data().unwrap().axes[2usize])
    }

    pub fn get_button(&self, index: u32) -> Result<bool> {
        Ok(self.get_button_data().unwrap().buttons & (1 >> index) > 0)
    }
}
