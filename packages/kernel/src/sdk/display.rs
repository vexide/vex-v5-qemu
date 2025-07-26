//! Brain Display

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use core::{
    ffi::{c_char, CStr, VaList},
    num::NonZeroU16,
    slice,
};

use vex_sdk::*;
use vex_v5_qemu_protocol::{
    display::{Color, DrawCommand, ScrollLocation, Shape, TextLocation, TextSize},
    geometry::{Point2, Rect},
    DisplayCommand, HostBoundPacket,
};

use crate::{
    protocol::{self, ProtocolError},
    sync::Mutex,
};

const HEADER_HEIGHT: i32 = 32;
const RESOLUTION_X: i32 = 480;
const RESOLUTION_Y: i32 = 272;

pub static DISPLAY: Mutex<Display> = Mutex::new(Display::new(
    Color(0xFFFFFF),
    Color(0x000000),
    Rect {
        top_left: Point2 {
            x: 0,
            y: HEADER_HEIGHT,
        },
        bottom_right: Point2 {
            x: RESOLUTION_X,
            y: RESOLUTION_Y,
        },
    },
));

struct Display {
    foreground: Color,
    background: Color,
    clip_region: Rect,
}

impl Display {
    pub const fn new(foreground: Color, background: Color, clip_region: Rect) -> Self {
        Self {
            foreground,
            background,
            clip_region,
        }
    }

    pub fn foreground(&self) -> Color {
        self.foreground
    }

    pub fn background(&self) -> Color {
        self.background
    }

    #[allow(unused)]
    pub fn clip_region(&self) -> Rect {
        self.clip_region
    }

    pub fn set_foreground(&mut self, color: Color) {
        self.foreground = color;
    }

    pub fn set_background(&mut self, color: Color) {
        self.background = color;
    }

    pub fn set_clip_region(&mut self, rect: Rect) {
        self.clip_region = rect;
    }

    pub fn erase(&mut self) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayCommand {
            command: DisplayCommand::Erase {
                color: self.background,
                clip_region: self.clip_region,
            },
        })
    }

    pub fn scroll(&mut self, location: ScrollLocation, lines: i32) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayCommand {
            command: DisplayCommand::Scroll {
                location,
                lines,
                background: self.background,
                clip_region: self.clip_region,
            },
        })
    }

    #[allow(unused)]
    pub fn fill(&mut self, shape: Shape, color: Color) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayCommand {
            command: DisplayCommand::Draw {
                command: DrawCommand::Fill(shape),
                color,
                clip_region: self.clip_region,
            },
        })
    }

    pub fn fill_with_foreground(&mut self, shape: Shape) -> Result<(), ProtocolError> {
        self.fill(shape, self.foreground)
    }

    pub fn fill_with_background(&mut self, shape: Shape) -> Result<(), ProtocolError> {
        self.fill(shape, self.background)
    }

    #[allow(unused)]
    pub fn stroke(&mut self, shape: Shape, color: Color) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayCommand {
            command: DisplayCommand::Draw {
                command: DrawCommand::Stroke(shape),
                color,
                clip_region: self.clip_region,
            },
        })
    }

    pub fn stroke_with_foreground(&mut self, shape: Shape) -> Result<(), ProtocolError> {
        self.stroke(shape, self.foreground)
    }

    #[allow(unused)]
    pub fn stroke_with_background(&mut self, shape: Shape) -> Result<(), ProtocolError> {
        self.stroke(shape, self.background)
    }

    pub fn copy_buffer(
        &mut self,
        top_left: Point2<i32>,
        bottom_right: Point2<i32>,
        stride: NonZeroU16,
        buffer: Vec<u32>,
    ) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayCommand {
            command: DisplayCommand::Draw {
                command: DrawCommand::CopyBuffer {
                    top_left,
                    bottom_right,
                    stride,
                    buffer,
                },
                color: self.foreground,
                clip_region: self.clip_region,
            },
        })
    }

    #[allow(unused)]
    pub fn draw_text(
        &mut self,
        data: String,
        size: TextSize,
        location: TextLocation,
        opaque: bool,
        background: Color,
    ) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayCommand {
            command: DisplayCommand::Draw {
                command: DrawCommand::Text {
                    data,
                    size,
                    location,
                    opaque,
                    background,
                },
                color: self.foreground,
                clip_region: self.clip_region,
            },
        })
    }
}

pub fn draw_error_box(message: [Option<&str>; 3]) {
    let mut display = DISPLAY.lock();
    display.fill(
        Shape::Rectangle {
            top_left: Point2 { x: 50, y: 50 },
            bottom_right: Point2 { x: 340, y: 140 },
        },
        Color(0x8b0000),
    ).unwrap();

    display.stroke(
        Shape::Rectangle {
            top_left: Point2 { x: 50, y: 50 },
            bottom_right: Point2 { x: 340, y: 140 },
        },
        Color(0xffffff),
    ).unwrap();

    for (n, line) in message.iter().enumerate() {
        if let Some(text) = line {
            log::error!("{text}");

            display
                .draw_text(
                    text.to_string(),
                    TextSize::Small,
                    TextLocation::Coordinates(Point2 {
                        x: 80,
                        y: 70 + 20 * (n as i32),
                    }),
                    false,
                    Color(0x8b0000),
                )
                .unwrap()
        }
    }
}

