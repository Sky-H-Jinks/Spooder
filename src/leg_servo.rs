use crate::hat::Hat;

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

    pub fn home_servo(&mut self, hat: &mut Hat) -> Result<(), Box<dyn std::error::Error>> {
        if self.homed {
            return Ok(());
        }
        
        self.set_start_angle(hat)
    }

    fn set_start_angle(&mut self, hat: &mut Hat) -> Result<(), Box<dyn std::error::Error>> {
        self.set_angle(0.0, hat)?;
        self.homed = true;
        Ok(())
    }

    pub fn set_angle(&mut self, angle: f32, hat: &mut Hat) -> Result<(), Box<dyn std::error::Error>> {
        let register = self.get_channel();
        let pulse = self.calc_pulse_us_from_angle(angle);

        hat.set_servo_pulse(register, pulse)?;
        self.current_angle_position = angle;

        Ok(())
    } 

    fn get_channel(&mut self) -> u8 {
        (self.leg_id * 3) + self.id
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
}