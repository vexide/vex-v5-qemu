#![allow(non_camel_case_types)] // Rust-analyzer gets mad even though this is in mod.rs????

use core::ffi::{c_char, c_int, c_void};

pub const XUSBPS_MAX_ENDPOINTS: usize = 12;
pub const XUSBPS_TIMEOUT_COUNTER: usize = 1000000;
pub const XUSBPS_EP_DIRECTION_IN: usize = 0x01;
/**< Endpoint direction IN. */
pub const XUSBPS_EP_DIRECTION_OUT: usize = 0x02;
/**< Endpoint direction OUT. */
pub const XUSBPS_EP_TYPE_NONE: usize = 0;
/**< Endpoint is not used. */
pub const XUSBPS_EP_TYPE_CONTROL: usize = 1;
/**< Endpoint for Control Transfers */
pub const XUSBPS_EP_TYPE_ISOCHRONOUS: usize = 2;
/**< Endpoint for isochronous data */
pub const XUSBPS_EP_TYPE_BULK: usize = 3;
/**< Endpoint for BULK Transfers. */
pub const XUSBPS_EP_TYPE_INTERRUPT: usize = 4;
/**< Endpoint for interrupt Transfers */
pub const ENDPOINT_MAXP_LENGTH: usize = 0x400;
pub const ENDPOINT_MAXP_MULT_MASK: usize = 0xC00;
pub const ENDPOINT_MAXP_MULT_SHIFT: usize = 10;
pub const XUSBPS_EP_STS_ADDRESS: usize = 1;
/**< Address of controller. */
pub const XUSBPS_EP_STS_CONTROLLER_STATE: usize = 2;
/**< Current controller state. */
pub const XUSBPS_SPEED_UNKNOWN: usize = 0;
pub const XUSBPS_SPEED_LOW: usize = 1;
pub const XUSBPS_SPEED_FULL: usize = 2;
pub const XUSBPS_SPEED_HIGH: usize = 3;
pub const XUSBPS_DEFAULT_ALT_SETTING: usize = 0;
/**< The default alternate setting is 0 */
pub const XUSBPS_STATE_ATTACHED: usize = 0;
pub const XUSBPS_STATE_POWERED: usize = 1;
pub const XUSBPS_STATE_DEFAULT: usize = 2;
pub const XUSBPS_STATE_ADDRESS: usize = 3;
pub const XUSBPS_STATE_CONFIGURED: usize = 4;
pub const XUSBPS_STATE_SUSPENDED: usize = 5;
/**
 * @name Endpoint event types
 * Definitions that are used to identify events that occur on endpoints. Passed
 * to the endpoint event handler functions registered with
 * XUsbPs_EpSetHandler().
 * @{
 */
pub const XUSBPS_EP_EVENT_SETUP_DATA_RECEIVED: usize = 0x01;
/**< Setup data has been received on the endpoint. */
pub const XUSBPS_EP_EVENT_DATA_RX: usize = 0x02;
/**< Data frame has been received on the endpoint. */
pub const XUSBPS_EP_EVENT_DATA_TX: usize = 0x03;
/**< Data frame has been sent on the endpoint. */
/* @} */
/*
 * Maximum packet size for endpoint, 1024
 * @{
 */
pub const XUSBPS_MAX_PACKET_SIZE: usize = 1024;

/**< Alignment of the Device Queue Head List BASE. */
pub const XUSBPS_dQH_BASE_ALIGN: usize = 2048;

/**< Alignment of a Device Queue Head structure. */
pub const XUSBPS_dQH_ALIGN: usize = 64;

/**< Alignment of a Device Transfer Descriptor structure. */
pub const XUSBPS_dTD_ALIGN: usize = 32;

/**< Size of one RX buffer for a OUT Transfer Descriptor. */
pub const XUSBPS_dTD_BUF_SIZE: usize = 4096;

/**< Maximum size of one RX/TX buffer. */
pub const XUSBPS_dTD_BUF_MAX_SIZE: usize = 16 * 1024;

/**< Alignment requirement for Transfer Descriptor buffers. */
pub const XUSBPS_dTD_BUF_ALIGN: usize = 4096;

pub type XUsbPs_EpHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u8, u8, *mut c_void)>;
pub type XUsbPs_EpIsoHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u32, u32)>;
pub type XUsbPs_IntrHandlerFunc = Option<unsafe extern "C" fn(*mut c_void, u32)>;

