#![allow(non_camel_case_types)] // Rust-analyzer gets mad even though this is in mod.rs????

use core::ffi::{c_int, c_void};

pub const XUSB_MAX_ENDPOINTS: usize = 8;

pub type XUsb_EpHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u8, u32)>;
pub type XUsb_IntrHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u32)>;

#[repr(C)]
pub struct XUsb_Config {
    /**< Unique ID of device. */
    pub DeviceId: u16,
    /**< Core register base address. */
    pub BaseAddress: *mut core::ffi::c_uint,
    /**< DMA support Enabled */
    pub DmaEnabled: u8,
    /**< DMA Address Width */
    pub AddrWidth: u8,
    /**< Bits[11:0] Interrupt-id Bits[15:12] trigger
    type and level flags */
    pub IntrId: u16,
    /**< Bit[0] Interrupt parent type Bit[64/32:1] */
    pub IntrParent: *mut core::ffi::c_uint,
}

#[repr(C)]
pub struct XUsb_EpConfig {
    /**< The end point direction */
    pub OutIn: c_int,
    /**< Bulk/interrupt/Isochronous */
    pub EpType: c_int,
    /**< Pkt Size for the first ping-pong
    buffer */
    pub Buffer0Count: c_int,
    /**< Status flag for first ping-pong
    buffer */
    pub Buffer0Ready: c_int,
    /**< Pkt Size for the second ping-pong
    buffer */
    pub Buffer1Count: c_int,
    /**< Status flag for second ping-pong
    buffer */
    pub Buffer1Ready: c_int,
    /**< Maximum buffer size for this end
    point */
    pub Size: u32,
    /**< The rambase offset value in the
    end point buffer space */
    pub RamBase: u32,
    /**< The current ping-pong buffer to be
    used */
    pub CurBufNum: c_int,
    /**< Call back function for this end
    point */
    pub HandlerFunc: XUsb_EpHandlerFunc,
    /**< Callback reference */
    pub HandlerRef: *mut c_void,
}

#[repr(C)]
pub struct XUsb_DeviceConfig {
    /**< Number of Endpoints */
    pub NumEndpoints: u8,
    /**< An array of end points */
    pub Ep: [XUsb_EpConfig; XUSB_MAX_ENDPOINTS],
    /**< USB device Status */
    pub Status: u8,
    /**< Current state of enumeration
    enumerated (1)/Not enumerated (0)*/
    pub CurrentConfiguration: u8,
    /**< Current Speed */
    pub CurrentSpeed: u32,
}

#[repr(C)]
pub struct XUsb {
    /**< Configuration structure */
    pub Config: XUsb_Config,
    /**< Device is initialized and ready */
    pub IsReady: u32,
    /**< The USB address of the device. */
    pub USBAddress: u32,
    /**< End point Offsets */
    pub EndPointOffset: [u32; XUSB_MAX_ENDPOINTS],

    /**
     * The following structure holds the configuration for the controller.
     * They are initialized using the XUsb_ConfigureDevice() function
     * call.
     */
    pub DeviceConfig: XUsb_DeviceConfig,
    //
    //  Callbacks and callback references
    //
    pub HandlerFunc: XUsb_IntrHandlerFunc,
    pub HandlerRef: *mut c_void,

    pub ErrHandlerFunc: XUsb_IntrHandlerFunc,
    pub ErrHandlerRef: *mut c_void,

    pub DmaHandlerFunc: XUsb_IntrHandlerFunc,
    pub DmaHandlerRef: *mut c_void,

    pub UlpiHandlerFunc: XUsb_IntrHandlerFunc,
    pub UlpiHandlerRef: *mut c_void,
}

extern "C" {
    pub fn XUsb_CfgInitialize(
        InstancePtr: *mut XUsb,
        ConfigPtr: *mut XUsb_Config,
        EffectiveAddr: u32,
    ) -> c_int;
    pub fn XUsb_ConfigureDevice(InstancePtr: *mut XUsb, CfgPtr: *mut XUsb_DeviceConfig) -> c_int;
    pub fn XUsb_Start(InstancePtr: *mut XUsb);
    pub fn XUsb_Stop(InstancePtr: *mut XUsb);
    pub fn XUsb_GetFrameNum(InstancePtr: *mut XUsb) -> u32;
    pub fn XUsb_SetDeviceAddress(InstancePtr: *mut XUsb, Address: u8) -> c_int;
    pub fn XUsb_SetTestMode(InstancePtr: *mut XUsb, TestMode: u8, BufPtr: *mut u8);
    pub fn XUsb_DmaReset(InstancePtr: *mut XUsb);
    pub fn XUsb_DmaTransfer(InstancePtr: *mut XUsb, SrcAddr: *mut u32, DstAddr: *mut u32, Length: u16);
    pub fn XUsb_ReadErrorCounters(
        InstancePtr: *mut XUsb,
        BitStuffErrors: *mut u8,
        PidErrors: *mut u8,
        CrcErrors: *mut u8,
    );
    pub fn XUsb_UlpiPhyReadRegister(InstancePtr: *mut XUsb, RegAddr: u8) -> u8;
    pub fn XUsb_UlpiPhyWriteRegister(InstancePtr: *mut XUsb, RegAddr: u8, UlpiPhyRegData: u8) -> u8;
    pub fn XUsb_SieReset(InstancePtr: *mut XUsb);
    pub fn XUsb_EpEnable(InstancePtr: *const XUsb, EpNum: u8);
    pub fn XUsb_EpDisable(InstancePtr: *const XUsb, EpNum: u8);
    pub fn XUsb_EpConfigure(InstancePtr: *mut XUsb, EpNum: u8, EpCfgPtr: *mut XUsb_EpConfig);
    pub fn XUsb_EpDataSend(InstancePtr: *mut XUsb, EpNum: u8, BufferPtr: *mut u8, BufferLen: u32) -> c_int;
    pub fn XUsb_EpDataRecv(InstancePtr: *mut XUsb, EpNum: u8, BufferPtr: *mut u8, BufferLen: u32) -> c_int;
    pub fn XUsb_EpStall(InstancePtr: *const XUsb, EpNum: u8);
    pub fn XUsb_EpUnstall(InstancePtr: *const XUsb, EpNum: u8);
    pub fn XUsb_EpIsoTransferConfigure(InstancePtr: *mut XUsb, EpNum: u8, NoOfTransfers: u8);
    pub fn XUsb_IntrEnable(InstancePtr: *mut XUsb, IntrMask: u32);
    pub fn XUsb_IntrDisable(InstancePtr: *mut XUsb, IntrMask: u32);
    pub fn XUsb_IntrHandler(InstancePtr: *mut c_void);
    pub fn XUsb_IntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    pub fn XUsb_EpSetHandler(
        InstancePtr: *mut XUsb,
        EpNum: u8,
        CallBackFunc: *mut XUsb_EpHandlerFunc,
        CallBackRef: *mut c_void,
    );
    pub fn XUsb_ErrIntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    pub fn XUsb_DmaIntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    pub fn XUsb_UlpiIntrSetHandler(
        InstancePtr: *mut XUsb,
        CallBackFunc: *mut c_void,
        CallBackRef: *mut c_void,
    );
    pub fn XUsb_LookupConfig(BaseAddress: u32) -> *mut XUsb_Config;
}
