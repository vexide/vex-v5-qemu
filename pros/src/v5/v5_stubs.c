#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

#include "v5_api.h"

void vexBackgroundProcessing() {}

// Console output
int32_t vexDebug(char const* fmt, ...) {
	return 0;
}

int32_t vex_printf(char const* fmt, ...) {
	va_list va;
	va_start(va, fmt);
	return vprintf(fmt, va);
	va_end(va);
}

int32_t vex_sprintf(char* out, const char* format, ...) {
	va_list va;
	va_start(va, format);
	return vsprintf(out, format, va);
	va_end(va);
}

int32_t vex_snprintf(char* out, uint32_t max_len, const char* format, ...) {
	va_list va;
	va_start(va, format);
	return vsnprintf(out, max_len, format, va);
	va_end(va);
}

int32_t vex_vsprintf(char* out, const char* format, va_list args) {
	return vsprintf(out, format, args);
}

int32_t vex_vsnprintf(char* out, uint32_t max_len, const char* format, va_list args) {
	return vsnprintf(out, max_len, format, args);
}

// system
uint32_t vexSystemTimeGet() {}

void vexGettime(struct time* pTime) {}

void vexGetdate(struct date* pDate) {}

void vexSystemMemoryDump() {}

void vexSystemDigitalIO(uint32_t pin, uint32_t value) {}

uint32_t vexSystemStartupOptions() {}

__attribute((noreturn)) void vexSystemExitRequest() {
	puts("Shutting down!\n");
	exit(0);
}

uint64_t vexSystemHighResTimeGet() {}

uint64_t vexSystemPowerupTimeGet() {}

uint32_t vexSystemLinkAddrGet() {}

uint32_t vexSystemUsbStatus() {}

uint32_t vexDeviceButtonStateGet() {}

// LED sensor
void vexDeviceLedSet(V5_DeviceT device, V5_DeviceLedColor value) {}

void vexDeviceLedRgbSet(V5_DeviceT device, uint32_t color) {}

V5_DeviceLedColor vexDeviceLedGet(V5_DeviceT device) {}

uint32_t vexDeviceLedRgbGet(V5_DeviceT device) {}

// ADI sensor
void vexDeviceAdiPortConfigSet(V5_DeviceT device, uint32_t port, V5_AdiPortConfiguration type) {}

V5_AdiPortConfiguration vexDeviceAdiPortConfigGet(V5_DeviceT device, uint32_t port) {}

void vexDeviceAdiValueSet(V5_DeviceT device, uint32_t port, int32_t value) {}

int32_t vexDeviceAdiValueGet(V5_DeviceT device, uint32_t port) {}

// Bumper switch
V5_DeviceBumperState vexDeviceBumperGet(V5_DeviceT device) {}

// Gyro - obsolete, there is currently no dedicated gyro
void vexDeviceGyroReset(V5_DeviceT device) {}

double vexDeviceGyroHeadingGet(V5_DeviceT device) {}

double vexDeviceGyroDegreesGet(V5_DeviceT device) {}

// Sonar - obsolete, there is currently no dedicated gyro
int32_t vexDeviceSonarValueGet(V5_DeviceT device) {}

// Generic sensor - (who knows !)
int32_t vexDeviceGenericValueGet(V5_DeviceT device) {}

// Vision sensor
void vexDeviceVisionModeSet(V5_DeviceT device, V5VisionMode mode) {}

V5VisionMode vexDeviceVisionModeGet(V5_DeviceT device) {}

int32_t vexDeviceVisionObjectCountGet(V5_DeviceT device) {}

int32_t vexDeviceVisionObjectGet(V5_DeviceT device, uint32_t indexObj, V5_DeviceVisionObject* pObject) {}

void vexDeviceVisionSignatureSet(V5_DeviceT device, V5_DeviceVisionSignature* pSignature) {}

bool vexDeviceVisionSignatureGet(V5_DeviceT device, uint32_t id, V5_DeviceVisionSignature* pSignature) {}

void vexDeviceVisionBrightnessSet(V5_DeviceT device, uint8_t percent) {}

uint8_t vexDeviceVisionBrightnessGet(V5_DeviceT device) {}

void vexDeviceVisionWhiteBalanceModeSet(V5_DeviceT device, V5VisionWBMode mode) {}

V5VisionWBMode vexDeviceVisionWhiteBalanceModeGet(V5_DeviceT device) {}

void vexDeviceVisionWhiteBalanceSet(V5_DeviceT device, V5_DeviceVisionRgb color) {}

V5_DeviceVisionRgb vexDeviceVisionWhiteBalanceGet(V5_DeviceT device) {}

void vexDeviceVisionLedModeSet(V5_DeviceT device, V5VisionLedMode mode) {}

V5VisionLedMode vexDeviceVisionLedModeGet(V5_DeviceT device) {}

void vexDeviceVisionLedBrigntnessSet(V5_DeviceT device, uint8_t percent) {}

uint8_t vexDeviceVisionLedBrigntnessGet(V5_DeviceT device) {}