pub type XUsbPs_dQH = [u8; XUSBPS_dQH_ALIGN];
pub type XUsbPs_dTD = [u8; XUSBPS_dTD_ALIGN];

/**
 * The following data structure represents OUT endpoint.
 */
#[repr(C)]
pub struct XUsbPs_EpOut {
    /**< Pointer to the Queue Head structure of the endpoint. */
    pub dQH: *mut XUsbPs_dQH,
    /**< Pointer to the first dTD of the dTD list for this
     * endpoint. */
    pub dTDs: *mut XUsbPs_dTD,
    /**< Buffer to the currently processed descriptor. */
    pub dTDCurr: *mut XUsbPs_dTD,
    /**< Pointer to the first buffer of the buffer list for this
     * endpoint. */
    pub dTDBufs: *mut u8,
    pub HandlerFunc: XUsbPs_EpHandlerFunc,
    /**< Handler function for this endpoint. */
    pub HandlerIsoFunc: XUsbPs_EpIsoHandlerFunc,
    /**< User data reference for the handler. */
    pub HandlerRef: *mut c_void,
    /**< RequestedBytes for transfer */
    pub RequestedBytes: u32,
    /**< Actual Bytes transferred */
    pub BytesTxed: u32,
    /**< Buffer location */
    pub BufferPtr: *mut u8,
    /**< Mem alloted and data is not received */
    pub MemAlloted: u8,
    /**< Data transfer service interval */
    pub Interval: u32,
}

/**
 * The following data structure represents IN endpoint.
 */
#[repr(C)]
pub struct XUsbPs_EpIn {
    /**< Pointer to the Queue Head structure of the endpoint. */
    pub dQH: *mut XUsbPs_dQH,
    /**< List of pointers to the Transfer Descriptors of the
     * endpoint. */
    pub dTDs: *mut XUsbPs_dTD,
    /**< Buffer to the next available descriptor in the list. */
    pub dTDHead: *mut XUsbPs_dTD,
    /**< Buffer to the last unsent descriptor in the list*/
    pub dTDTail: *mut XUsbPs_dTD,
    pub HandlerFunc: XUsbPs_EpHandlerFunc,
    /**< Handler function for this endpoint. */
    pub HandlerIsoFunc: XUsbPs_EpIsoHandlerFunc,
    /**< User data reference for the handler. */
    pub HandlerRef: *mut c_void,
    /**< RequestedBytes for transfer */
    pub RequestedBytes: u32,
    /**< Actual Bytes transferred */
    pub BytesTxed: u32,
    /**< Buffer location */
    pub BufferPtr: *mut u8,
    /**< Data transfer service interval */
    pub Interval: u32,
}

/**
 * The following data structure represents an endpoint used internally
 * by the L0/L1 driver.
 */
#[repr(C)]
pub struct XUsbPs_Endpoint {
    /**< OUT endpoint structure */
    pub Out: XUsbPs_EpOut,
    /**< IN endpoint structure */
    pub In: XUsbPs_EpIn,
}

/**
 * The following structure is used by the user to receive Setup Data from an
 * endpoint. Using this structure simplifies the process of interpreting the
 * setup data in the core's data fields.
 *
 * The naming scheme for the members of this structure is different from the
 * naming scheme found elsewhere in the code. The members of this structure are
 * defined in the Chapter 9 USB reference guide. Using this naming scheme makes
 * it easier for people familiar with the standard to read the code.
 */
#[repr(C)]
pub struct XUsbPs_SetupData {
    /**< bmRequestType in setup data */
    pub bmRequestType: u8,
    /**< bRequest in setup data */
    pub bRequest: u8,
    /**< wValue in setup data */
    pub wValue: u16,
    /**< wIndex in setup data */
    pub wIndex: u16,
    /**< wLength in setup data */
    pub wLength: u16,
}

/**
 * Data structures used to configure endpoints.
 */
#[repr(C)]
pub struct XUsbPs_EpSetup {
    /**< Endpoint type:
    - XUSBPS_EP_TYPE_CONTROL
    - XUSBPS_EP_TYPE_ISOCHRONOUS
    - XUSBPS_EP_TYPE_BULK
    - XUSBPS_EP_TYPE_INTERRUPT */
    pub Type: u32,

    /**< Number of buffers to be handled by this endpoint. */
    pub NumBufs: u32,
    /**< Buffer size. Only relevant for OUT (receive) Endpoints. */
    pub BufSize: u32,

