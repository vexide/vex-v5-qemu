#include "v5_api.h"
#include "emu_devices.h"
#include "pros/error.h"
#include "pros/rtos.h"
#include <errno.h>
#include <string.h>

void vexDeviceImuReset(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
    device->imu.reset_timestamp = millis();
}

double vexDeviceImuHeadingGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return PROS_ERR_F;
    }
    return device->imu.rotation.z;
}

double vexDeviceImuDegreesGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return PROS_ERR_F;
    }
    return fmod(device->imu.rotation.z, 360);
}

void vexDeviceImuQuaternionGet(V5_DeviceT device, V5_DeviceImuQuaternion *data) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
}

void vexDeviceImuAttitudeGet(V5_DeviceT device, V5_DeviceImuAttitude *data) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
    data->pitch = device->imu.rotation.x;
    data->roll = device->imu.rotation.y;
    data->yaw = device->imu.rotation.z;
}

void vexDeviceImuRawGyroGet(V5_DeviceT device, V5_DeviceImuRaw *data) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
    memcpy(data, &device->imu.rotation, sizeof(V5_DeviceImuRaw));
}

void vexDeviceImuRawAccelGet(V5_DeviceT device, V5_DeviceImuRaw *data) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
    memcpy(data, &device->imu.acceleration, sizeof(V5_DeviceImuRaw));
}

uint32_t vexDeviceImuStatusGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    if (device->imu.reset_timestamp != 0) {
        if (millis() > (device->imu.reset_timestamp + 3000)) {
            device->imu.reset_timestamp = 0;
            return 0;
        }
        return 1;
    }
    return 0;
}

void vexDeviceImuModeSet(V5_DeviceT device, uint32_t mode) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
}

uint32_t vexDeviceImuModeGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
}

void vexDeviceImuDataRateSet(V5_DeviceT device, uint32_t rate) {
    if (!device->exists || device->type != kDeviceTypeImuSensor) {
        errno = ENODEV;
        return;
    }
}
