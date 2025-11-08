#include "../lib/leg_servo.h"
#include <linux/i2c-dev.h>
#include <linux/i2c.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <iostream>
#include <cstdint>
#include <fstream>
#include <vector>

LegServo::LegServo(int id, float calibration, I2CDevice* device)
    : id(id), calibrationOffset(calibration), device(device)
{
}


int LegServo::i2cWriteWord(int fd, uint8_t reg, uint16_t value)
{
    if(fd < 0) return -1;
    buffer[0] = reg;
    buffer[1] = (value >> 8) & 0xFF;   // MSB first
    buffer[2] = value & 0xFF;          // LSB second
    
    ssize_t n = write(fd, buffer, 3);
    if (n != 3) {
        perror("i2c write");
        return -1;
    }
    return 0;
}

int LegServo::PulseWidthToTicks(int pulse_us, int period)
{
    return (pulse_us * period) / 20000;
}

int LegServo::AngleToPulseWidth(float angle)
{
    angle += calibrationOffset;
    
    // Constrain angle to valid range
    if (angle < -90.0f) angle = -90.0f;
    if (angle > 90.0f) angle = 90.0f;
    
    // Map angle (-90 to +90) to pulse width (500μs to 2500μs)
    // angle = -90: 500μs
    // angle = 0:   1500μs
    // angle = +90: 2500μs
    int pulse_us = (int)((angle + 90.0f) * (2500 - 500) / 180.0f + 500);
    
    return pulse_us;

}

void LegServo::SetServoAngle(float angle)
{    
    if(!device) { std::cerr << "No I2C Device\n"; return; }
    int fd = device->get();
    if(fd < 0) { std::cerr << "Invalid I2C FD\n"; return; }

    // Convert angle to pulse width with offset
    int pulse_us = AngleToPulseWidth(angle);
    
    // Convert to ticks
    int ticks = PulseWidthToTicks(pulse_us);
    
    // Write to servo
    int reg = 0x20 + id;
    i2cWriteWord(fd, reg, ticks);
    
    std::cout << "  Servo P" << id 
              << ": angle=" << angle << "°"
              << ", offset=" << calibrationOffset << "°"
              << ", effective=" << (angle + calibrationOffset) << "°"
              << ", pulse=" << pulse_us << "μs"
              << ", ticks=" << ticks << std::endl;

    usleep(20'000);
}