#![allow(unused_imports)]

use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};
use uom::si::{
    angle::{degree, radian},
    angular_velocity::radian_per_second,
    f64::{Angle, AngularVelocity, Length, Time, Velocity},
    length::{meter, millimeter},
    time::second,
    velocity::meter_per_second,
};

#[derive(Serialize, Deserialize)]
pub struct ChoreoTrajectory {
    pub name: String,
    pub version: u32,
    pub snapshot: Snapshot,
    pub params: Params,
    pub trajectory: TrajectoryData,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub waypoints: Vec<SnapshotWaypoint>,
    pub constraints: Vec<Constraint>,
    #[serde(rename = "targetDt")]
    pub target_dt: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotWaypoint {
    pub x: f64,
    pub y: f64,
    pub heading: f64,
    pub intervals: u32,
    pub split: bool,
    #[serde(rename = "fixTranslation")]
    pub fix_translation: bool,
    #[serde(rename = "fixHeading")]
    pub fix_heading: bool,
    #[serde(rename = "overrideIntervals")]
    pub override_intervals: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Constraint {
    // TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct Params {
    // TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct Event {
    // TODO: implement
}

#[derive(Serialize, Deserialize)]
pub struct TrajectoryData {
    pub samples: Vec<Sample>,
    pub waypoints: Vec<f64>,
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

pub struct Path {
    samples: Vec<PoseSample>,
    waypoints: Vec<f64>,
}

#[derive(Clone, Debug)]
struct PoseSample {
    time: f64,
    pose: Pose,
}

impl Path {
    pub fn from_trajectory(trajectory: &str) -> Result<Self, serde_json::Error> {
        let choreo = serde_json::from_str::<ChoreoTrajectory>(trajectory)?;

        let valid_waypoints = choreo
            .snapshot
            .waypoints
            .iter()
            .enumerate()
            .filter(|(_, wp)| wp.split)
            .map(|(i, _)| choreo.trajectory.waypoints[i])
            .collect();

        let trajectory_data = TrajectoryData {
            samples: choreo.trajectory.samples,
            waypoints: valid_waypoints,
        };

        Ok(Self::from_trajectory_data(trajectory_data))
    }

    fn from_trajectory_data(data: TrajectoryData) -> Self {
        let mut samples = Vec::with_capacity(data.samples.len());
        for sample in data.samples {
            samples.push(PoseSample {
                time: sample.t,
                pose: sample.into(),
            });
        }

        // Sort by time just in case
        samples.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        Self {
            samples,
            waypoints: data.waypoints,
        }
    }

    pub fn get(&self, elapsed: Time) -> Pose {
        let t = elapsed.get::<second>();

        match self
            .samples
            .binary_search_by(|probe| probe.time.partial_cmp(&t).unwrap())
        {
            Ok(idx) => self.samples[idx].pose.clone(), // exact match
            Err(0) => self.samples[0].pose.clone(),    // before first sample
            Err(idx) if idx >= self.samples.len() => self.samples.last().unwrap().pose.clone(), // after last sample
            Err(idx) => {
                let before = &self.samples[idx - 1];
                let after = &self.samples[idx];
                let progress = (t - before.time) / (after.time - before.time);
                before.pose.lerp(&after.pose, progress)
            }
        }
    }

    pub fn length(&self) -> Time {
        Time::new::<second>(self.samples.last().unwrap().time)
    }

    pub fn waypoints(&self) -> &Vec<f64> {
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
    fn lerp(&self, other: &Pose, l: f64) -> Pose {
        Pose {
            x: lerp(self.x, other.x, l),
            y: lerp(self.y, other.y, l),
            heading: lerp(self.heading, other.heading, l),
            angular_velocity: lerp(self.angular_velocity, other.angular_velocity, l),
            velocity_x: lerp(self.velocity_x, other.velocity_x, l),
            velocity_y: lerp(self.velocity_y, other.velocity_y, l),
        }
    }

    /// X and Y are half of the field length and width
    /// velocity might be wrong
    pub fn mirror(&self, x: Length, y: Length) -> Pose {
        Pose {
            x: x - self.x + x,
            y: y - self.y + y,
            heading: self.heading + Angle::new::<radian>(std::f64::consts::PI),
            angular_velocity: -self.angular_velocity,
            velocity_x: self.velocity_x,
            velocity_y: self.velocity_y,
        }
    }
}

fn lerp<A>(a: A, b: A, l: f64) -> A
where
    A: Sub<A, Output = A> + Add<A, Output = A> + Mul<f64, Output = A> + Clone,
{
    a.clone() + (b - a) * l
}

impl From<Sample> for Pose {
    fn from(value: Sample) -> Self {
        Self {
            x: Length::new::<meter>(value.x),
            y: Length::new::<meter>(value.y),
            heading: Angle::new::<radian>(value.heading),
            angular_velocity: AngularVelocity::new::<radian_per_second>(value.angular_velocity),
            velocity_x: Velocity::new::<meter_per_second>(value.velocity_x),
            velocity_y: Velocity::new::<meter_per_second>(value.velocity_y),
        }
    }
}

#[test]
fn parse() {
    let data = include_str!("../../RobotCode2025/auto/Blue2.traj");
    let path = Path::from_trajectory(data).unwrap();
    for i in path.waypoints() {
        println!("{}", i);
    }
}

#[test]
fn mirror_test() {
    let path = Path::from_trajectory(include_str!("../../RobotCode2025/auto/Blue2.traj")).unwrap();
    let setpoint = path.get(Time::new::<second>(0.0));

    println!("{:?}", setpoint);

    let setpoint = setpoint.mirror(
        Length::new::<meter>(17.55 / 2.),
        Length::new::<meter>(8.05 / 2.),
    );

    assert!((setpoint.x.get::<meter>() - 9.53808).abs() < 1e-5);
    assert!((setpoint.y.get::<meter>() - 0.44384).abs() < 1e-5);
    assert!((setpoint.heading.get::<degree>() - 180.).abs() < 1e-5);
}
