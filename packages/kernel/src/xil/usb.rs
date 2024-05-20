#![allow(non_camel_case_types)] // Rust-analyzer gets mad even though this is in mod.rs????

use core::ffi::{c_int, c_void};

pub const XUSB_MAX_ENDPOINTS: usize = 8;

pub type XUsb_EpHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u8, u32)>;
pub type XUsb_IntrHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u32)>;

#[repr(C)]
struct XUsb_Config {
    /**< Unique ID of device. */
    DeviceId: u16,
    /**< Core register base address. */
    BaseAddress: *mut core::ffi::c_uint,
    /**< DMA support Enabled */
    DmaEnabled: u8,
    /**< DMA Address Width */
    AddrWidth: u8,
    /**< Bits[11:0] Interrupt-id Bits[15:12] trigger
    type and level flags */
    IntrId: u16,
    /**< Bit[0] Interrupt parent type Bit[64/32:1] */
    IntrParent: *mut core::ffi::c_uint,
}

#[repr(C)]
struct XUsb_EpConfig {
    /**< The end point direction */
    OutIn: c_int,
    /**< Bulk/interrupt/Isochronous */
    EpType: c_int,
    /**< Pkt Size for the first ping-pong
    buffer */
    Buffer0Count: c_int,
    /**< Status flag for first ping-pong
    buffer */
    Buffer0Ready: c_int,
    /**< Pkt Size for the second ping-pong
    buffer */
    Buffer1Count: c_int,
    /**< Status flag for second ping-pong
    buffer */
    Buffer1Ready: c_int,
    /**< Maximum buffer size for this end
    point */
    Size: u32,
    /**< The rambase offset value in the
    end point buffer space */
    RamBase: u32,
    /**< The current ping-pong buffer to be
    used */
    CurBufNum: c_int,
    /**< Call back function for this end
    point */
    HandlerFunc: XUsb_EpHandlerFunc,
    /**< Callback reference */
    HandlerRef: *mut c_void,
}

#[repr(C)]
struct XUsb_DeviceConfig {
    /**< Number of Endpoints */
    NumEndpoints: u8,
    /**< An array of end points */
    Ep: [XUsb_EpConfig; XUSB_MAX_ENDPOINTS],
    /**< USB device Status */
    Status: u8,
    /**< Current state of enumeration
    enumerated (1)/Not enumerated (0)*/
    CurrentConfiguration: u8,
    /**< Current Speed */
    CurrentSpeed: u32,
}

#[repr(C)]
struct XUsb {
    /**< Configuration structure */
    Config: XUsb_Config,
    /**< Device is initialized and ready */
    IsReady: u32,
    /**< The USB address of the device. */
    USBAddress: u32,
    /**< End point Offsets */
    EndPointOffset: [u32; XUSB_MAX_ENDPOINTS],

    /**
     * The following structure holds the configuration for the controller.
     * They are initialized using the XUsb_ConfigureDevice() function
     * call.
     */
    DeviceConfig: XUsb_DeviceConfig,
    //
    //  Callbacks and callback references
    //
    HandlerFunc: XUsb_IntrHandlerFunc,
    HandlerRef: *mut c_void,

    ErrHandlerFunc: XUsb_IntrHandlerFunc,
    ErrHandlerRef: *mut c_void,

    DmaHandlerFunc: XUsb_IntrHandlerFunc,
    DmaHandlerRef: *mut c_void,

    UlpiHandlerFunc: XUsb_IntrHandlerFunc,
    UlpiHandlerRef: *mut c_void,
}

extern "C" {
    fn XUsb_CfgInitialize(
        InstancePtr: *mut XUsb,
        ConfigPtr: *mut XUsb_Config,
        EffectiveAddr: u32,
    ) -> c_int;
    fn XUsb_ConfigureDevice(InstancePtr: *mut XUsb, CfgPtr: *mut XUsb_DeviceConfig) -> c_int;
    fn XUsb_Start(InstancePtr: *mut XUsb);
    fn XUsb_Stop(InstancePtr: *mut XUsb);
    fn XUsb_GetFrameNum(InstancePtr: *mut XUsb) -> u32;
    fn XUsb_SetDeviceAddress(InstancePtr: *mut XUsb, Address: u8) -> c_int;
    fn XUsb_SetTestMode(InstancePtr: *mut XUsb, TestMode: u8, BufPtr: *mut u8);
    fn XUsb_DmaReset(InstancePtr: *mut XUsb);
    fn XUsb_DmaTransfer(InstancePtr: *mut XUsb, SrcAddr: *mut u32, DstAddr: *mut u32, Length: u16);
    fn XUsb_ReadErrorCounters(
        InstancePtr: *mut XUsb,
        BitStuffErrors: *mut u8,
        PidErrors: *mut u8,
        CrcErrors: *mut u8,
    );
    fn XUsb_UlpiPhyReadRegister(InstancePtr: *mut XUsb, RegAddr: u8) -> u8;
    fn XUsb_UlpiPhyWriteRegister(InstancePtr: *mut XUsb, RegAddr: u8, UlpiPhyRegData: u8) -> u8;
    fn XUsb_SieReset(InstancePtr: *mut XUsb);
    fn XUsb_EpEnable(InstancePtr: *const XUsb, EpNum: u8);
    fn XUsb_EpDisable(InstancePtr: *const XUsb, EpNum: u8);
    fn XUsb_EpConfigure(InstancePtr: *mut XUsb, EpNum: u8, EpCfgPtr: *mut XUsb_EpConfig);
    fn XUsb_EpDataSend(InstancePtr: *mut XUsb, EpNum: u8, BufferPtr: *mut u8, BufferLen: u32) -> c_int;
    fn XUsb_EpDataRecv(InstancePtr: *mut XUsb, EpNum: u8, BufferPtr: *mut u8, BufferLen: u32) -> c_int;
    fn XUsb_EpStall(InstancePtr: *const XUsb, EpNum: u8);
    fn XUsb_EpUnstall(InstancePtr: *const XUsb, EpNum: u8);
    fn XUsb_EpIsoTransferConfigure(InstancePtr: *mut XUsb, EpNum: u8, NoOfTransfers: u8);
    fn XUsb_IntrEnable(InstancePtr: *mut XUsb, IntrMask: u32);
    fn XUsb_IntrDisable(InstancePtr: *mut XUsb, IntrMask: u32);
    fn XUsb_IntrHandler(InstancePtr: *mut c_void);
    fn XUsb_IntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    fn XUsb_EpSetHandler(
        InstancePtr: *mut XUsb,
        EpNum: u8,
        CallBackFunc: *mut XUsb_EpHandlerFunc,
        CallBackRef: *mut c_void,
    );
    fn XUsb_ErrIntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    fn XUsb_DmaIntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    fn XUsb_UlpiIntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    fn XUsb_LookupConfig(BaseAddress: u32) -> *mut Config;
}