    /**< Maximum packet size for this endpoint. This number will
     * define the maximum number of bytes sent on the wire per
     * transaction. Range: 0..1024 */
    pub MaxPacketSize: u16,
}

/**
 * The XUsbPs_Config structure contains configuration information for the USB
 * controller.
 *
 * This structure only contains the basic configuration for the controller. The
 * caller also needs to initialize the controller for the DEVICE mode
 * using the XUsbPs_DeviceConfig data structures with the
 * XUsbPs_ConfigureDevice() function call
 */
#[repr(C)]
pub struct XUsbPs_Config {
    /**< Unique Name of controller */
    pub Name: *mut c_char,
    /**< Core register base address. */
    pub BaseAddress: u32,
    /** Bits[11:0] Interrupt-id Bits[15:12] trigger type */
    pub IntrId: u16,
    /** level flags */
    pub IntrParent: u32,
}

/**
 * The XUsbPs_DeviceConfig structure contains the configuration information to
 * configure the USB controller for DEVICE mode. This data structure is used
 * with the XUsbPs_ConfigureDevice() function call.
 */
#[repr(C)]
pub struct XUsbPs_DeviceConfig {
    pub NumEndpoints: u8,
    /**< Number of Endpoints for the controller.
    This number depends on the runtime
    configuration of driver. The driver may
    configure fewer endpoints than are available
    in the core. */
    pub EpCfg: [XUsbPs_EpConfig; XUSBPS_MAX_ENDPOINTS],
    /**< List of endpoint configurations. */
    pub DMAMemPhys: u32,
    /**< Physical base address of DMAable memory
    allocated for the driver. */

    /* The following members are used internally by the L0/L1 driver.  They
     * MUST NOT be accesses and/or modified in any way by the upper layers.
     *
     * The reason for having these members is that we generally try to
     * avoid allocating memory in the L0/L1 driver as we want to be OS
     * independent. In order to avoid allocating memory for this data
     * structure wihin L0/L1 we put it into the XUsbPs_DeviceConfig
     * structure which is allocated by the caller.
     */
    /**< List of endpoint metadata structures. */
    pub Ep: [XUsbPs_Endpoint; XUSBPS_MAX_ENDPOINTS],

    /**< 64 byte aligned base address of the DMA
    memory block. Will be computed and set by
    the L0/L1 driver. */
    pub PhysAligned: u32,
}

#[repr(C)]
pub struct Usb_DevData {
    pub Speed: u8,
    pub State: u8,

    pub PrivateData: *mut c_void,
}

/**
 * The XUsbPs driver instance data. The user is required to allocate a
 * variable of this type for every USB controller in the system. A pointer to a
 * variable of this type is then passed to the driver API functions.
 */
#[repr(C)]
pub struct XUsbPs {
    pub SetupData: XUsbPs_SetupData,
    /**< Setup Packet buffer */
    pub Config: XUsbPs_Config,
    /**< Configuration structure */
    pub CurrentAltSetting: c_int,
    /**< Current alternative setting of interface */
    pub UserDataPtr: *mut c_void,
    /**< Data pointer to be used by upper layers to
    store application dependent data structures.
    The upper layers are responsible to allocated
    and free the memory. The driver will not
    mofidy this data pointer. */

    /**
     * The following structures hold the configuration for DEVICE mode
     * of the controller. They are initialized using the
     * XUsbPs_ConfigureDevice() function call.
     */
    pub DeviceConfig: XUsbPs_DeviceConfig,
    /**< Configuration for the DEVICE mode. */
    pub HandlerFunc: XUsbPs_IntrHandlerFunc,
    /**< Handler function for the controller. */
    pub HandlerRef: *mut c_void,
    /**< User data reference for the handler. */
    pub HandlerMask: u32,
    /**< User interrupt mask. Defines which interrupts will cause
     * the callback to be called. */
    pub AppData: *mut Usb_DevData,
    pub IsConfigDone: u8,
    pub data_ptr: *mut c_void, /* pointer for storing applications data */
}

#[repr(C)]
pub struct XUsbPs_EpConfig {
    /**< OUT component of endpoint. */
    pub Out: XUsbPs_EpSetup,
    /**< IN component of endpoint. */
    pub In: XUsbPs_EpSetup,
}

