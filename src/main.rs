use std::{time::Duration, time::Instant, thread::sleep};

use rppal::{gpio::Gpio, i2c::I2c};

use crate::tween::Tween;

mod leg_servo;
mod leg;
mod tween;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let calibration_offsets = [
        [3.43, -0.58, 11.96],    // Leg 1
        [-2.39, -19.27, -19.98], // Leg 2
        [3.27, 0.67, -14.4],    // Leg 3 <- Currently changing leg 2?????
        [3.96, -5.56, -11.96]    // Leg 4
    ];

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
    
    let mut leg1 = leg::Leg::new(0, calibration_offsets[0]);
    let mut leg2 = leg::Leg::new(1, calibration_offsets[1]);
    let mut leg3 = leg::Leg::new(2, calibration_offsets[2]);
    let mut leg4 = leg::Leg::new(3, calibration_offsets[3]);

    for (index, offset_collection) in calibration_offsets.iter().enumerate() {
        for (servo_index, servo_offset) in offset_collection.iter().enumerate() {
            println!("Leg {} Servo {} Offset {}", index + 1, servo_index, servo_offset);
        }
    }

    leg1.move_joint_start_pos(&mut i2c);
    sleep(Duration::from_millis(100));

    leg2.move_joint_start_pos(&mut i2c);
    sleep(Duration::from_millis(100));

    leg3.move_joint_start_pos(&mut i2c);
    sleep(Duration::from_millis(100));

    leg4.move_joint_start_pos(&mut i2c);
    sleep(Duration::from_millis(100));


    let now = Instant::now();
    let duration = Duration::from_secs(15);

    // Waving loop to test smoothness
    /*loop {
        sleep(Duration::from_millis(100));

        leg1.move_joint(leg::LegJoint::Ankle, &mut i2c, -45.0);
        leg1.move_joint(leg::LegJoint::Ankle, &mut i2c, 45.0);

        sleep(Duration::from_millis(100));

        leg2.move_joint(leg::LegJoint::Ankle, &mut i2c, -45.0);
        leg2.move_joint(leg::LegJoint::Ankle, &mut i2c, 45.0);

        sleep(Duration::from_millis(100));

        leg3.move_joint(leg::LegJoint::Ankle, &mut i2c, -45.0);
        leg3.move_joint(leg::LegJoint::Ankle, &mut i2c, 45.0);

        sleep(Duration::from_millis(100));

        leg4.move_joint(leg::LegJoint::Ankle, &mut i2c, -45.0);
        leg4.move_joint(leg::LegJoint::Ankle, &mut i2c, 45.0);

        if now.elapsed() >= duration {
            break;
        }
    }
    */
    

    /*  ** TODO ** 
        - Seperate out each servo so each address can be given commands together 
    
     */

    Ok(())
}