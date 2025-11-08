#include <linux/i2c-dev.h>
#include <linux/i2c.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <unistd.h>
#include <iostream>
#include <cstdint>
#include <fstream>
#include <vector>

#include "../lib/leg.h"
#include "../lib/I2CDevice.h"

const std::string I2C_DEVICE_PATH = "/dev/i2c-1";

const std::vector<float> CALIBRATION_OFFSETS = {
    3.43, -0.58, 11.96, -2.39, -19.27, -19.98,
    -3.27, 0.67, -16.4, 3.96, -5.56, -11.96
};

Leg* Legs[4];

static void reset_mcu() {
    std::cout << "Resetting MCU..." << std::endl;
    system("echo 5 > /sys/class/gpio/export 2>/dev/null");
    usleep(100000);
    system("echo out > /sys/class/gpio/gpio5/direction");
    usleep(10000);
    system("echo 0 > /sys/class/gpio/gpio5/value");
    usleep(10000);
    system("echo 1 > /sys/class/gpio/gpio5/value");
    usleep(200000);
    std::cout << "✓ MCU reset complete" << std::endl;
}

static bool init_pwm(I2CDevice &dev) {
    if (!dev.ok()) return false;
    // these match your working example
    if (dev.writeWord(0x40, 350) != 0) { perror("write prescaler"); return false; }
    if (dev.writeWord(0x44, 4095) != 0) { perror("write period"); return false; }
    usleep(100000);
    return true;
}

int main() {
    reset_mcu();   

    I2CDevice dev(I2C_DEVICE_PATH, 0x14);
    if (!dev.ok()) { std::cerr << "Failed to open I2C device\n"; return 1; }
    if (!init_pwm(dev)) { std::cerr << "PWM init failed\n"; return 1; }

    for(int x = 0; x < 4; x++) {
        Legs[x] = new Leg(x,
            CALIBRATION_OFFSETS[x * 3 + 0],
            CALIBRATION_OFFSETS[x * 3 + 1],
            CALIBRATION_OFFSETS[x * 3 + 2],
            &dev
        );
    }

    for(int x = 0; x < 4; x++) {
        (*Legs[x]).Stand();
    }
}