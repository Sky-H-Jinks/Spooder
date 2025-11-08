#pragma once
#include <string>
#include <fcntl.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <linux/i2c-dev.h>
#include <memory>
#include <cstdio>

class I2CDevice {
public:
    I2CDevice(const std::string &devPath, int addr) : fd(-1), address(addr) {
        fd = open(devPath.c_str(), O_RDWR);
        if (fd < 0) { perror("open i2c"); return; }
        if (ioctl(fd, I2C_SLAVE, address) < 0) { perror("ioctl I2C_SLAVE"); close(fd); fd = -1; }
    }
    ~I2CDevice(){ if (fd >= 0) close(fd); }
    int get() const { return fd; }
    bool ok() const { return fd >= 0; }

    // helper convenience for your existing i2cWriteWord
    int writeWord(uint8_t reg, uint16_t value) {
        if (fd < 0) return -1;
        // MSB-first to match Robot HAT v4 single-file example
        uint8_t buf[3] = { reg, static_cast<uint8_t>((value >> 8) & 0xFF), static_cast<uint8_t>(value & 0xFF) };
        ssize_t n = ::write(fd, buf, sizeof(buf));
        return (n == (ssize_t)sizeof(buf)) ? 0 : -1;
    }

private:
    int fd;
    int address;
};