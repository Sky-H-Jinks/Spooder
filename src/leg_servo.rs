use std::{string, thread::sleep, time::Instant, time::Duration};

use rppal::i2c::I2c;
use crate::tween::Tween;

pub struct LegServo {
    pub id: u8,
    pub leg_id: u8,
    pub calibration_offset: f32,
    pub current_angle_position: f32,
    pub homed: bool,
}

impl LegServo {
    pub fn new(id: u8, leg_id: u8, calibration_offset: f32) -> Self {
        LegServo { id, leg_id, calibration_offset, current_angle_position: 0.0, homed: false }
    }

    pub fn smooth_set_angle(&mut self, i2c: &mut I2c, angle: f32) -> Result<(), Box<dyn std::error::Error>> {
        if !self.homed {
            return Err("servo not homed".into());
        }
        
        let tween = Tween {
            servo_idx: (self.leg_id * 3 + self.id) as usize, // not used yet
            start: self.current_angle_position,
            started: Instant::now(),
            target: angle,
            duration: Duration::from_millis(360),
        };
        
        loop {
            let now = Instant::now(); 

            let new_angle = tween.sample(now);
            self.set_angle(new_angle, i2c)?;
            if tween.is_finished(now){
                break;
            }

            sleep(std::time::Duration::from_millis(20));
        }

        Ok(())
    }

    pub fn home_servo(&mut self, i2c: &mut I2c) -> Result<(), Box<dyn std::error::Error>> {
        if self.homed {
            return Ok(());
        }
        
        self.set_start_angle(i2c)
    }

    fn set_start_angle(&mut self, i2c: &mut I2c) -> Result<(), Box<dyn std::error::Error>> {
        self.set_angle(0.0, i2c)?;
        self.homed = true;
        Ok(())
    }

    fn set_angle(&mut self, angle: f32, i2c: &mut I2c) -> Result<(), Box<dyn std::error::Error>> {
        let register = self.get_register();
        let pulse = self.calc_pulse_us_from_angle(angle);
        let payload = Self::build_servo_packet(register, pulse);

        i2c.write(&payload)?;

        self.current_angle_position = angle;

        Ok(())
    } 

    fn get_register(&mut self) -> u8 {
        0x20 + (self.leg_id * 3) + self.id
    }

    fn calc_pulse_us_from_angle(&mut self, angle: f32) -> u32 {
        let calibrated = angle + self.calibration_offset;

        // Clamp the angle to the range -90° to +90°
        let clamped_angle = calibrated.clamp(-90.0, 90.0);
        let pulse_us = 1500.0 + (clamped_angle / 90.0) * 1000.0;       // all f32, can be 500.0..2500.0

        // redundant by design - safety net during development
        let clamped_pulse_us = pulse_us.clamp(500.0, 2500.0); // clamp to 500µs..2500µs - ensures that the pulse width is within the valid range for the servo
        
        clamped_pulse_us as u32                                             // single cast at the end
    }

    fn build_servo_packet(register: u8, pulse_us: u32) -> [u8; 3] {
        let ticks = (pulse_us * 4095) / 20000;
        let high_byte = ((ticks >> 8) & 0xFF) as u8; // >>8 divides by 256 → high byte (number of 256s)
        let low_byte = (ticks & 0xFF) as u8;         // &0xFF keeps low 8 bits → remainder
        [register, high_byte, low_byte]
    }
}