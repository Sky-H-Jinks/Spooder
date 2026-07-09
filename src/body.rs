use crate::{hat::Hat, leg::Leg, leg_servo::LegServo};
use std::{time::Duration, thread::sleep};

pub type Pose = [f32; 12];
pub struct Keyframe {
    pub pose: Pose,
    pub duration: Duration
}

pub struct Body {
    hat: Hat,
    legs: [Leg; 4]
}

impl Body {
    pub fn new(offsets: &[[f32; 3]; 4]) -> Result<Body, Box<dyn std::error::Error>> {
        let legs = std::array::from_fn(|i| Leg::new(i as u8, offsets[i]));
        Ok(Body { hat: Hat::new()?, legs })
    }

    pub fn snap_pose(&mut self, kf: &Keyframe) -> Result<(), Box<dyn std::error::Error>> {
        for(x, i) in kf.pose.iter().copied().enumerate() {
            self.legs[x/3].servos[x%3].set_angle(i, &mut self.hat)?;
            sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    pub fn home_all_servos(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for i in &mut self.legs{
            i.move_joint_start_pos(&mut self.hat)?;
        }

        Ok(())
    }
}