
/*----------------------------------------------------------------------------*/
/*                                                                            */
/*    Copyright (c) Innovation First 2016, All rights reserved.               */
/*                                                                            */
/*    Module:     v5_apitypes.h                                               */
/*    Author:     James Pearman                                               */
/*    Created:    8 Nov 2016                                                  */
/*                                                                            */
/*    Revisions:  V0.1                                                        */
/*                                                                            */
/*----------------------------------------------------------------------------*/

#ifndef V5_APITYPES_H_
#define V5_APITYPES_H_

#include "stdint.h"
#include "stdbool.h"

/*-----------------------------------------------------------------------------*/
/** @file    v5_apitypes.h
  * @brief   Header for V5 API - type definitions
*//*---------------------------------------------------------------------------*/

#ifdef __cplusplus
extern "C" {
#endif

/*----------------------------------------------------------------------------*/
/** @brief      Code signature                                                */
/*----------------------------------------------------------------------------*/
/** @details
 *   The first 16 bytes of a user code binary should contain the user code
 *   signature.  For simple user code programs this will be created by the
 *   startup code in the runtime library, certain types of user code,
 *   for example a virtual machine, may override the default settings to cause
 *   the V5 system code to enable custom functionality yet TBD.
 *
 *   to override the default use a definition like this in one of the user
 *   code source files.
 *
 *   __attribute__ ((section (".boot_data"))) vcodesig vexCodeSig =
 *   { V5_SIG_MAGIC,
 *     V5_SIG_TYPE_USER,
 *     V5_SIG_OWNER_VEX,
 *     V5_SIG_OPTIONS_NONE
 *   };
 *
 *   V5_SIG_MAGIC must be set, other fields and their behavior are TBD
 *
 *   vexCodeSig is defined as "weak" and this definition will override it.
 */
typedef struct  __attribute__ ((__packed__)) _vcodesig {
  uint32_t  magic;                        // Magic, must be 'VXV5' 0x35565856 le
  uint32_t  type;                         // program type
  uint32_t  owner;                        // program originator
  uint32_t  options;                      // program options
} vcodesig ;

//
#define V5_SIG_MAGIC            0x35585658
#define V5_SIG_TYPE_USER        0
#define V5_SIG_OWNER_SYS        0
#define V5_SIG_OWNER_VEX        1
#define V5_SIG_OWNER_PARTNER    2
#define V5_SIG_OPTIONS_NONE     0
#define V5_SIG_OPTIONS_INDG     (1 << 0)   // Invert default graphics colors
#define V5_SIG_OPTIONS_EXIT     (1 << 1)   // Kill threads when main exits
#define V5_SIG_OPTIONS_THDG     (1 << 2)   // Invert graphics based on theme

#define V5_CODE_SIG( type, owner, options ) \
vcodesig  vexCodeSig  __attribute__ ((section (".boot_data"))) = \
 { V5_SIG_MAGIC, type, owner, options };

/*----------------------------------------------------------------------------*/
/** @brief      Structures used by system time functions                      */
/*----------------------------------------------------------------------------*/

// Time
struct time {
  uint8_t   ti_hour;                      /// Hours
  uint8_t   ti_min;                       /// Minutes
  uint8_t   ti_sec;                       /// Seconds
  uint8_t   ti_hund;                      /// Hundredths of seconds
};

// Time
struct date {
  uint16_t  da_year;                      /// Year - 1980
  uint8_t   da_day;                       /// Day of the month
  uint8_t   da_mon;                       /// Month (1 = Jan)
};
// DEVICE IDs

/*----------------------------------------------------------------------------*/
/** @brief      V5 Device type definitions                                    */
/*----------------------------------------------------------------------------*/
// TODO
// Do we really want to include the declare.h file ??
typedef enum {
  kDeviceTypeNoSensor        = 0,
  kDeviceTypeMotorSensor     = 2,
  kDeviceTypeLedSensor       = 3,
  kDeviceTypeAbsEncSensor    = 4,
  kDeviceTypeCrMotorSensor   = 5,
  kDeviceTypeImuSensor       = 6,
  kDeviceTypeRangeSensor     = 7, // obsolete
  kDeviceTypeDistanceSensor  = 7,
  kDeviceTypeRadioSensor     = 8,
  kDeviceTypeTetherSensor    = 9,
  kDeviceTypeBrainSensor     = 10,
  kDeviceTypeVisionSensor    = 11,
  kDeviceTypeAdiSensor       = 12,
  kDeviceTypeRes1Sensor      = 13,
  kDeviceTypeRes2Sensor      = 14,
  kDeviceTypeRes3Sensor      = 15,
  kDeviceTypeOpticalSensor   = 16,
  kDeviceTypeMagnetSensor    = 17,
  kDeviceTypeGpsSensor       = 20,
  kDeviceTypeBumperSensor    = 0x40,
  kDeviceTypeGyroSensor      = 0x46,
  kDeviceTypeSonarSensor     = 0x47,
  kDeviceTypeGenericSensor   = 128,
  kDeviceTypeGenericSerial   = 129,
  kDeviceTypeUndefinedSensor = 255
} V5_DeviceType;


// Opaque pointer to V5 device type structure
typedef struct _V5_Device * V5_DeviceT;

// Some ports will be virtual, some physical
#define V5_MAX_DEVICE_PORTS   32
typedef V5_DeviceType V5_DeviceTypeBuffer[V5_MAX_DEVICE_PORTS];

/*----------------------------------------------------------------------------*/
/** @brief      V5 Controller definitions                                     */
/*----------------------------------------------------------------------------*/
//
// March 2018, obsolete types now removed
// additional enum for final V5 controller labels added
//
typedef enum _V5_ControllerIndex {
    AnaLeftX    =   0,
    AnaLeftY,
    AnaRightX,
    AnaRightY,
    AnaSpare1,
    AnaSpare2,

    Button5U,
    Button5D,
    Button6U,
    Button6D,
    Button7U,
    Button7D,
    Button7L,
    Button7R,
    Button8U,
    Button8D,
    Button8L,
    Button8R,

    ButtonSEL,

    BatteryLevel,

    ButtonAll,
    Flags,
    BatteryCapacity,

    // Final V5 names
    Axis1       = AnaRightX,
    Axis2       = AnaRightY,
    Axis3       = AnaLeftY,
    Axis4       = AnaLeftX,

    ButtonL1    = Button5U,
    ButtonL2    = Button5D,
    ButtonR1    = Button6U,
    ButtonR2    = Button6D,

    ButtonUp    = Button7U,
    ButtonDown  = Button7D,
    ButtonLeft  = Button7L,
    ButtonRight = Button7R,

    ButtonX     = Button8U,
    ButtonB     = Button8D,
    ButtonY     = Button8L,
    ButtonA     = Button8R
} V5_ControllerIndex;

typedef enum _V5_ControllerStatus {
    kV5ControllerOffline = 0,
    kV5ControllerTethered,
    kV5ControllerVexnet
} V5_ControllerStatus;

typedef enum _V5_ControllerId {
    kControllerMaster = 0,
    kControllerPartner
} V5_ControllerId;

/*----------------------------------------------------------------------------*/
/** @brief      V5 LED device definitions                                     */
/*----------------------------------------------------------------------------*/
// The LED is an obsolete sensor but we will leave as it's available on the Dev
// systems
typedef enum _V5_DeviceLedColor {
    kLedColorBlack   = 0,
    kLedColorRed     = 0xFF0000,
    kLedColorGreen   = 0x00FF00,
    kLedColorBlue    = 0x0000FF,
    kLedColorYellow  = 0xFFFF00,
    kLedColorCyan    = 0x00FFFF,
    kLedColorMagenta = 0xFF00FF,
    kLedColorWhite   = 0xFFFFFF
} V5_DeviceLedColor;

/*----------------------------------------------------------------------------*/
/** @brief      V5 ADI device definitions                                     */
/*----------------------------------------------------------------------------*/
typedef enum _V5_AdiPortConfiguration {
    kAdiPortTypeAnalogIn = 0,
    kAdiPortTypeAnalogOut,
    kAdiPortTypeDigitalIn,
    kAdiPortTypeDigitalOut,

    kAdiPortTypeSmartButton,
    kAdiPortTypeSmartPot,

    kAdiPortTypeLegacyButton,
    kAdiPortTypeLegacyPotentiometer,
    kAdiPortTypeLegacyLineSensor,
    kAdiPortTypeLegacyLightSensor,
    kAdiPortTypeLegacyGyro,
    kAdiPortTypeLegacyAccelerometer,

    kAdiPortTypeLegacyServo,
    kAdiPortTypeLegacyPwm,

    kAdiPortTypeQuadEncoder,
    kAdiPortTypeSonar,

    kAdiPortTypeLegacyPwmSlew,

    kAdiPortTypeUndefined = 255
} V5_AdiPortConfiguration;

// ADI sensor has 8 ports
#define V5_ADI_PORT_NUM     8

/*----------------------------------------------------------------------------*/
/** @brief      V5 Bumper switch device definitions                           */
/*----------------------------------------------------------------------------*/
typedef enum _V5_DeviceBumperState {
    kBumperReleased  = 0,                 /// Switch pressed
    kBumperPressed   = 1                  /// Switch released
} V5_DeviceBumperState;

/*----------------------------------------------------------------------------*/
/** @brief      V5 Motor definitions                                          */
/*----------------------------------------------------------------------------*/
// Most of this is still TBD and is currently based on the IQ implementation
typedef enum  {
    kMotorControlModeOFF         = 0,     /// Motor is off and in coast mode
    kMotorControlModeBRAKE       = 1,     /// Motor is off and in brake mode
    kMotorControlModeHOLD        = 2,     /// Motor is holding at current position
    kMotorControlModeSERVO       = 3,     /// Motor is in "Servo" mode. Move to position and hold at that position
    kMotorControlModePROFILE     = 4,     /// Motor moves to set position and stops.
    kMotorControlModeVELOCITY    = 5,     /// Motor is unlimited movement at set 'velocity'
    kMotorControlModeUNDEFINED   = 6      ///
} V5MotorControlMode;

typedef enum _V5_MotorBrakeMode {
    kV5MotorBrakeModeCoast       = 0,     /// Motor will coast when stopped
    kV5MotorBrakeModeBrake       = 1,     /// Motor will brake when stopped
    kV5MotorBrakeModeHold        = 2      /// Motor will hold position when stopped
} V5MotorBrakeMode;

typedef enum  {
    kMotorEncoderDegrees         = 0,     /// degrees Encoder Count Mode
    kMotorEncoderRotations       = 1,     /// rotations Encoder Count Mode
    kMotorEncoderCounts          = 2      /// Raw Encoder Count Mode
} V5MotorEncoderUnits;

typedef enum _V5MotorGearset {
  kMotorGearSet_36 = 0,                   /// 36:1 gear set, torque
  kMotorGearSet_18 = 1,                   /// 18:1 gear set, speed
  kMotorGearSet_06 = 2                    /// 6:1 gear set, high speed
} V5MotorGearset;

// This is for 36:1 gear set
#define V5_MOTOR_COUNTS_PER_ROT  1800.0

//
// preliminary, used for position and velocity
//
typedef struct __attribute__ ((__packed__)) _V5_DeviceMotorPid {
	uint8_t   kf;
	uint8_t   kp;
	uint8_t   ki;
	uint8_t   kd;
	uint8_t   filter;
	uint8_t   pad1;
	uint16_t  limit;
	uint8_t   threshold;
	uint8_t   loopspeed;
	uint8_t   pad2[2];
} V5_DeviceMotorPid;

/*----------------------------------------------------------------------------*/
/** @brief      V5 Vision sensor definitions                                  */
/*----------------------------------------------------------------------------*/

// subject to change
typedef enum {
  kVisionModeNormal      = 0,
  kVisionModeMixed       = 1,
  kVisionModeLineDetect  = 2,
  kVisionTypeTest        = 3
} V5VisionMode;

typedef enum {
  kVisionTypeNormal      = 0,
  kVisionTypeColorCode   = 1,
  kVisionTypeLineDetect  = 2
} V5VisionBlockType;

// White balance
typedef enum {
  kVisionWBNormal       = 0,
  kVisionWBStart        = 1,
  kVisionWBManual       = 2
} V5VisionWBMode;

// LED control, from V5 or by the vision sensor
typedef enum {
  kVisionLedModeAuto    = 0,
  kVisionLedModeManual  = 1
} V5VisionLedMode;

// Wifi mode
typedef enum {
  kVisionWifiModeOff    = 0,
  kVisionWifiModeOn     = 1
} V5VisionWifiMode;

// signature should be 40 bytes and is the same data
// as sent to the vision sensor

// if bit0 is now set you know signature is read back
#define VISION_SIG_FLAG_STATUS    (1<<0)

typedef struct __attribute__ ((__packed__)) _V5_DeviceVisionSignature {
    uint8_t             id;
    uint8_t             flags;
    uint8_t             pad[2];
    float               range;
    int32_t             uMin;
    int32_t             uMax;
    int32_t             uMean;
    int32_t             vMin;
    int32_t             vMax;
    int32_t             vMean;
    uint32_t            mRgb;
    uint32_t            mType;
} V5_DeviceVisionSignature;

typedef struct __attribute__ ((__packed__)) _V5_DeviceVisionObject {
    uint16_t            signature;        /// block signature
    V5VisionBlockType   type;             /// block type
    uint16_t            xoffset;          /// left side of block
    uint16_t            yoffset;          /// top of block
    uint16_t            width;            /// width of block
    uint16_t            height;           /// height of block
    uint16_t            angle;            /// angle of CC block in 0.1 deg units
} V5_DeviceVisionObject;

// Color structure, used for LED and white balance
typedef struct __attribute__ ((__packed__)) _V5_DeviceVisionRgb {
    uint8_t             red;
    uint8_t             green;
    uint8_t             blue;
    uint8_t             brightness;    /// not used yet
} V5_DeviceVisionRgb;

/*----------------------------------------------------------------------------*/
/** @brief      V5 IMU sensor definitions                                     */
/*----------------------------------------------------------------------------*/

// Quaternion data from IMU
typedef struct __attribute__ ((__packed__)) _V5_DeviceImuQuaternion {
    double              a;
    double              b;
    double              c;
    double              d;
} V5_DeviceImuQuaternion;

// Attitude data from IMU
typedef struct __attribute__ ((__packed__)) _V5_DeviceImuAttitude {
    double              pitch;  // x
    double              roll;   // y
    double              yaw;    // z
} V5_DeviceImuAttitude;

// Raw data from IMU
typedef struct __attribute__ ((__packed__)) _V5_DeviceImuRaw {
    double              x;
    double              y;
    double              z;
    double              w;
} V5_DeviceImuRaw;

// native is same direction as 3wire gyro
// clockwise is positive
typedef enum _V5ImuHeadingnMode {
    kImuHeadingNative     = 0x00,
    kImuHeadingIQ         = 0x01
} _V5ImuHeadingnMode;

// Orientation mode
typedef enum _V5ImuOrientationMode {
    kImuOrientationZUp    = 0x00,
    kImuOrientationZDown  = 0x10,
    kImuOrientationXUp    = 0x20,
    kImuOrientationXDown  = 0x30,
    kImuOrientationYUp    = 0x40,
    kImuOrientationYDown  = 0x50,
    kImuOrientationAuto   = 0x80
} V5ImuOrientationMode;

// Quaternion mode
typedef enum _V5ImuQuaternionMode {
    kImuQuaternionProcessed = 0x000,
    kImuQuaternionRaw       = 0x100
} V5ImuQuaternionMode;

/*----------------------------------------------------------------------------*/
/** @brief      V5 Color sensor definitions                                   */
/*----------------------------------------------------------------------------*/
// for raw crgb data
typedef struct _V5_DeviceOpticalRaw {
    uint16_t     clear;
    uint16_t     red;
    uint16_t     green;
    uint16_t     blue;
} V5_DeviceOpticalRaw;

typedef struct _V5_DeviceOpticalRgb {
    double       red;
    double       green;
    double       blue;
    double       brightness;
} V5_DeviceOpticalRgb;

// gesture data
typedef struct _V5_DeviceOpticalGesture {
    uint8_t      udata;
    uint8_t      ddata;
    uint8_t      ldata;
    uint8_t      rdata;
    uint8_t      type;
    uint8_t      pad;
    uint16_t     count;
    uint32_t     time;
} V5_DeviceOpticalGesture;

/*----------------------------------------------------------------------------*/
/** @brief      V5 magnet definitions                                         */
/*----------------------------------------------------------------------------*/
typedef enum _V5_DeviceMagnetDuration {
    kMagnetDurationShort,
    kMagnetDurationMedium,
    kMagnetDurationLong,
    kMagnetDurationExtraLong,
} V5_DeviceMagnetDuration;

/*----------------------------------------------------------------------------*/
/** @brief      V5 gps definitions                                            */
/*----------------------------------------------------------------------------*/
// Quaternion data from GPS
typedef struct __attribute__ ((__packed__)) _V5_DeviceGpsQuaternion {
    double              a;
    double              b;
    double              c;
    double              d;
} V5_DeviceGpsQuaternion;

// Attitude data from GPS, collect all useful info into this structure now
typedef struct __attribute__ ((__packed__)) _V5_DeviceGpsAttitude {
    double              pitch;  // x
    double              roll;   // y
    double              yaw;    // z

    // spacial position on the field
    double              position_x;
    double              position_y;
    double              position_z;

    // alternative roll, pitch and yaw
    double              az;
    double              el;
    double              rot;
} V5_DeviceGpsAttitude;

// Raw data from GPS
typedef struct __attribute__ ((__packed__)) _V5_DeviceGpsRaw {
    double              x;
    double              y;
    double              z;
    double              w;
} V5_DeviceGpsRaw;

/*----------------------------------------------------------------------------*/
/** @brief      V5 AI Camera definitions                                      */
/*----------------------------------------------------------------------------*/

// detected object
typedef struct __attribute__ (( __packed__ )) _V5_DeviceAicamObject {
    int16_t         type;
    int16_t         xpos;
    int16_t         ypos;
    int16_t         zpos;
    int16_t         width;
    int16_t         height;
} V5_DeviceAicamObject;

/*----------------------------------------------------------------------------*/
/** @brief      V5 SD File system definitions                                 */
/*----------------------------------------------------------------------------*/

// Opaque pointer (FIL *) to file structure
typedef   void          FIL;

// seek flags
// changed, Mar 6 2018 to be more consistent with stdio.h
#define   FS_SEEK_SET     0
#define   FS_SEEK_CUR     1
#define   FS_SEEK_END     2

// new 1.0.13, file status
#define   FS_FILE_EXIST   1
#define   FS_FILE_DIR     2

// File function return code (FRESULT)
typedef enum {
    FR_OK = 0U,                           /// (0) Succeeded
    FR_DISK_ERR,                          /// (1) A hard error occurred in the low level disk I/O layer
    FR_INT_ERR,                           /// (2) Assertion failed
    FR_NOT_READY,                         /// (3) The physical drive cannot work
    FR_NO_FILE,                           /// (4) Could not find the file
    FR_NO_PATH,                           /// (5) Could not find the path
    FR_INVALID_NAME,                      /// (6) The path name format is invalid
    FR_DENIED,                            /// (7) Access denied due to prohibited access or directory full
    FR_EXIST,                             /// (8) Access denied due to prohibited access
    FR_INVALID_OBJECT,                    /// (9) The file/directory object is invalid
    FR_WRITE_PROTECTED,                   /// (10) The physical drive is write protected
    FR_INVALID_DRIVE,                     /// (11) The logical drive number is invalid
    FR_NOT_ENABLED,                       /// (12) The volume has no work area
    FR_NO_FILESYSTEM,                     /// (13) There is no valid FAT volume
    FR_MKFS_ABORTED,                      /// (14) The f_mkfs() aborted due to any parameter error
    FR_TIMEOUT,                           /// (15) Could not get a grant to access the volume within defined period
    FR_LOCKED,                            /// (16) The operation is rejected according to the file sharing policy
    FR_NOT_ENOUGH_CORE,                   /// (17) LFN working buffer could not be allocated
    FR_TOO_MANY_OPEN_FILES,               /// (18) Number of open files > _FS_SHARE
    FR_INVALID_PARAMETER                  /// (19) Given parameter is invalid
} FRESULT;

/*----------------------------------------------------------------------------*/
/** @brief      V5 touch events                                               */
/*----------------------------------------------------------------------------*/
typedef enum _touchEvent {
    kTouchEventRelease,
    kTouchEventPress,
    kTouchEventPressAuto
} V5_TouchEvent;

typedef struct _V5_TouchStatus {
    V5_TouchEvent lastEvent;
    int16_t       lastXpos;
    int16_t       lastYpos;
    int32_t       pressCount;
    int32_t       releaseCount;
} V5_TouchStatus;

/*----------------------------------------------------------------------------*/
/** @brief      V5 competition status bits                                    */
/*----------------------------------------------------------------------------*/

#define V5_COMP_BIT_EBL     1   // if set then robot disabled
#define V5_COMP_BIT_MODE    2   // if set then robot in autonomous
#define V5_COMP_BIT_COMP    4   // if set then either comp switch or field control connected
#define V5_COMP_BIT_GAME    8   // if set then field control connected

/*----------------------------------------------------------------------------*/
/** @brief   structure holding image info - used by bmp/png read code         */
/*----------------------------------------------------------------------------*/
// data must point to suitable buffer now
typedef struct __attribute__ ((__packed__)) _v5_image {
    uint16_t  width;
    uint16_t  height;
    uint32_t *data;
    uint32_t *p;
} v5_image;

#define SYSTEM_DISPLAY_WIDTH       		480
#define SYSTEM_DISPLAY_HEIGHT      		272

#ifdef __cplusplus
}
#endif
#endif /* V5_APITYPES_H_*/