use rppal::i2c::I2c;
use std::{thread::sleep, time::Duration};
use crate::{leg_servo::LegServo, tween::Tween};

#[derive(Clone, Copy)]
pub enum LegJoint {
    Ankle = 0,
    Knee = 1,
    Hip = 2
}

pub struct Leg {
    pub id: u8,
    pub servos: [LegServo; 3]
}

impl Leg {
    pub fn new(id: u8, offsets: [f32; 3]) -> Leg {
        let servos:  [LegServo; 3] = [
            LegServo::new(0, id, offsets[LegJoint::Ankle as usize]),
            LegServo::new(1, id, offsets[LegJoint::Knee as usize]),
            LegServo::new(2, id, offsets[LegJoint::Hip as usize])
        ];

        Leg { id, servos }
    }

    pub fn move_joint(&mut self, joint: LegJoint, i2c: &mut I2c, angle: f32) -> Result<(), Box<dyn std::error::Error>> {
        self.servos[joint as usize].smooth_set_angle(i2c, angle)
    }

    pub fn move_joint_start_pos(&mut self, i2c: &mut I2c) -> Result<(), Box<dyn std::error::Error>> {
        for servo in &mut self.servos {
            servo.home_servo(i2c);
            sleep(Duration::from_millis(100));
        }
        
        Ok(())
    }
}