void vexDeviceVisionLedColorSet(V5_DeviceT device, V5_DeviceVisionRgb color) {}

V5_DeviceVisionRgb vexDeviceVisionLedColorGet(V5_DeviceT device) {}

void vexDeviceVisionWifiModeSet(V5_DeviceT device, V5VisionWifiMode mode) {}

V5VisionWifiMode vexDeviceVisionWifiModeGet(V5_DeviceT device) {}

// Rangefinder/Lidar - actual API to be determined
int32_t vexDeviceRangeValueGet(V5_DeviceT device) {}

// Absolute encoder
void vexDeviceAbsEncReset(V5_DeviceT device) {}

void vexDeviceAbsEncPositionSet(V5_DeviceT device, int32_t position) {}

int32_t vexDeviceAbsEncPositionGet(V5_DeviceT device) {}

int32_t vexDeviceAbsEncVelocityGet(V5_DeviceT device) {}

int32_t vexDeviceAbsEncAngleGet(V5_DeviceT device) {}

void vexDeviceAbsEncReverseFlagSet(V5_DeviceT device, bool value) {}

bool vexDeviceAbsEncReverseFlagGet(V5_DeviceT device) {}

uint32_t vexDeviceAbsEncStatusGet(V5_DeviceT device) {}

void vexDeviceAbsEncDataRateSet(V5_DeviceT device, uint32_t rate) {}

// Optical/color sensor
double vexDeviceOpticalHueGet(V5_DeviceT device) {}

double vexDeviceOpticalSatGet(V5_DeviceT device) {}

double vexDeviceOpticalBrightnessGet(V5_DeviceT device) {}

int32_t vexDeviceOpticalProximityGet(V5_DeviceT device) {}

void vexDeviceOpticalRgbGet(V5_DeviceT device, V5_DeviceOpticalRgb* data) {}

void vexDeviceOpticalLedPwmSet(V5_DeviceT device, int32_t value) {}

int32_t vexDeviceOpticalLedPwmGet(V5_DeviceT device) {}

uint32_t vexDeviceOpticalStatusGet(V5_DeviceT device) {}

void vexDeviceOpticalRawGet(V5_DeviceT device, V5_DeviceOpticalRaw* data) {}

void vexDeviceOpticalModeSet(V5_DeviceT device, uint32_t mode) {}

uint32_t vexDeviceOpticalModeGet(V5_DeviceT device) {}

uint32_t vexDeviceOpticalGestureGet(V5_DeviceT device, V5_DeviceOpticalGesture* pData) {}

void vexDeviceOpticalGestureEnable(V5_DeviceT device) {}

void vexDeviceOpticalGestureDisable(V5_DeviceT device) {}

int32_t vexDeviceOpticalProximityThreshold(V5_DeviceT device, int32_t value) {}

void vexDeviceOpticalIntegrationTimeSet(V5_DeviceT device, double timeMs) {}

double vexDeviceOpticalIntegrationTimeGet(V5_DeviceT device) {}

// Electro magnet
void vexDeviceMagnetPowerSet(V5_DeviceT device, int32_t value, int32_t time) {}

int32_t vexDeviceMagnetPowerGet(V5_DeviceT device) {}

void vexDeviceMagnetPickup(V5_DeviceT device, V5_DeviceMagnetDuration duration) {}

void vexDeviceMagnetDrop(V5_DeviceT device, V5_DeviceMagnetDuration duration) {}

double vexDeviceMagnetTemperatureGet(V5_DeviceT device) {}

double vexDeviceMagnetCurrentGet(V5_DeviceT device) {}

uint32_t vexDeviceMagnetStatusGet(V5_DeviceT device) {}

// Distance
uint32_t vexDeviceDistanceDistanceGet(V5_DeviceT device) {}

uint32_t vexDeviceDistanceConfidenceGet(V5_DeviceT device) {}

int32_t vexDeviceDistanceObjectSizeGet(V5_DeviceT device) {}

double vexDeviceDistanceObjectVelocityGet(V5_DeviceT device) {}

uint32_t vexDeviceDistanceStatusGet(V5_DeviceT device) {}

// Gps
void vexDeviceGpsReset(V5_DeviceT device) {}

double vexDeviceGpsHeadingGet(V5_DeviceT device) {}

double vexDeviceGpsDegreesGet(V5_DeviceT device) {}

void vexDeviceGpsQuaternionGet(V5_DeviceT device, V5_DeviceGpsQuaternion* data) {}

void vexDeviceGpsAttitudeGet(V5_DeviceT device, V5_DeviceGpsAttitude* data, bool bRaw) {}

void vexDeviceGpsRawGyroGet(V5_DeviceT device, V5_DeviceGpsRaw* data) {}

void vexDeviceGpsRawAccelGet(V5_DeviceT device, V5_DeviceGpsRaw* data) {}

uint32_t vexDeviceGpsStatusGet(V5_DeviceT device) {}

void vexDeviceGpsModeSet(V5_DeviceT device, uint32_t mode) {}

uint32_t vexDeviceGpsModeGet(V5_DeviceT device) {}

