#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub hundredths: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Date {
    pub year: u16,
    pub day: u8,
    pub month: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorBrakeMode {
    Coast = 0, // Motor will coast when stopped
    Brake = 1, // Motor will brake when stopped
    Hold = 2,  // Motor will hold position when stopped
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorControlMode {
    Off = 0,      // Motor is off and in coast mode
    Brake = 1,    // Motor is off and in brake mode
    Hold = 2,     // Motor is holding at current position
    Servo = 3,    // Motor is in "Servo" mode. Move to position and hold at that position
    Profile = 4,  // Motor moves to set position and stops.
    Velocity = 5, // Motor is unlimited movement at set 'velocity'
    Undefined = 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorEncoderUnits {
    Degrees = 0,   // degrees Encoder Count Mode
    Rotations = 1, // rotations Encoder Count Mode
    Counts = 2,    // Raw Encoder Count Mode
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MotorGearset {
    Gearing36 = 0, // 36:1 gear set, torque
    Gearing18 = 1, // 18:1 gear set, speed
    Gearing06 = 2, // 6:1 gear set, high speed
}

#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct MotorPid {
    pub kf: u8,
    pub kp: u8,
    pub ki: u8,
    pub kd: u8,
    pub filter: u8,
    pub d1: u8,
    pub limit: u16,
    pub threshold: u8,
    pub loopspeed: u8,
    pub pad2: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MotorData {
    pub brake_mode: MotorBrakeMode,
    pub control_mode: MotorControlMode,
    pub encoder_units: MotorEncoderUnits,
    pub gearing: MotorGearset,
    pub pos_pid: *mut MotorPid,
    pub vel_pid: *mut MotorPid,
    pub velocity_target: i32,
    pub velocity_max: i32,
    pub current: i32,
    pub current_max: i32,
    pub voltage: i32,
    pub voltage_max: i32,
    pub position: f64,
    pub position_target: f64,
    pub velocity: f64,
    pub power: f64,
    pub torque: f64,
    pub efficiency: f64,
    pub temperature: f64,
    pub faults: u32,
    pub flags: u8,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuRaw {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ImuOrientationMode {
    ZUp = 0x00,
    ZDown = 0x10,
    XUp = 0x20,
    XDown = 0x30,
    YUp = 0x40,
    YDown = 0x50,
    Auto = 0x80,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ImuData {
    pub orientation: ImuOrientationMode,
    pub rotation: ImuRaw,
    pub acceleration: ImuRaw,
    pub reset_timestamp: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GenericPositionData {
    pub position: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalData {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub brightness: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdiExpanderData {
    pub adi_types: [AdiPortConfiguration; 8],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpsData {
    pub offset_x: f64,
    pub offset_y: f64,

    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DeviceData {
    pub motor: MotorData,
    pub imu: ImuData,
    pub rotation: GenericPositionData,
    pub distance: GenericPositionData,
    pub optical: OpticalData,
    pub vision: (),
    pub gps: GpsData,
    pub adi_expander: AdiExpanderData,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct V5Device {
    pub port: u8,
    pub exists: bool,
    pub device_type: V5DeviceType,
    pub timestamp: u32,

    pub device_specific_data: DeviceData,
}

pub type V5DeviceHandle = *mut V5Device;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum V5DeviceType {
    NoSensor = 0,
    MotorSensor = 2,
    LedSensor = 3,
    AbsEncSensor = 4,
    CrMotorSensor = 5,
    ImuSensor = 6,
    DistanceSensor = 7,
    RadioSensor = 8,
    TetherSensor = 9,
    BrainSensor = 10,
    VisionSensor = 11,
    AdiSensor = 12,
    Res1Sensor = 13,
    Res2Sensor = 14,
    Res3Sensor = 15,
    OpticalSensor = 16,
    MagnetSensor = 17,
    GpsSensor = 20,
    BumperSensor = 0x40,
    GyroSensor = 0x46,
    SonarSensor = 0x47,
    GenericSensor = 128,
    GenericSerial = 129,
    UndefinedSensor = 255,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerID {
    Primary = 0,
    Partner = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ControllerChannel {
    AnaLeftX = 0,
    AnaLeftY,
    AnaRightX,
    AnaRightY,
    AnaSpare1,
    AnaSpare2,

    ButtonL1,
    ButtonL2,
    ButtonR1,
    ButtonR2,
    ButtonUp,
    ButtonDown,
    ButtonLeft,
    ButtonRight,
    ButtonX,
    ButtonB,
    ButtonY,
    ButtonA,

    ButtonSEL,

    BatteryLevel,

    ButtonAll,
    Flags,
    BatteryCapacity,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum AdiPortConfiguration {
    AnalogIn = 0,
    AnalogOut,
    DigitalIn,
    DigitalOut,

    SmartButton,
    SmartPot,

    LegacyButton,
    LegacyPotentiometer,
    LegacyLineSensor,
    LegacyLightSensor,
    LegacyGyro,
    LegacyAccelerometer,

    LegacyServo,
    LegacyPwm,

    QuadEncoder,
    Sonar,

    LegacyPwmSlew,

    Undefined = 255,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionMode {
    Normal = 0,
    Mixed = 1,
    LineDetect = 2,
    Test = 3,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionBlockType {
    Normal = 0,
    ColorCode = 1,
    LineDetect = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionObject {
    /// block signature
    pub signature: u16,
    /// block type
    pub block_type: VisionBlockType,
    /// left side of block
    pub xoffset: u16,
    /// top of block
    pub yoffset: u16,
    /// width of block
    pub width: u16,
    /// height of block
    pub height: u16,
    /// angle of CC block in 0.1 deg units
    pub angle: u16,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionSignature {
    pub id: u8,
    pub flags: u8,
    pub pad: [u8; 2],
    pub range: f32,
    pub u_min: i32,
    pub u_max: i32,
    pub u_mean: i32,
    pub v_min: i32,
    pub v_max: i32,
    pub v_mean: i32,
    pub m_rgb: u32,
    pub m_type: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionWhiteBalanceMode {
    Normal = 0,
    Start = 1,
    Manual = 2,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct VisionRgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub brightness: u8,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionLedMode {
    Auto = 0,
    Manual = 1,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum VisionWifiMode {
    Off = 0,
    On = 1,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuQuaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct ImuAttitude {
    pub pitch: f64,
    pub roll: f64,
    pub yaw: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalRaw {
    pub clear: u16,
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalRgb {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub brightness: f64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpticalGesture {
    pub udata: u8,
    pub ddata: u8,
    pub ldata: u8,
    pub rdata: u8,
    pub gesture_type: u8,
    pub padding: u8,
    pub count: u16,
    pub time: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum MagnetDuration {
    Short,
    Medium,
    Long,
    ExtraLong,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct GpsRaw {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct GpsAttitude {
    pitch: f64, // x
    roll: f64,  // y
    yaw: f64,   // z

    // spacial position on the field
    position_x: f64,
    position_y: f64,
    position_z: f64,

    // alternative roll, pitch and yaw
    az: f64,
    el: f64,
    rot: f64,
}

#[repr(C)]
#[repr(packed)]
#[derive(Debug, Copy, Clone)]
pub struct GpsQuaternion {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
