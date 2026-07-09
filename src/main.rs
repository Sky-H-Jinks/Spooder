#![deny(unused_must_use)]

use std::{time::Duration, time::Instant, thread::sleep};

use rppal::{gpio::Gpio, i2c::I2c};

use crate::{body::{Body, Keyframe, Pose}, hat::Hat, tween::Tween};

mod leg_servo;
mod leg;
mod tween;
mod hat;
mod body;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calibration_offsets = [
        [3.43, -0.58, 11.96],    // Leg 1
        [-2.39, -19.27, -19.98], // Leg 2
        [3.27, 0.67, -14.4],     // Leg 3 
        [3.96, -5.56, -11.96]    // Leg 4
    ];

    let mut body = Body::new(&calibration_offsets)?;
    body.home_all_servos()?;
    sleep(Duration::from_millis(500)); // Lets home settle
 
    let stand: Pose = [
        -60.0, 45.0, -45.0,   // leg 0: ankle, knee, hip
        -60.0, 45.0,  45.0,   // leg 1
        -60.0, 45.0,  45.0,   // leg 2
        -60.0, 45.0, -45.0,   // leg 3
    ];

    body.snap_pose(&Keyframe { pose: stand, duration: Duration::from_millis(500) })?;

    let stand_new: Pose = [
        -30.0, 75.0, -15.0,   // leg 0: ankle, knee, hip
        -30.0, 75.0,  15.0,   // leg 1
        -30.0, 75.0,  15.0,   // leg 2
        -30.0, 75.0, -15.0,   // leg 3
    ];

    body.snap_pose(&Keyframe { pose: stand_new, duration: Duration::from_millis(500) })?;


    Ok(())
}