void vexDeviceGpsDataRateSet(V5_DeviceT device, uint32_t rate) {}

void vexDeviceGpsOriginSet(V5_DeviceT device, double ox, double oy) {}

void vexDeviceGpsOriginGet(V5_DeviceT device, double* ox, double* oy) {}

void vexDeviceGpsRotationSet(V5_DeviceT device, double value) {}

double vexDeviceGpsRotationGet(V5_DeviceT device) {}

void vexDeviceGpsInitialPositionSet(V5_DeviceT device, double initial_x, double initial_y, double initial_rotation) {}

double vexDeviceGpsErrorGet(V5_DeviceT device) {}

// Generic serial port comms to any device
void vexDeviceGenericSerialEnable(V5_DeviceT device, int32_t options) {}

void vexDeviceGenericSerialBaudrate(V5_DeviceT device, int32_t baudrate) {}

int32_t vexDeviceGenericSerialWriteChar(V5_DeviceT device, uint8_t c) {}

int32_t vexDeviceGenericSerialWriteFree(V5_DeviceT device) {}

int32_t vexDeviceGenericSerialTransmit(V5_DeviceT device, uint8_t* buffer, int32_t length) {}

int32_t vexDeviceGenericSerialReadChar(V5_DeviceT device) {}

int32_t vexDeviceGenericSerialPeekChar(V5_DeviceT device) {}

int32_t vexDeviceGenericSerialReceiveAvail(V5_DeviceT device) {}

int32_t vexDeviceGenericSerialReceive(V5_DeviceT device, uint8_t* buffer, int32_t length) {}

void vexDeviceGenericSerialFlush(V5_DeviceT device) {}

// special use only ! Talk to James.
int32_t vexScratchMemoryPtr(void** ptr) {}

bool vexScratchMemoryLock() {}

void vexScratchMemoryUnlock() {}

// SD card
FRESULT vexFileMountSD() {}

FRESULT vexFileDirectoryGet(const char* path, char* buffer, uint32_t len) {}

FIL* vexFileOpen(const char* filename, const char* mode) {}

FIL* vexFileOpenWrite(const char* filename) {}

FIL* vexFileOpenCreate(const char* filename) {}

void vexFileClose(FIL* fdp) {}

int32_t vexFileRead(char* buf, uint32_t size, uint32_t nItems, FIL* fdp) {}

int32_t vexFileWrite(char* buf, uint32_t size, uint32_t nItems, FIL* fdp) {}

int32_t vexFileSize(FIL* fdp) {}

FRESULT vexFileSeek(FIL* fdp, uint32_t offset, int32_t whence) {}

bool vexFileDriveStatus(uint32_t drive) {}

int32_t vexFileTell(FIL* fdp) {}

void vexFileSync(FIL* fdp) {}

uint32_t vexFileStatus(const char* filename) {}

// CDC
int32_t vexSerialWriteChar(uint32_t channel, uint8_t c) {}

int32_t vexSerialWriteBuffer(uint32_t channel, uint8_t* data, uint32_t data_len) {}

int32_t vexSerialReadChar(uint32_t channel) {}

int32_t vexSerialPeekChar(uint32_t channel) {}

int32_t vexSerialWriteFree(uint32_t channel) {}

// rtos hooks
void vexSystemTimerStop() {}

void vexSystemTimerClearInterrupt() {}

int32_t vexSystemTimerReinitForRtos(uint32_t priority, void (*handler)(void* data)) {}

void vexSystemApplicationIRQHandler(uint32_t ulICCIAR) {}

int32_t vexSystemWatchdogReinitRtos() {}

uint32_t vexSystemWatchdogGet() {}

// Hooks into the interrupt system, some of these will be used by FreeRTOS
// the others are just placeholders for now
void vexSystemBoot() {}

void vexSystemUndefinedException() {}

void vexSystemFIQInterrupt() {}

void vexSystemIQRQnterrupt() {}

void vexSystemSWInterrupt() {}

void vexSystemDataAbortInterrupt() {}

void vexSystemPrefetchAbortInterrupt() {}

// touch

// system utility
uint32_t vexSystemVersion() {}

uint32_t vexStdlibVersion() {}

// get SDK version
uint32_t vexSdkVersion() {}

// duplication of prototypes in v5_util.h for use by user code
uint32_t vexStdlibVersionLinked() {}

bool vexStdlibVersionVerify() {}

void vexCompetitionControl(uint32_t data) {}

// battery
int32_t vexBatteryVoltageGet() {}

int32_t vexBatteryCurrentGet() {}

double vexBatteryTemperatureGet() {}

double vexBatteryCapacityGet() {}

int32_t vexDeviceAdiAddrLedSet(V5_DeviceT device, uint32_t port, uint32_t* pData, uint32_t nOffset, uint32_t nLength,
                               uint32_t options) {}

int32_t vexAdiAddrLedSet(uint32_t index, uint32_t port, uint32_t* pData, uint32_t nOffset, uint32_t nLength,
                         uint32_t options) {}