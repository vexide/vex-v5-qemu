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
    display::{
        Color, DisplayRenderMode, DrawCommand, ScrollLocation, Shape, TextLocation, TextMeasurement, TextSize
    },
    geometry::{Point2, Rect},
    HostBoundPacket, KernelBoundPacket,
};

use crate::{
    protocol::{self, ProtocolError},
    sync::Mutex,
};

const HEADER_HEIGHT: i32 = 32;
const RESOLUTION_X: i32 = 480;
const RESOLUTION_Y: i32 = 272;

static DISPLAY: Mutex<Display> = Mutex::new(Display::new(
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
        protocol::send_packet(HostBoundPacket::DisplayErase {
            color: self.background,
            clip_region: self.clip_region,
        })
    }

    pub fn scroll(&mut self, location: ScrollLocation, lines: i32) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayScroll {
            location,
            lines,
            background: self.background,
            clip_region: self.clip_region,
        })
    }

    #[allow(unused)]
    pub fn fill(&mut self, shape: Shape, color: Color) -> Result<(), ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayDraw {
            command: DrawCommand::Fill(shape),
            color,
            clip_region: self.clip_region,
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
        protocol::send_packet(HostBoundPacket::DisplayDraw {
            command: DrawCommand::Stroke(shape),
            color,
            clip_region: self.clip_region,
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
        protocol::send_packet(HostBoundPacket::DisplayDraw {
            command: DrawCommand::CopyBuffer {
                top_left,
                bottom_right,
                stride,
                buffer,
            },
            color: self.foreground,
            clip_region: self.clip_region,
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
        protocol::send_packet(HostBoundPacket::DisplayDraw {
            command: DrawCommand::Text {
                data,
                size,
                location,
                opaque,
                background,
            },
            color: self.foreground,
            clip_region: self.clip_region,
        })
    }

    pub fn measure_text(
        &self,
        data: String,
        size: TextSize,
    ) -> Result<TextMeasurement, ProtocolError> {
        protocol::send_packet(HostBoundPacket::DisplayMeasureText { data, size })?;

        loop {
            if let Some(packet) = protocol::recv_packet()? {
                match packet {
                    KernelBoundPacket::DisplayTextMeasurement(measurement) => {
                        return Ok(measurement)
                    }
                    _ => panic!("Unexpected packet reply to HostBoundPacket::DisplayMeasureText!"),
                }
            }
        }
    }
}

pub fn vexDisplayForegroundColor(col: u32) {
    DISPLAY.lock().set_foreground(Color(col));
}
pub fn vexDisplayBackgroundColor(col: u32) {
    DISPLAY.lock().set_background(Color(col));
}
pub fn vexDisplayErase() {
    DISPLAY.lock().erase().unwrap();
}
pub fn vexDisplayScroll(nStartLine: i32, nLines: i32) {
    DISPLAY
        .lock()
        .scroll(ScrollLocation::Line(nStartLine), nLines)
        .unwrap();
}
pub fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32) {
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
pub fn vexDisplayPixelSet(x: u32, y: u32) {
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
pub fn vexDisplayPixelClear(x: u32, y: u32) {
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
pub fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        // it's a line so we don't stroke
        .fill_with_foreground(Shape::Line {
            start: Point2 { x: x1, y: y1 },
            end: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Line {
            start: Point2 { x: x1, y: y1 },
            end: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .stroke_with_foreground(Shape::Rectangle {
            top_left: Point2 { x: x1, y: y1 },
            bottom_right: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Rectangle {
            top_left: Point2 { x: x1, y: y1 },
            bottom_right: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY
        .lock()
        .fill_with_foreground(Shape::Rectangle {
            top_left: Point2 { x: x1, y: y1 },
            bottom_right: Point2 { x: x2, y: y2 },
        })
        .unwrap()
}
pub fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32) {
    DISPLAY
        .lock()
        .stroke_with_foreground(Shape::Circle {
            center: Point2 { x: xc, y: yc },
            radius: radius as _,
        })
        .unwrap()
}
pub fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32) {
    DISPLAY
        .lock()
        .fill_with_background(Shape::Circle {
            center: Point2 { x: xc, y: yc },
            radius: radius as _,
        })
        .unwrap()
}
pub fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32) {
    DISPLAY
        .lock()
        .fill_with_foreground(Shape::Circle {
            center: Point2 { x: xc, y: yc },
            radius: radius as _,
        })
        .unwrap()
}
pub fn vexDisplayTextSize(n: u32, d: u32) {}
pub fn vexDisplayFontNamedSet(pFontName: *const c_char) {}
pub fn vexDisplayForegroundColorGet() -> u32 {
    DISPLAY.lock().foreground().0
}
pub fn vexDisplayBackgroundColorGet() -> u32 {
    DISPLAY.lock().background().0
}

/// # Safety
///
/// pString must satisfy the safety requirements outlined by [`CStr::from_ptr`].
pub unsafe fn vexDisplayStringWidthGet(pString: *const c_char) -> i32 {
    DISPLAY
        .lock()
        .measure_text(
            unsafe { CStr::from_ptr(pString) }
                .to_str()
                .unwrap()
                .to_string(),
            TextSize::Normal,
        )
        .unwrap()
        .width
}

/// # Safety
///
/// pString must satisfy the safety requirements outlined by [`CStr::from_ptr`].
pub unsafe fn vexDisplayStringHeightGet(pString: *const c_char) -> i32 {
    DISPLAY
        .lock()
        .measure_text(
            unsafe { CStr::from_ptr(pString) }
                .to_str()
                .unwrap()
                .to_string(),
            TextSize::Normal,
        )
        .unwrap()
        .height
}
pub fn vexDisplayPenSizeSet(width: u32) {}
pub fn vexDisplayPenSizeGet() -> u32 {
    Default::default()
}
pub fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32) {
    DISPLAY.lock().set_clip_region(Rect {
        top_left: Point2 { x: x1, y: y1 },
        bottom_right: Point2 { x: x2, y: y2 },
    })
}
pub fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool) {
    protocol::send_packet(HostBoundPacket::DisplayRender).unwrap();
}
pub fn vexDisplayDoubleBufferDisable() {
    protocol::send_packet(HostBoundPacket::DisplayRenderMode(
        DisplayRenderMode::Immediate,
    ))
    .unwrap();
}
pub fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32) {}
pub fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32 {
    Default::default()
}
pub fn vexImagePngRead(
    ibuf: *const u8,
    oBuf: *mut v5_image,
    maxw: u32,
    maxh: u32,
    ibuflen: u32,
) -> u32 {
    Default::default()
}
pub fn vexDisplayVPrintf(
    xpos: i32,
    ypos: i32,
    bOpaque: i32,
    format: *const c_char,
    args: VaList<'_, '_>,
) {
}
pub fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVBigStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVSmallStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {
}
pub fn vexDisplayVCenteredString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub fn vexDisplayVBigCenteredString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {
}
