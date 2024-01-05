#include "v5_api.h"

struct {
    bool a: 1;
    bool b: 1;
    bool x: 1;
    bool y: 1;
    bool l: 1;
    bool r: 1;
    bool u: 1;
    bool d: 1;
    bool l1: 1;
    bool l2: 1;
    bool r1: 1;
    bool r2: 1;
    int8_t lx;
    int8_t rx;
    int8_t ly;
    int8_t ry;
} _controller_data;

// Controller
int32_t vexControllerGet(V5_ControllerId id, V5_ControllerIndex index) {
    switch (index) {
        case AnaLeftY:
            return _controller_data.ly;
        case AnaRightY:
            return _controller_data.ry;
        case AnaLeftX:
            return _controller_data.lx;
        case AnaRightX:
            return _controller_data.rx;
        case AnaSpare1:
            break;
        case AnaSpare2:
            break;
        case Button5U:
            break;
        case Button5D:
            break;
        case Button6U:
            break;
        case Button6D:
            break;
        case Button7U:
            break;
        case Button7D:
            break;
        case Button7L:
            break;
        case Button7R:
            break;
        case Button8U:
            break;
        case Button8D:
            break;
        case Button8L:
            break;
        case Button8R:
            break;
        case ButtonSEL:
            break;
        case BatteryLevel:
            break;
        case ButtonAll:
            break;
        case Flags:
            break;
        case BatteryCapacity:
            break;
    }
    return 0;
}

V5_ControllerStatus vexControllerConnectionStatusGet(V5_ControllerId id) {
}

bool vexControllerTextSet(V5_ControllerId id, uint32_t line, uint32_t col, const char* str) {
}
