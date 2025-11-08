#pragma once

#include <cstdint> 
#include "leg_servo.h"
#include "I2CDevice.h"

class Leg {
public:
    int8_t id;
    Leg(uint8_t id, float c1, float c2, float c3, I2CDevice* device) {
        this->id = id;
        this->Hip = new LegServo(id * 3 + 0, c1, device);
        this->Knee = new LegServo(id * 3 + 1, c2, device);
        this->Ankle = new LegServo(id * 3 + 2, c3, device);
    }

    void Stand() {
        this->Hip->SetServoAngle(90.0f);
        this->Knee->SetServoAngle(90.0f);
        this->Ankle->SetServoAngle(45.0f);
    }

private:
    LegServo* Hip;
    LegServo* Knee;
    LegServo* Ankle;
};