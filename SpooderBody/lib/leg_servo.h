#pragma once

#include <cstdint> 
#include "I2CDevice.h"

class LegServo {
public: 
    int id;
    
    LegServo(int id, float calibration, I2CDevice* device);

    void SetServoAngle(float angle);

private:
    float calibrationOffset;
    uint8_t buffer[3];
    I2CDevice* device;
    int PulseWidthToTicks(int pulse_us, int period = 4095);
    int AngleToPulseWidth(float angle);
    int i2cWriteWord(int fd, uint8_t reg, uint16_t value);
};