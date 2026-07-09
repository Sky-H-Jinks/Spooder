use std::time::Duration;

use rppal::{gpio::Gpio, i2c::I2c, gpio::OutputPin};

pub struct Hat {
    i2c: I2c,
    _reset: OutputPin,
}

impl Hat {
    pub fn new() -> Result<Hat, Box<dyn std::error::Error>> {
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
        i2c.write(&[0x41, 0x01, 0x5E])?;
        i2c.write(&[0x42, 0x01, 0x5E])?;
        i2c.write(&[0x43, 0x01, 0x5E])?;

        // Sets the PWM resolution to 4095
        // 4096 steps, starts at 0 - Essentially divides a 20ms cycle into roughly 4.88µs tick segments
        // ticks = (pulse_us × 4095) / 20000
        i2c.write(&[0x44, 0x0F, 0xFF])?;
        i2c.write(&[0x45, 0x0F, 0xFF])?;
        i2c.write(&[0x46, 0x0F, 0xFF])?;
        i2c.write(&[0x47, 0x0F, 0xFF])?;

        Ok(Hat { i2c, _reset: gpio })
    }

    pub fn set_servo_pulse(&mut self, channel: u8, pulse_us: u32) -> Result<(), Box<dyn std::error::Error>> {
        debug_assert!(channel < 12);
        
        let ticks = (pulse_us * 4095) / 20000;
        let high_byte = ((ticks >> 8) & 0xFF) as u8; // >>8 divides by 256 → high byte (number of 256s)
        let low_byte = (ticks & 0xFF) as u8;         // &0xFF keeps low 8 bits → remainder

        self.i2c.write(&[0x20 + channel, high_byte, low_byte])?;

        Ok(())
    }
} 