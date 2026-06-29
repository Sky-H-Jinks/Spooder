use std::time::Duration;

use rppal::{gpio::Gpio, i2c::I2c};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creates an I2C bus instance and sets the slave address to 0x14
    let mut i2c: I2c = I2c::new()?;
    i2c.set_slave_address(0x14)?;

    /* Resets MCU via GPIO5 */
    let mut gpio = Gpio::new()?.get(5)?.into_output();
    gpio.set_low();
    std::thread::sleep(Duration::from_millis(10));

    gpio.set_high();
    std::thread::sleep(Duration::from_millis(200));

    // Frequency prescaler. PWM freq = clock / (prescaler × resolution).
    // 350 = high 0x01 (×256) + low 0x5E (94). → ~50Hz
    i2c.write(&[0x40, 0x01, 0x5E])?;

    // Sets the PWM resolution to 4095
    // 4096 steps, starts at 0 - Essentially divides a 20ms cycle into roughly 4.88µs tick segments
    // ticks = (pulse_us × 4095) / 20000
    i2c.write(&[0x44, 0x0F, 0xFF])?;
    
    // Centre servo: 307 ticks (0x0133) = 1500µs = 0°. (500µs/-90° … 2500µs/+90°)
    let center_ticks = build_servo_packet(0x20, calc_pulse_us_from_angle(0.0));
    i2c.write(&center_ticks)?;
    
    Ok(())
}

fn calc_pulse_us_from_angle(angle: f32) -> u32 {
    // Clamp the angle to the range -90° to +90°
    let clamped_angle = angle.clamp(-90.0, 90.0);
    
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