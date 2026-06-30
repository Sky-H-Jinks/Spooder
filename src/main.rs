use std::time::Duration;

use rppal::{gpio::Gpio, i2c::I2c};

mod leg_servo;
mod leg;

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
    
    let leg1 = leg::Leg::new(0, [0.0, 0.0, 0.0]);
    leg1.move_joint(leg::LegJoint::Hip, 0.0, &mut i2c)?;
    leg1.move_joint(leg::LegJoint::Knee, 0.0, &mut i2c)?;
    leg1.move_joint(leg::LegJoint::Ankle, 0.0, &mut i2c)?;

    let leg2 = leg::Leg::new(1, [0.0, 0.0, 0.0]);
    leg2.move_joint(leg::LegJoint::Hip, 0.0, &mut i2c)?;
    leg2.move_joint(leg::LegJoint::Knee, 0.0, &mut i2c)?;
    leg2.move_joint(leg::LegJoint::Ankle, 0.0, &mut i2c)?;

    let leg3 = leg::Leg::new(2, [0.0, 0.0, 0.0]);
    leg3.move_joint(leg::LegJoint::Hip, 0.0, &mut i2c)?;
    leg3.move_joint(leg::LegJoint::Knee, 0.0, &mut i2c)?;
    leg3.move_joint(leg::LegJoint::Ankle, 0.0, &mut i2c)?;

    let leg4 = leg::Leg::new(3, [0.0, 0.0, 0.0]);
    leg4.move_joint(leg::LegJoint::Hip, 0.0, &mut i2c)?;
    leg4.move_joint(leg::LegJoint::Knee, 0.0, &mut i2c)?;
    leg4.move_joint(leg::LegJoint::Ankle, 0.0, &mut i2c)?;
    
    Ok(())
}