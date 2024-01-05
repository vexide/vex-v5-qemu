#include <math.h>
#include <stdio.h>

#include "emu_devices.h"
#include "pros/error.h"

#include <errno.h>

#define MOTOR_TEMP_MAX 40 // todo

void vexDeviceMotorVelocitySet(V5_DeviceT device, int32_t velocity) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.velocity_target = velocity;
    device->motor.controlMode = kMotorControlModeVELOCITY;
}

void vexDeviceMotorVelocityUpdate(V5_DeviceT device, int32_t velocity) { //todo not at all confident in this one
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.velocity_target = velocity;
}

void vexDeviceMotorVoltageSet(V5_DeviceT device, int32_t value) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.voltage = value;
    device->motor.controlMode = kMotorControlModeUNDEFINED;
}

int32_t vexDeviceMotorVelocityGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.velocity_target;
}

double vexDeviceMotorActualVelocityGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR_F;
    }
    return device->motor.velocity;
}

int32_t vexDeviceMotorDirectionGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.reverseFlag ? -1 : 1;
}

void vexDeviceMotorModeSet(V5_DeviceT device, V5MotorControlMode mode) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.controlMode = mode;
}

V5MotorControlMode vexDeviceMotorModeGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.controlMode;
}

void vexDeviceMotorPwmSet(V5_DeviceT device, __attribute((unused)) int32_t _) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    printf("This function does nothing!");
}

int32_t vexDeviceMotorPwmGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    printf("This function does nothing!");
    return 0;
}

void vexDeviceMotorCurrentLimitSet(V5_DeviceT device, int32_t value) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.current_max = value;
}

int32_t vexDeviceMotorCurrentLimitGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.current_max;
}

void vexDeviceMotorVoltageLimitSet(V5_DeviceT device, int32_t value) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.current_max = value;
}

int32_t vexDeviceMotorVoltageLimitGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.voltage_max;
}

void vexDeviceMotorPositionPidSet(V5_DeviceT device, V5_DeviceMotorPid* pid) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.pos_pid = pid;
}

void vexDeviceMotorVelocityPidSet(V5_DeviceT device, V5_DeviceMotorPid* pid) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.vel_pid = pid;
}

int32_t vexDeviceMotorCurrentGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.current;
}

int32_t vexDeviceMotorVoltageGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.voltage;
}

double vexDeviceMotorPowerGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.power;
}

double vexDeviceMotorTorqueGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.torque;
}

double vexDeviceMotorEfficiencyGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.efficiency;
}

double vexDeviceMotorTemperatureGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.temperature;
}

bool vexDeviceMotorOverTempFlagGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.temperature > MOTOR_TEMP_MAX;
}

bool vexDeviceMotorCurrentLimitFlagGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.currentLimitFlag;
}

uint32_t vexDeviceMotorFaultsGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.faults;
}

bool vexDeviceMotorZeroVelocityFlagGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.zeroVelocityFlag;
}

bool vexDeviceMotorZeroPositionFlagGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.zeroPositionFlag;
}

uint32_t vexDeviceMotorFlagsGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.flags;
}

void vexDeviceMotorReverseFlagSet(V5_DeviceT device, bool value) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.reverseFlag = value;
}

bool vexDeviceMotorReverseFlagGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.reverseFlag;
}

void vexDeviceMotorEncoderUnitsSet(V5_DeviceT device, V5MotorEncoderUnits units) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.encoderUnits = units;
}

V5MotorEncoderUnits vexDeviceMotorEncoderUnitsGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.encoderUnits;
}

void vexDeviceMotorBrakeModeSet(V5_DeviceT device, V5MotorBrakeMode mode) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.brakeMode = mode;
}

V5MotorBrakeMode vexDeviceMotorBrakeModeGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.brakeMode;
}

void vexDeviceMotorPositionSet(V5_DeviceT device, double position) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.position = position;
}

double vexDeviceMotorPositionGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR_F;
    }
    return device->motor.position; // todo
}

int32_t vexDeviceMotorPositionRawGet(V5_DeviceT device, uint32_t* timestamp) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    *timestamp = device->timestamp;
    return (int32_t) device->motor.position;
}

void vexDeviceMotorPositionReset(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.position = 0;
}

double vexDeviceMotorTargetGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR_F;
    }
    return device->motor.position_target;
}

void vexDeviceMotorServoTargetSet(V5_DeviceT device, __attribute((unused)) double _) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    printf("This function does nothing!");
}

void vexDeviceMotorAbsoluteTargetSet(V5_DeviceT device, double position, int32_t velocity) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.position_target = position;
    device->motor.velocity_max = velocity;
    device->motor.controlMode = kMotorControlModePROFILE;
}

void vexDeviceMotorRelativeTargetSet(V5_DeviceT device, double position, int32_t velocity) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.position_target = device->motor.position + position; // todo
    device->motor.velocity_max = velocity;
    device->motor.controlMode = kMotorControlModePROFILE;
}

void vexDeviceMotorGearingSet(V5_DeviceT device, V5MotorGearset value) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    device->motor.gearset = value;
}

V5MotorGearset vexDeviceMotorGearingGet(V5_DeviceT device) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return PROS_ERR;
    }
    return device->motor.gearset;
}

void vexDeviceMotorExternalProfileSet(V5_DeviceT device, __attribute((unused)) _, __attribute((unused)) int32_t __) {
    if (!device->exists || device->type != kDeviceTypeMotorSensor) {
        errno = ENODEV;
        return;
    }
    printf("This function does nothing!");
}
