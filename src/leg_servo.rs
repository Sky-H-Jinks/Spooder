use std::string;

use rppal::i2c::I2c;

pub struct LegServo {
    pub id: u8,
    pub leg_id: u8,
    pub calibration_offset: f32,
}

impl LegServo {
    pub fn new(id: u8, leg_id: u8, calibration_offset: f32) -> Self {
        LegServo { id, leg_id, calibration_offset }
    }

    pub fn set_angle(&self, angle: f32, i2c: &mut I2c) -> Result<(), Box<dyn std::error::Error>> {
        let register = self.get_register();
        let pulse = self.calc_pulse_us_from_angle(angle);
        let payload = Self::build_servo_packet(register, pulse);

        i2c.write(&payload)?;

        Ok(())
    }

    fn get_register(&self) -> u8 {
        0x20 + (self.leg_id * 3) + self.id
    }

    fn calc_pulse_us_from_angle(&self, angle: f32) -> u32 {
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