//! Brain Display

use core::ffi::{c_char, VaList};
use vex_sdk::*;

pub extern "C" fn vexDisplayForegroundColor(col: u32) {}
pub extern "C" fn vexDisplayBackgroundColor(col: u32) {}
pub extern "C" fn vexDisplayErase() {}
pub extern "C" fn vexDisplayScroll(nStartLine: i32, nLines: i32) {}
pub extern "C" fn vexDisplayScrollRect(x1: i32, y1: i32, x2: i32, y2: i32, nLines: i32) {}
pub extern "C" fn vexDisplayCopyRect(x1: i32, y1: i32, x2: i32, y2: i32, pSrc: *mut u32, srcStride: i32) {}
pub extern "C" fn vexDisplayPixelSet(x: u32, y: u32) {}
pub extern "C" fn vexDisplayPixelClear(x: u32, y: u32) {}
pub extern "C" fn vexDisplayLineDraw(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexDisplayLineClear(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexDisplayRectDraw(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexDisplayRectClear(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexDisplayRectFill(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexDisplayCircleDraw(xc: i32, yc: i32, radius: i32) {}
pub extern "C" fn vexDisplayCircleClear(xc: i32, yc: i32, radius: i32) {}
pub extern "C" fn vexDisplayCircleFill(xc: i32, yc: i32, radius: i32) {}
pub extern "C" fn vexDisplayTextSize(n: u32, d: u32) {}
pub extern "C" fn vexDisplayFontNamedSet(pFontName: *const c_char) {}
pub extern "C" fn vexDisplayForegroundColorGet() -> u32 {
    Default::default()
}
pub extern "C" fn vexDisplayBackgroundColorGet() -> u32 {
    Default::default()
}
pub extern "C" fn vexDisplayStringWidthGet(pString: *const c_char) -> i32 {
    Default::default()
}
pub extern "C" fn vexDisplayStringHeightGet(pString: *const c_char) -> i32 {
    Default::default()
}
pub extern "C" fn vexDisplayPenSizeSet(width: u32) {}
pub extern "C" fn vexDisplayPenSizeGet() -> u32 {
    Default::default()
}
pub extern "C" fn vexDisplayClipRegionSet(x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexDisplayRender(bVsyncWait: bool, bRunScheduler: bool) {}
pub extern "C" fn vexDisplayDoubleBufferDisable() {}
pub extern "C" fn vexDisplayClipRegionSetWithIndex(index: i32, x1: i32, y1: i32, x2: i32, y2: i32) {}
pub extern "C" fn vexImageBmpRead(ibuf: *const u8, oBuf: *mut v5_image, maxw: u32, maxh: u32) -> u32 {
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
pub extern "C" fn vexDisplayVString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub extern "C" fn vexDisplayVStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub extern "C" fn vexDisplayVBigString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub extern "C" fn vexDisplayVBigStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub extern "C" fn vexDisplayVSmallStringAt(xpos: i32, ypos: i32, format: *const c_char, args: VaList<'_, '_>) {
}
pub extern "C" fn vexDisplayVCenteredString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {}
pub extern "C" fn vexDisplayVBigCenteredString(nLineNumber: i32, format: *const c_char, args: VaList<'_, '_>) {
}
