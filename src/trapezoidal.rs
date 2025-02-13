/// A stateful trapezoidal motion profile that accumulates time internally.
/// Instead of requiring the caller to provide the absolute time,
/// you update the profile with the elapsed time (dt) each loop.
pub struct TrapezoidalProfile {
    start: f64,
    goal: f64,
    max_vel: f64,
    max_acc: f64,
    t_accel: f64, // Time to accelerate (or reach peak velocity in a triangular profile)
    t_flat: f64,  // Time at constant velocity (zero for a triangular profile)
    t_total: f64, // Total time for the motion profile
    sign: f64,    // +1 or -1 depending on the direction
    elapsed: f64, // Accumulated time since the profile started (in seconds)
}

impl TrapezoidalProfile {
    /// Create a new trapezoidal profile.
    /// The profile’s timer is set to zero when constructed.
    pub fn new(start: f64, goal: f64, max_vel: f64, max_acc: f64) -> Self {
        let sign = if goal >= start { 1.0 } else { -1.0 };
        let distance = (goal - start).abs();
        // Candidate time to reach max velocity
        let t_accel_candidate = max_vel / max_acc;
        // Distance covered during acceleration (or deceleration)
        let d_accel = 0.5 * max_acc * t_accel_candidate * t_accel_candidate;
        let (t_accel, t_flat, t_total) = if distance < 2.0 * d_accel {
            // Triangular profile: never reach max_vel
            let t_accel = (distance / max_acc).sqrt();
            let t_total = 2.0 * t_accel;
            (t_accel, 0.0, t_total)
        } else {
            // Trapezoidal profile: accelerate, cruise, then decelerate
            let t_flat = (distance - 2.0 * d_accel) / max_vel;
            let t_total = 2.0 * t_accel_candidate + t_flat;
            (t_accel_candidate, t_flat, t_total)
        };

        Self {
            start,
            goal,
            max_vel,
            max_acc,
            t_accel,
            t_flat,
            t_total,
            sign,
            elapsed: 0.0,
        }
    }

    /// Update the profile by the elapsed time `dt` (in seconds).
    /// This increments the internal timer and returns the current setpoint (position, velocity).
    pub fn update(&mut self, dt: f64) -> (f64, f64) {
        self.elapsed += dt;
        let pos = self.position_at(self.elapsed);
        let vel = self.velocity_at(self.elapsed);
        (pos, vel)
    }

    /// Compute the desired position at time `t` (in seconds).
    pub fn position_at(&self, t: f64) -> f64 {
        if t <= 0.0 {
            return self.start;
        } else if t >= self.t_total {
            return self.goal;
        }
        if t < self.t_accel {
            // Acceleration phase: x = 0.5 * a * t^2
            let pos = 0.5 * self.max_acc * t * t;
            self.start + self.sign * pos
        } else if t < self.t_accel + self.t_flat {
            // Constant velocity phase
            let pos_accel = 0.5 * self.max_acc * self.t_accel * self.t_accel;
            let pos_const = self.max_vel * (t - self.t_accel);
            self.start + self.sign * (pos_accel + pos_const)
        } else {
            // Deceleration phase
            let t_dec = t - self.t_accel - self.t_flat;
            let pos_accel = 0.5 * self.max_acc * self.t_accel * self.t_accel;
            let pos_const = self.max_vel * self.t_flat;
            let pos_decel = self.max_vel * t_dec - 0.5 * self.max_acc * t_dec * t_dec;
            self.start + self.sign * (pos_accel + pos_const + pos_decel)
        }
    }

    /// Compute the desired velocity at time `t` (in seconds).
    pub fn velocity_at(&self, t: f64) -> f64 {
        if t <= 0.0 || t >= self.t_total {
            return 0.0;
        }
        if t < self.t_accel {
            self.max_acc * t * self.sign
        } else if t < self.t_accel + self.t_flat {
            self.max_vel * self.sign
        } else {
            let t_dec = t - self.t_accel - self.t_flat;
            (self.max_vel - self.max_acc * t_dec) * self.sign
        }
    }
}

/// A simple PID controller for demonstration purposes.
pub struct PID {
    k_p: f64,
    k_i: f64,
    k_d: f64,
    prev_error: f64,
    integral: f64,
}

impl PID {
    pub fn new(k_p: f64, k_i: f64, k_d: f64) -> Self {
        Self {
            k_p,
            k_i,
            k_d,
            prev_error: 0.0,
            integral: 0.0,
        }
    }

    /// Updates the PID loop with the setpoint and measured value.
    /// Returns a control effort (percent output, clamped between -1.0 and 1.0).
    pub fn update(&mut self, setpoint: f64, measured: f64, dt: f64) -> f64 {
        let error = setpoint - measured;
        self.integral += error * dt;
        let derivative = (error - self.prev_error) / dt;
        self.prev_error = error;
        let output = self.k_p * error + self.k_i * self.integral + self.k_d * derivative;
        output.clamp(-1.0, 1.0)
    }
}

#[test]
fn trapezoid() {
    // Define motion profile parameters
    let start_position = 0.0;
    let goal_position = 100.0;
    let max_velocity = 50.0;     // units per second (for example, inches/sec or degrees/sec)
    let max_acceleration = 100.0; // units per second^2

    // Create the trapezoidal profile
    let profile = TrapezoidalProfile::new(start_position, goal_position, max_velocity, max_acceleration);

    // Simulate a control loop running at 20ms intervals.
    let dt = 0.02; // 20 ms
    let mut t = 0.0;

    println!("Time\tDesired\tFeedforward Velocity");
    // Run the loop a bit longer than the profile total time
    while t <= profile.t_total {
        // Get the desired setpoint from the trapezoidal profile.
        let desired_position = profile.position_at(t);
        // (Optionally) retrieve the desired velocity for feedforward control.
        let desired_velocity = profile.velocity_at(t);

        println!(
            "{:.2}\t{:.2}\t{:.2}",
            t, desired_position, desired_velocity
        );

        t += dt;
    }
}
