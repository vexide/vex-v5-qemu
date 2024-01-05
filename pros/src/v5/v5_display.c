#include "v5_api.h"

void vexDisplayForegroundColor(uint32_t col) {}
void vexDisplayBackgroundColor(uint32_t col) {}
uint32_t vexDisplayForegroundColorGet(void) {}
uint32_t vexDisplayBackgroundColorGet(void) {}
void vexDisplayErase() {}
void vexDisplayScroll(int32_t nStartLine, int32_t nLines) {}
void vexDisplayScrollRect(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t nLines) {}
void vexDisplayCopyRect(int32_t x1, int32_t y1, int32_t x2, int32_t y2, uint32_t* pSrc, int32_t srcStride) {}
void vexDisplayPixelSet(uint32_t x, uint32_t y) {}
void vexDisplayPixelClear(uint32_t x, uint32_t y) {}
void vexDisplayLineDraw(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {}
void vexDisplayLineClear(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {}
void vexDisplayRectDraw(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {}
void vexDisplayRectClear(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {}
void vexDisplayRectFill(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {}
void vexDisplayCircleDraw(int32_t xc, int32_t yc, int32_t radius) {}
void vexDisplayCircleClear(int32_t xc, int32_t yc, int32_t radius) {}
void vexDisplayCircleFill(int32_t xc, int32_t yc, int32_t radius) {}

void vexDisplayPrintf(int32_t xpos, int32_t ypos, uint32_t bOpaque, const char* format, ...) {}
void vexDisplayString(const int32_t nLineNumber, const char* format, ...) {}
void vexDisplayStringAt(int32_t xpos, int32_t ypos, const char* format, ...) {}
void vexDisplayBigString(const int32_t nLineNumber, const char* format, ...) {}
void vexDisplayBigStringAt(int32_t xpos, int32_t ypos, const char* format, ...) {}
void vexDisplaySmallStringAt(int32_t xpos, int32_t ypos, const char* format, ...) {}
void vexDisplayCenteredString(const int32_t nLineNumber, const char* format, ...) {}
void vexDisplayBigCenteredString(const int32_t nLineNumber, const char* format, ...) {}

// Not really for user code but we need these for some wrapper functions
void vexDisplayVPrintf(int32_t xpos, int32_t ypos, uint32_t bOpaque, const char* format, va_list args) {}
void vexDisplayVString(const int32_t nLineNumber, const char* format, va_list args) {}
void vexDisplayVStringAt(int32_t xpos, int32_t ypos, const char* format, va_list args) {}
void vexDisplayVBigString(const int32_t nLineNumber, const char* format, va_list args) {}
void vexDisplayVBigStringAt(int32_t xpos, int32_t ypos, const char* format, va_list args) {}
void vexDisplayVSmallStringAt(int32_t xpos, int32_t ypos, const char* format, va_list args) {}
void vexDisplayVCenteredString(const int32_t nLineNumber, const char* format, va_list args) {}
void vexDisplayVBigCenteredString(const int32_t nLineNumber, const char* format, va_list args) {}

void vexDisplayTextSize(uint32_t n, uint32_t d) {}
void vexDisplayFontNamedSet(const char* pFontName) {}
int32_t vexDisplayStringWidthGet(const char* pString) {}
int32_t vexDisplayStringHeightGet(const char* pString) {}

bool vexDisplayRender(bool bVsyncWait, bool bRunScheduler) {}
void vexDisplayDoubleBufferDisable() {}

void vexDisplayClipRegionSet(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {}
void vexDisplayClipRegionClear() {}

void vexTouchUserCallbackSet(void (*callback)(V5_TouchEvent, int32_t, int32_t)) {}
bool vexTouchDataGet(V5_TouchStatus *status) {}