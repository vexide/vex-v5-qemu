#pragma once

#include "v5_apitypes.h"

typedef struct {
        V5_AdiPortConfiguration type;
} emu_adi;

struct _V5_Device {
        int8_t port;
        int8_t exists;
        V5_DeviceType type;
        uint32_t timestamp;

        union {
                struct {
                        V5MotorBrakeMode brakeMode;
                        V5MotorControlMode controlMode;
                        V5MotorEncoderUnits encoderUnits;
                        V5MotorGearset gearset;
                        V5_DeviceMotorPid *pos_pid, *vel_pid;
                        int32_t velocity_target, velocity_max, current, current_max, voltage, voltage_max;
                        double position, position_target, velocity, power, torque, efficiency, temperature;
                        uint32_t faults;

                        union {
                                uint8_t flags;

                                struct {
                                        bool overTempFlag : 1;
                                        bool currentLimitFlag : 1;
                                        bool zeroVelocityFlag : 1;
                                        bool zeroPositionFlag : 1;
                                        bool reverseFlag : 1;
                                };
                        };
                } motor;

                struct {
                        V5ImuOrientationMode orientationMode;
                        V5_DeviceImuRaw rotation;
                        V5_DeviceImuRaw acceleration;
                        uint32_t reset_timestamp;
                } imu;

                struct {
                        double position;
                } rotation, distance;

                struct {
                        V5_DeviceOpticalRgb color;
                } optical;

                struct {
                } vision;

                struct {
                        double offset_x, offset_y;
                        V5_DeviceGpsRaw position;
                } gps;

                struct {
                        emu_adi adi[8];
                } expander;
        };
};

extern struct _V5_Device emu_smart_ports[];
extern emu_adi emu_adi_ports[];
