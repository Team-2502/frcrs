#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};
use uom::si::{
    angle::radian,
    angular_velocity::radian_per_second,
    f64::{Angle, AngularVelocity, Length, Time, Velocity},
    length::meter,
    time::second,
    velocity::meter_per_second,
};

#[derive(Serialize, Deserialize)]
pub struct ChoreoTrajectory {
    pub name: String,
    pub version: u32,
    pub snapshot: Snapshot,
    pub trajectory: Trajectory,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub waypoints: Vec<SnapshotWaypoint>,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotWaypoint {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub split: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Event {}

#[derive(Serialize, Deserialize)]
pub struct Trajectory {
    pub waypoints: Vec<f64>,
    pub splits: Vec<usize>,
    pub samples: Vec<Sample>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sample {
    pub t: f64,
    pub x: f64,
    pub y: f64,
    pub heading: f64,

    #[serde(rename = "vx")]
    pub velocity_x: f64,

    #[serde(rename = "vy")]
    pub velocity_y: f64,

    #[serde(rename = "omega")]
    pub angular_velocity: f64,
}

#[derive(Debug)]
pub struct Path {
    samples: Vec<PoseSample>,
    waypoints: Vec<f64>,
}

#[derive(Debug)]
struct PoseSample {
    time: f64,
    pose: Pose,
}

impl Path {
    pub fn from_trajectory(json: &str) -> Result<Self, serde_json::Error> {
        let choreo = serde_json::from_str::<ChoreoTrajectory>(json)?;

        let mut valid_waypoints = choreo
            .trajectory
            .splits
            .iter()
            .filter_map(|&i| choreo.trajectory.waypoints.get(i).copied())
            .collect::<Vec<_>>();

        if let Some(&last) = choreo.trajectory.waypoints.last() {
            valid_waypoints.push(last);
        }

        let mut samples = choreo
            .trajectory
            .samples
            .into_iter()
            .map(|s| PoseSample {
                time: s.t,
                pose: s.into(),
            })
            .collect::<Vec<_>>();

        samples.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        Ok(Self {
            samples,
            waypoints: valid_waypoints,
        })
    }

    pub fn get(&self, time: Time) -> Pose {
        let t = time.get::<second>();

        match self
            .samples
            .binary_search_by(|s| s.time.partial_cmp(&t).unwrap())
        {
            Ok(i) => self.samples[i].pose.clone(),
            Err(0) => self.samples[0].pose.clone(),
            Err(i) if i >= self.samples.len() => self.samples.last().unwrap().pose.clone(),
            Err(i) => {
                let a = &self.samples[i - 1];
                let b = &self.samples[i];

                let progress = (t - a.time) / (b.time - a.time);

                a.pose.lerp(&b.pose, progress)
            }
        }
    }

    pub fn length(&self) -> Time {
        Time::new::<second>(self.samples.last().unwrap().time)
    }

    pub fn waypoints(&self) -> &[f64] {
        &self.waypoints
    }
}

#[derive(Clone, Debug)]
pub struct Pose {
    pub x: Length,
    pub y: Length,
    pub heading: Angle,
    pub angular_velocity: AngularVelocity,
    pub velocity_x: Velocity,
    pub velocity_y: Velocity,
}

impl Pose {
    fn lerp(&self, other: &Pose, t: f64) -> Pose {
        Pose {
            x: lerp(self.x, other.x, t),
            y: lerp(self.y, other.y, t),

            heading: Angle::new::<radian>(lerp_angle(
                self.heading.get::<radian>(),
                other.heading.get::<radian>(),
                t,
            )),

            angular_velocity: lerp(self.angular_velocity, other.angular_velocity, t),
            velocity_x: lerp(self.velocity_x, other.velocity_x, t),
            velocity_y: lerp(self.velocity_y, other.velocity_y, t),
        }
    }

    pub fn mirror(&self, half_field_x: Length, half_field_y: Length) -> Pose {
        Pose {
            x: half_field_x * 2.0 - self.x,
            y: half_field_y * 2.0 - self.y,

            heading: Angle::new::<radian>(self.heading.get::<radian>() + std::f64::consts::PI),

            angular_velocity: -self.angular_velocity,

            velocity_x: -self.velocity_x,
            velocity_y: -self.velocity_y,
        }
    }
}

fn lerp<T>(a: T, b: T, t: f64) -> T
where
    T: Sub<T, Output = T> + Add<T, Output = T> + Mul<f64, Output = T> + Clone,
{
    a.clone() + (b - a) * t
}

fn lerp_angle(a: f64, b: f64, t: f64) -> f64 {
    let mut diff = b - a;

    while diff > std::f64::consts::PI {
        diff -= 2.0 * std::f64::consts::PI;
    }

    while diff < -std::f64::consts::PI {
        diff += 2.0 * std::f64::consts::PI;
    }

    a + diff * t
}

impl From<Sample> for Pose {
    fn from(s: Sample) -> Self {
        Self {
            x: Length::new::<meter>(s.x),
            y: Length::new::<meter>(s.y),
            heading: Angle::new::<radian>(s.heading),
            angular_velocity: AngularVelocity::new::<radian_per_second>(s.angular_velocity),
            velocity_x: Velocity::new::<meter_per_second>(s.velocity_x),
            velocity_y: Velocity::new::<meter_per_second>(s.velocity_y),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uom::si::{length::meter, time::second};

    fn fake_path() -> &'static str {
        r#"
        {
            "name": "test_path",
            "version": 3,
            "snapshot": {
                "waypoints": []
            },
            "events": [],
            "trajectory": {
                "waypoints": [0.0, 1.0],
                "splits": [0],
                "samples": [
                    {
                        "t": 0.0,
                        "x": 0.0,
                        "y": 0.0,
                        "heading": 0.0,
                        "vx": 0.0,
                        "vy": 0.0,
                        "omega": 0.0
                    },
                    {
                        "t": 1.0,
                        "x": 1.0,
                        "y": 2.0,
                        "heading": 1.0,
                        "vx": 1.0,
                        "vy": 2.0,
                        "omega": 0.5
                    }
                ]
            }
        }
        "#
    }

    #[test]
    fn parse_json() {
        let path = Path::from_trajectory(fake_path()).unwrap();

        assert_eq!(path.samples.len(), 2);
    }

    #[test]
    fn waypoints_parse_correctly() {
        let path = Path::from_trajectory(fake_path()).unwrap();

        let waypoints = path.waypoints();

        assert_eq!(waypoints.len(), 2);
        assert_eq!(waypoints[0], 0.0);
    }

    #[test]
    fn duration_is_correct() {
        let path = Path::from_trajectory(fake_path()).unwrap();

        let duration = path.length();

        assert_eq!(duration.get::<second>(), 1.0);
    }

    #[test]
    fn exact_sample_retrieval() {
        let path = Path::from_trajectory(fake_path()).unwrap();

        let pose = path.get(Time::new::<second>(1.0));

        assert_eq!(pose.x.get::<meter>(), 1.0);
        assert_eq!(pose.y.get::<meter>(), 2.0);
    }

    #[test]
    fn interpolation_midpoint() {
        let path = Path::from_trajectory(fake_path()).unwrap();

        let pose = path.get(Time::new::<second>(0.5));

        assert!((pose.x.get::<meter>() - 0.5) < 1e-6);
        assert!((pose.y.get::<meter>() - 1.0) < 1e-6);
    }
}