pub extern "C" fn vexDisplayForegroundColor(col: u32) {
    DISPLAY.lock().set_foreground(Color(col));
}
pub extern "C" fn vexDisplayBackgroundColor(col: u32) {
    DISPLAY.lock().set_background(Color(col));
}
pub extern "C" fn vexDisplayErase() {
    DISPLAY.lock().erase().unwrap();
}
pub extern "C" fn vexDisplayScroll(nStartLine: i32, nLines: i32) {
    DISPLAY
        .lock()
        .scroll(ScrollLocation::Line(nStartLine), nLines)
        .unwrap();
}
pub extern "C" fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32) {
    DISPLAY
        .lock()
        .scroll(
            ScrollLocation::Rect(Rect {
                top_left: Point2 { x: x1, y: y1 },
                bottom_right: Point2 { x: x2, y: y2 },
            }),
            nLines,
        )
        .unwrap();
}
/// # Safety
///
/// pSrc must satisfy the safety requirements outlined by
/// [`slice::from_raw_parts`].
pub unsafe fn vexDisplayCopyRect(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    pSrc: *mut u32,
    srcStride: i32,
) {
    DISPLAY
        .lock()
        .copy_buffer(
            Point2 { x: x1, y: y1 },
            Point2 { x: x2, y: y2 },
            // todo: figure out what VEX does here rather than panicking on invalid stride.
            NonZeroU16::new(srcStride as u16).unwrap(),
            unsafe { slice::from_raw_parts(pSrc, (x2 - x1) as usize * (y2 - y1) as usize) }
                .to_vec(),
        )
        .unwrap();
}
pub extern "C" fn vexDisplayPixelSet(x: u32, y: u32) {
    DISPLAY
        .lock()
        .fill_with_foreground(Shape::Rectangle {
            top_left: Point2 {
                x: x as _,
                y: y as _,
            },
            bottom_right: Point2 {
                x: x as _,
                y: y as _,
            },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayPixelClear(x: u32, y: u32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Rectangle {
            top_left: Point2 {
                x: x as _,
                y: y as _,
            },
            bottom_right: Point2 {
                x: x as _,
                y: y as _,
            },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        // it's a line so we don't stroke
        .fill_with_foreground(Shape::Line {
            start: Point2 { x: x1, y: y1 },
            end: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Line {
            start: Point2 { x: x1, y: y1 },
            end: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .stroke_with_foreground(Shape::Rectangle {
            top_left: Point2 { x: x1, y: y1 },
            bottom_right: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Rectangle {
            top_left: Point2 { x: x1, y: y1 },
            bottom_right: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .fill_with_foreground(Shape::Rectangle {
            top_left: Point2 { x: x1, y: y1 },
            bottom_right: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub extern "C" fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32) {
    DISPLAY
        .lock()
        .stroke_with_foreground(Shape::Circle {
            center: Point2 { x: xc, y: yc },
            radius: radius as _,
        })
        .unwrap()
}
pub extern "C" fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Circle {
            center: Point2 { x: xc, y: yc },
            radius: radius as _,
        })
        .unwrap()
}
pub extern "C" fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32) {
    DISPLAY
        .lock()
        .fill_with_foreground(Shape::Circle {
            center: Point2 { x: xc, y: yc },
            radius: radius as _,
        })
        .unwrap()
}
pub extern "C" fn vexDisplayTextSize(n: u32, d: u32) {}
pub extern "C" fn vexDisplayFontNamedSet(pFontName: *const c_char) {}
pub extern "C" fn vexDisplayForegroundColorGet() -> u32 {
    DISPLAY.lock().foreground().0
}
pub extern "C" fn vexDisplayBackgroundColorGet() -> u32 {
    DISPLAY.lock().background().0
}

/// # Safety
///
/// pString must satisfy the safety requirements outlined by [`CStr::from_ptr`].
pub unsafe extern "C" fn vexDisplayStringWidthGet(pString: *const c_char) -> i32 {
    (unsafe { CStr::from_ptr(pString) }.count_bytes() * 8) as _
}

/// # Safety
///
/// pString must satisfy the safety requirements outlined by [`CStr::from_ptr`].
pub unsafe extern "C" fn vexDisplayStringHeightGet(pString: *const c_char) -> i32 {
    15
}
pub extern "C" fn vexDisplayPenSizeSet(width: u32) {}
pub extern "C" fn vexDisplayPenSizeGet() -> u32 {
    Default::default()
}
pub extern "C" fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY.lock().set_clip_region(Rect {
        top_left: Point2 { x: x1, y: y1 },
        bottom_right: Point2 { x: x2, y: y2 },
    })
}
pub extern "C" fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool) {
    protocol::send_packet(HostBoundPacket::DisplayCommand {
        command: DisplayCommand::Render,
    })
    .unwrap();
}
pub extern "C" fn vexDisplayDoubleBufferDisable() {
    protocol::send_packet(HostBoundPacket::DisplayCommand {
        command: DisplayCommand::DisableDoubleBuffering,
    })
    .unwrap();
}
pub extern "C" fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32) {
}
pub extern "C" fn vexImageBmpRead(
    ibuf: *const u8,
    oBuf: *mut v5_image,
    maxw: u32,
    maxh: u32,
) -> u32 {
    Default::default()
}
pub extern "C" fn vexImagePngRead(
    ibuf: *const u8,
    oBuf: *mut v5_image,
    maxw: u32,
    maxh: u32,
    ibuflen: u32,
) -> u32 {
    Default::default()
}
pub extern "C" fn vexDisplayVPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub extern "C" fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {
}
pub extern "C" fn vexDisplayVStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub extern "C" fn vexDisplayVBigString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub extern "C" fn vexDisplayVBigStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub extern "C" fn vexDisplayVSmallStringAt(
    xpos: i32,
    ypos: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub extern "C" fn vexDisplayVCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub extern "C" fn vexDisplayVBigCenteredString(
    nLineNumber: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