extern "C" {

    /**
     * Setup / Initialize functions.
     *
     * Implemented in file xusbps.c
     */
    pub fn XUsbPs_CfgInitialize(
        InstancePtr: *mut XUsbPs,
        ConfigPtr: *const XUsbPs_Config,
        BaseAddress: u32,
    ) -> c_int;

    pub fn XUsbPs_ConfigureDevice(
        InstancePtr: *mut XUsbPs,
        CfgPtr: *const XUsbPs_DeviceConfig,
    ) -> c_int;

    /**
     * Common functions used for DEVICE/HOST mode.
     */
    pub fn XUsbPs_Reset(InstancePtr: *mut XUsbPs) -> c_int;

    pub fn XUsbPs_DeviceReset(InstancePtr: *mut XUsbPs);

    /**
     * DEVICE mode specific functions.
     */
    pub fn XUsbPs_BusReset(InstancePtr: *mut XUsbPs) -> c_int;
    pub fn XUsbPs_SetDeviceAddress(InstancePtr: *mut XUsbPs, Address: u8) -> c_int;

    /**
     * Handling Suspend and Resume.
     *
     * Implemented in xusbps.c
     */
    pub fn XUsbPs_Suspend(InstancePtr: *const XUsbPs) -> c_int;
    pub fn XUsbPs_Resume(InstancePtr: *const XUsbPs) -> c_int;
    pub fn XUsbPs_RequestHostResume(InstancePtr: *const XUsbPs) -> c_int;

    /*
     * Functions for managing Endpoints / Transfers
     *
     * Implemented in file xusbps_endpoint.c
     */
    pub fn XUsbPs_EpBufferSend(
        InstancePtr: *mut XUsbPs,
        EpNum: u8,
        BufferPtr: *const u8,
        BufferLen: u32,
    ) -> c_int;
    pub fn XUsbPs_EpBufferSendWithZLT(
        InstancePtr: *mut XUsbPs,
        EpNum: u8,
        BufferPtr: *const u8,
        BufferLen: u32,
    ) -> c_int;
    pub fn XUsbPs_EpBufferReceive(
        InstancePtr: *mut XUsbPs,
        EpNum: u8,
        BufferPtr: *mut *mut u8,
        BufferLenPtr: *mut u32,
        Handle: *mut u32,
    ) -> c_int;
    pub fn XUsbPs_EpBufferRelease(Handle: u32);

    pub fn XUsbPs_EpSetHandler(
        InstancePtr: *mut XUsbPs,
        EpNum: u8,
        Direction: u8,
        CallBackFunc: XUsbPs_EpHandlerFunc,
        CallBackRef: *mut c_void,
    ) -> c_int;
    pub fn XUsbPs_EpSetIsoHandler(
        InstancePtr: *mut XUsbPs,
        EpNum: u8,
        Direction: u8,
        CallBackFunc: XUsbPs_EpIsoHandlerFunc,
    ) -> c_int;
    pub fn XUsbPs_EpGetSetupData(
        InstancePtr: *mut XUsbPs,
        EpNum: c_int,
        SetupDataPtr: *mut XUsbPs_SetupData,
    ) -> c_int;

    pub fn XUsbPs_EpPrime(InstancePtr: *mut XUsbPs, EpNum: u8, Direction: u8) -> c_int;

    pub fn XUsbPs_ReconfigureEp(
        InstancePtr: *mut XUsbPs,
        CfgPtr: *mut XUsbPs_DeviceConfig,
        EpNum: c_int,
        NewDirection: u16,
        DirectionChanged: c_int,
    ) -> c_int;

    /*
     * Interrupt handling functions
     *
     * Implemented in file xusbps_intr.c
     */
    pub fn XUsbPs_IntrHandler(InstancePtr: *mut XUsbPs);

    pub fn XUsbPs_IntrSetHandler(
        InstancePtr: *mut XUsbPs,
        CallBackFunc: XUsbPs_IntrHandlerFunc,
        CallBackRef: *mut c_void,
        Mask: u32,
    ) -> c_int;
    pub fn XUsbPs_EpGetData(InstancePtr: *mut XUsbPs, EpNum: u8, BufferLen: u32);

    pub fn XUsbPs_EpDataBufferReceive(
        InstancePtr: *mut XUsbPs,
        EpNum: u8,
        BufferPtr: *mut u8,
        BufferLen: u32,
    ) -> c_int;
    /*
     * Helper functions for static configuration.
     * Implemented in xusbps_sinit.c
     */
    pub fn XUsbPs_LookupConfig(BaseAddress: u32) -> *mut XUsbPs_Config;
}
