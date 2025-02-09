/**
*Trapezoidal motion profile*
- max_accel_rate: Change in output per second during acceleration
- max_decel_rate: Change in output per second during deceleration
- cruise_power: Max power during cruise (-1.0 to 1.0)
- last_output: Tracks last output for rate limiting
- tolerance: Tolerance for reaching target position
*/
pub struct TrapezoidalProfile {
    max_accel_rate: f64,
    max_decel_rate: f64,
    cruise_power: f64,
    last_output: f64,
    tolerance: f64,
}

impl TrapezoidalProfile {
    pub fn new(max_accel_rate: f64, max_decel_rate: f64, cruise_power: f64, tolerance: f64) -> Self {
        TrapezoidalProfile {
            max_accel_rate,
            max_decel_rate,
            cruise_power: cruise_power.clamp(-1.0, 1.0),
            last_output: 0.0,
            tolerance
        }
    }

    pub fn calculate_output(&mut self, current_position: f64, target_position: f64, dt: f64) -> f64 {
        let total_distance = target_position - current_position;
        let direction = total_distance.signum();
        let distance_remaining = total_distance.abs();

        // If we're basically there, stop
        if distance_remaining < 1.0 {
            self.last_output = 0.0;
            return 0.0;
        }

        // Calculate desired output based on distance
        let desired_output = if distance_remaining > 0.0 {
            self.cruise_power * direction
        } else {
            0.0
        };

        // Apply rate limiting
        let max_change = if (desired_output.abs() > self.last_output.abs())
            || (desired_output.signum() != self.last_output.signum()) {
            // Accelerating or changing direction
            self.max_accel_rate * dt
        } else {
            // Decelerating
            self.max_decel_rate * dt
        };

        // Calculate new output with rate limiting
        let output_change = desired_output - self.last_output;
        let limited_change = output_change.clamp(-max_change, max_change);
        let output = self.last_output + limited_change;

        // Store for next iteration
        self.last_output = output;

        // Clamp final output to valid range
        output.clamp(-self.cruise_power, self.cruise_power)
    }
}


#[test]
fn trapezoidal() {
    // Create profile:
    // - Accelerate at 2.0 units per second
    // - Decelerate at 4.0 units per second
    // - Cruise at 80% power
    let mut profile = TrapezoidalProfile::new(20.0, 40.0, 0.8);

    // Test over time
    let target = 5000.0;
    let dt = 0.02; // 20ms loop time

    let positions = [0.0, 500.0, 1000.0, 2000.0, 3500.0, 4000.0, 4250.0, 4500.0, 4750.0, 4900.0, 5100.0, 5200.0, 5300.0, 5400.0, 5500.0, 5600.0, 5700.0, 5800.0, 5900.0, 6000.0];

    for pos in positions.iter() {
        let output = profile.calculate_output(*pos, target, dt);
        println!("Position: {:.2}, Output: {:.3}", pos, output);
    }
}