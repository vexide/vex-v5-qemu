/*----------------------------------------------------------------------------*/
/*                                                                            */
/*    Copyright (c) Innovation First 2016, All rights reserved.               */
/*                                                                            */
/*    Module:     v5_apiuser.h                                                */
/*    Author:     James Pearman                                               */
/*    Created:    8 Nov 2016                                                  */
/*                                                                            */
/*    Revisions:  V0.1                                                        */
/*                                                                            */
/*----------------------------------------------------------------------------*/

#ifndef V5_APIUSER_H_
#define V5_APIUSER_H_

#include "stdint.h"
#include "stdbool.h"
#include "v5_apitypes.h"

/*----------------------------------------------------------------------------*/
/** @file    v5_apiuser.h
  * @brief   Header for V5 API device wrapper functions
*//*--------------------------------------------------------------------------*/

#ifdef __cplusplus
extern "C" {
#endif

void                vexDelay( uint32_t timems );

void                vexLedSet( uint32_t index, V5_DeviceLedColor value );
void                vexLedRgbSet( uint32_t index, uint32_t color);
V5_DeviceLedColor   vexLedGet( uint32_t index );
uint32_t            vexLedRgbGet( uint32_t index );

void                vexAdiPortConfigSet( uint32_t index, uint32_t port, V5_AdiPortConfiguration type );
V5_AdiPortConfiguration vexAdiPortConfigGet( uint32_t index, uint32_t port );
void                vexAdiValueSet( uint32_t index, uint32_t port, int32_t value );
int32_t             vexAdiValueGet( uint32_t index, uint32_t port );

V5_DeviceBumperState  vexBumperGet( uint32_t index );

void                vexGyroReset( uint32_t index );
double              vexGyroHeadingGet( uint32_t index );
double              vexGyroDegreesGet( uint32_t index );

int32_t             vexSonarValueGet( uint32_t index );

int32_t             vexGenericValueGet( uint32_t index );

void                vexMotorVelocitySet( uint32_t index, int32_t velocity );
void                vexMotorVelocityUpdate( uint32_t index, int32_t velocity );
void                vexMotorVoltageSet( uint32_t index, int32_t value );
int32_t             vexMotorVelocityGet( uint32_t index );
int32_t             vexMotorDirectionGet( uint32_t index );
double              vexMotorActualVelocityGet( uint32_t index );
void                vexMotorModeSet( uint32_t index, V5MotorControlMode mode );
V5MotorControlMode  vexMotorModeGet( uint32_t index );
void                vexMotorPwmSet( uint32_t index, int32_t value );
int32_t             vexMotorPwmGet( uint32_t index );
void                vexMotorCurrentLimitSet( uint32_t index, int32_t value );
int32_t             vexMotorCurrentLimitGet( uint32_t index );
void                vexMotorVoltageLimitSet( uint32_t index, int32_t value );
int32_t             vexMotorVoltageLimitGet( uint32_t index );
void                vexMotorPositionPidSet( uint32_t index, V5_DeviceMotorPid *pid );
void                vexMotorVelocityPidSet( uint32_t index, V5_DeviceMotorPid *pid );
int32_t             vexMotorCurrentGet( uint32_t index );
int32_t             vexMotorVoltageGet( uint32_t index );
double              vexMotorPowerGet( uint32_t index );
double              vexMotorTorqueGet( uint32_t index );
double              vexMotorEfficiencyGet( uint32_t index );
double              vexMotorTemperatureGet( uint32_t index );
bool                vexMotorOverTempFlagGet( uint32_t index );
bool                vexMotorCurrentLimitFlagGet( uint32_t index );
uint32_t            vexMotorFaultsGet( uint32_t index );
bool                vexMotorZeroVelocityFlagGet( uint32_t index );
bool                vexMotorZeroPositionFlagGet( uint32_t index );
uint32_t            vexMotorFlagsGet( uint32_t index );
void                vexMotorReverseFlagSet( uint32_t index, bool value );
bool                vexMotorReverseFlagGet( uint32_t index );
void                vexMotorEncoderUnitsSet( uint32_t index, V5MotorEncoderUnits units );
V5MotorEncoderUnits vexMotorEncoderUnitsGet( uint32_t index );
void                vexMotorBrakeModeSet( uint32_t index, V5MotorBrakeMode mode );
V5MotorBrakeMode    vexMotorBrakeModeGet( uint32_t index );
void                vexMotorPositionSet( uint32_t index, double position );
double              vexMotorPositionGet( uint32_t index );
int32_t             vexMotorPositionRawGet( uint32_t index, uint32_t *timestamp );
void                vexMotorPositionReset( uint32_t index );
double              vexMotorTargetGet( uint32_t index );
void                vexMotorServoTargetSet( uint32_t index, double position );
void                vexMotorAbsoluteTargetSet( uint32_t index, double position, int32_t velocity );
void                vexMotorRelativeTargetSet( uint32_t index, double position, int32_t velocity );
void                vexMotorGearingSet( uint32_t index, V5MotorGearset value );
V5MotorGearset      vexMotorGearingGet( uint32_t index );
void                vexMotorExternalProfileSet( uint32_t index, double position, int32_t velocity );

void                vexVisionModeSet( uint32_t index, V5VisionMode mode );
V5VisionMode        vexVisionModeGet( uint32_t index );
int32_t             vexVisionObjectCountGet( uint32_t index );
int32_t             vexVisionObjectGet( uint32_t index, uint32_t indexObj, V5_DeviceVisionObject *pObject );
void                vexVisionSignatureSet( uint32_t index, V5_DeviceVisionSignature *pSignature );
bool                vexVisionSignatureGet( uint32_t index, uint32_t id, V5_DeviceVisionSignature *pSignature );
void                vexVisionBrightnessSet( uint32_t index, uint8_t percent );
uint8_t             vexVisionBrightnessGet( uint32_t index );
void                vexVisionWhiteBalanceModeSet( uint32_t index, V5VisionWBMode mode );
V5VisionWBMode      vexVisionWhiteBalanceModeGet( uint32_t index );
void                vexVisionWhiteBalanceSet( uint32_t index, V5_DeviceVisionRgb color );
V5_DeviceVisionRgb  vexVisionWhiteBalanceGet( uint32_t index );
void                vexVisionLedModeSet( uint32_t index, V5VisionLedMode mode );
V5VisionLedMode     vexVisionLedModeGet( uint32_t index );
void                vexVisionLedBrigntnessSet( uint32_t index, uint8_t percent );
uint8_t             vexVisionLedBrigntnessGet( uint32_t index );
void                vexVisionLedColorSet( uint32_t index, V5_DeviceVisionRgb color);
V5_DeviceVisionRgb  vexVisionLedColorGet( uint32_t index );
void                vexVisionWifiModeSet( uint32_t index, V5VisionWifiMode mode );
V5VisionWifiMode    vexVisionWifiModeGet( uint32_t index );

void                vexImuReset( uint32_t index );
double              vexImuHeadingGet( uint32_t index );
double              vexImuDegreesGet( uint32_t index );
void                vexImuQuaternionGet( uint32_t index, V5_DeviceImuQuaternion *data );
void                vexImuAttitudeGet( uint32_t index, V5_DeviceImuAttitude *data );
void                vexImuRawGyroGet( uint32_t index, V5_DeviceImuRaw *data );
void                vexImuRawAccelGet( uint32_t index, V5_DeviceImuRaw *data );
uint32_t            vexImuStatusGet( uint32_t index );
void                vexImuModeSet( uint32_t index, uint32_t mode );
uint32_t            vexImuModeGet( uint32_t index );
void                vexImuDataRateSet( uint32_t index, uint32_t rate );

int32_t             vexRangeValueGet( uint32_t index );

void                vexAbsEncReset( uint32_t index );
void                vexAbsEncPositionSet( uint32_t index, int32_t position );
int32_t             vexAbsEncPositionGet( uint32_t index );
int32_t             vexAbsEncVelocityGet( uint32_t index );
int32_t             vexAbsEncAngleGet( uint32_t index );
void                vexAbsEncReverseFlagSet( uint32_t index, bool value );
bool                vexAbsEncReverseFlagGet( uint32_t index );
uint32_t            vexAbsEncStatusGet( uint32_t index );
void                vexAbsEncDataRateSet( uint32_t index, uint32_t rate );

double              vexOpticalHueGet( uint32_t index );
double              vexOpticalSatGet( uint32_t index );
double              vexOpticalBrightnessGet( uint32_t index );
int32_t             vexOpticalProximityGet( uint32_t index );
void                vexOpticalRgbGet( uint32_t index, V5_DeviceOpticalRgb *data );
void                vexOpticalLedPwmSet( uint32_t index, int32_t value );
int32_t             vexOpticalLedPwmGet( uint32_t index );
uint32_t            vexOpticalStatusGet( uint32_t index );
void                vexOpticalRawGet( uint32_t index, V5_DeviceOpticalRaw *data );
void                vexOpticalModeSet( uint32_t index, uint32_t mode );
uint32_t            vexOpticalModeGet( uint32_t index );
uint32_t            vexOpticalGestureGet( uint32_t index, V5_DeviceOpticalGesture *pData );
void                vexOpticalGestureEnable( uint32_t index );
void                vexOpticalGestureDisable( uint32_t index );
int32_t             vexOpticalProximityThreshold( uint32_t index, int32_t value );
void                vexOpticalIntegrationTimeSet( uint32_t index, double timems );
double              vexOpticalIntegrationTimeGet( uint32_t index );

void                vexMagnetPowerSet( uint32_t index, int32_t value, int32_t time );
int32_t             vexMagnetPowerGet( uint32_t index );
void                vexMagnetPickup( uint32_t index, V5_DeviceMagnetDuration duration );
void                vexMagnetDrop( uint32_t index, V5_DeviceMagnetDuration duration );
double              vexMagnetTemperatureGet( uint32_t index );
double              vexMagnetCurrentGet( uint32_t index );
uint32_t            vexMagnetStatusGet( uint32_t index );

uint32_t            vexDistanceDistanceGet( uint32_t index );
uint32_t            vexDistanceConfidenceGet( uint32_t index );
int32_t             vexDistanceObjectSizeGet( uint32_t index );
double              vexDistanceObjectVelocityGet( uint32_t index );
uint32_t            vexDistanceStatusGet( uint32_t index );

void                vexGpsReset( uint32_t index );
double              vexGpsHeadingGet( uint32_t index );
double              vexGpsDegreesGet( uint32_t index );
void                vexGpsQuaternionGet( uint32_t index, V5_DeviceGpsQuaternion *data );
void                vexGpsAttitudeGet( uint32_t index, V5_DeviceGpsAttitude *data, bool bRaw );
void                vexGpsRawGyroGet( uint32_t index, V5_DeviceGpsRaw *data );
void                vexGpsRawAccelGet( uint32_t index, V5_DeviceGpsRaw *data );
uint32_t            vexGpsStatusGet( uint32_t index );
void                vexGpsModeSet( uint32_t index, uint32_t mode );
uint32_t            vexGpsModeGet( uint32_t index );
void                vexGpsDataRateSet( uint32_t index, uint32_t rate );
void                vexGpsOriginSet( uint32_t index, double ox, double oy );
void                vexGpsOriginGet( uint32_t index, double *ox, double *oy );
void                vexGpsRotationSet( uint32_t index, double value );
double              vexGpsRotationGet( uint32_t index );
void                vexGpsInitialPositionSet( uint32_t index, double initial_x, double initial_y, double initial_rotation );
double              vexGpsErrorGet( uint32_t index );

// Generic serial port comms to any device
void                vexGenericSerialEnable( uint32_t index, int32_t options );
void                vexGenericSerialBaudrate( uint32_t index, int32_t baudrate );
int32_t             vexGenericSerialWriteChar( uint32_t index, uint8_t c );
int32_t             vexGenericSerialWriteFree( uint32_t index );
int32_t             vexGenericSerialTransmit( uint32_t index, uint8_t *buffer, int32_t length );
int32_t             vexGenericSerialReadChar( uint32_t index );
int32_t             vexGenericSerialPeekChar( uint32_t index );
int32_t             vexGenericSerialReceiveAvail( uint32_t index );
int32_t             vexGenericSerialReceive( uint32_t index, uint8_t *buffer, int32_t length );
void                vexGenericSerialFlush( uint32_t index );


#ifdef __cplusplus
}
#endif
#endif /* V5_APIUSER_H_ */
