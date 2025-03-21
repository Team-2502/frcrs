mod talon;
mod cancoder;
mod srx;
mod pigeon;
mod canrange;

pub use talon::*;
pub use cancoder::*;
pub use srx::*;
pub use pigeon::*;
pub use canrange::*;

pub enum TalonInvertType {
    CounterClockwise,
}

impl TalonInvertType {
    pub fn as_str(&self) -> &str {
        match self {
            TalonInvertType::CounterClockwise => "TalonFXCounterClockwise",
        }
    }
}



const TICKS_TO_ROTATIONS: f32 = (1.) / (1. * 12.8);

unit! {
    system: uom::si;
    quantity: uom::si::angle;


    // this is undocumented, but the base unit for rotation is radians despite being a "unitless" quantity
    @talon_encoder_tick: (crate::ctre::TICKS_TO_ROTATIONS*(std::f32::consts::PI * 2.0)).into(); "tt", "talon encoder tick", "talon encoder ticks";
}

pub trait ToTalonEncoder {
    fn talon_encoder_ticks(&self) -> f64;
    fn from_talon_encoder_ticks(&self) -> f64;
}

impl ToTalonEncoder for f64 {
    fn talon_encoder_ticks(&self) -> f64 {
        self / ((360.) / (2048. * 12.8))
    }

    fn from_talon_encoder_ticks(&self) -> f64 {
        self * ((360.) / (2048. * 12.8))
    }
}

#[cfg(test)]
mod tests {
    use uom::si::{f64::Angle, angle::{degree, revolution}};

    use super::{ToTalonEncoder, talon_encoder_tick};

    #[test]
    fn uom_equivalent() {
        let degrees = 42.;
        let tics = degrees.talon_encoder_ticks();

        let tics_uom = Angle::new::<degree>(degrees).get::<talon_encoder_tick>();

        let diff = tics - tics_uom;

        assert!(diff < 0.01);
    }

    #[test]
    fn uom_decode() {
        let angle = Angle::new::<degree>(42.);

        let tics = angle.get::<degree>().talon_encoder_ticks();

        let tics_uom = angle.get::<talon_encoder_tick>();

        let diff = tics - tics_uom;

        assert!(diff < 0.01);
    }
}
