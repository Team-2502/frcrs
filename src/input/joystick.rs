use std::time::Instant;

use hal_sys::{HAL_JoystickAxes, HAL_JoystickButtons};


use bitvec::prelude::*;

enum AxisType { X, Y, Z, Rx, Ry, Rz, Slider, POV, None }


pub struct Joystick {
    id: i32,
    buttons: BitVec,
    axes: HAL_JoystickAxes,
    last_updated: Instant,
}

impl Joystick {
    pub fn new(id: i32) -> Self {
        let buttons = bitvec![0; 32];
        let last_updated = Instant::now();

        let axes = HAL_JoystickAxes {
            count: 0,
            axes: [0.;12],
            raw: [0;12],
        };

        Self { id, buttons, axes, last_updated }
    }

    fn refresh(&mut self) {
        if self.last_updated.elapsed().as_millis() < 10 {
            return;
        }

        let mut buttons = HAL_JoystickButtons {
            buttons: 0,
            count: 0,
        };

        let _ret = unsafe {
            hal_sys::HAL_GetJoystickButtons(self.id, &mut buttons)
        };

        let _ret = unsafe {
            hal_sys::HAL_GetJoystickAxes(self.id, &mut self.axes);
        };

        self.buttons[..].store(buttons.buttons);
        self.last_updated = Instant::now();
    }

    pub fn get_x(&mut self) -> f32 {
        self.refresh();

        self.axes.axes[AxisType::X as usize]
    }

    pub fn get_y(&mut self) -> f32 {
        self.refresh();

        self.axes.axes[AxisType::Y as usize]
    }

    pub fn get_z(&mut self) -> f32 {
        self.refresh();

        self.axes.axes[AxisType::Z as usize]
    }

    pub fn get_throttle(&mut self) -> f32 {
        self.refresh();

        -self.axes.axes[AxisType::Slider as usize]
    }

    pub fn get(&mut self, id: usize) -> bool {
        self.refresh();

        self.buttons[id - 1]
    }
}
