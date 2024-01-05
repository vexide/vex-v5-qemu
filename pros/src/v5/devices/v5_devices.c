#include "emu_devices.h"
#include "string.h"

struct _V5_Device emu_smart_ports[V5_MAX_DEVICE_PORTS];
emu_adi emu_adi_ports[8];

void installDevice(uint8_t index, V5_DeviceType type) {
    if(index > V5_MAX_DEVICE_PORTS) return;
    memset(&(emu_smart_ports[index].motor), 0, sizeof(emu_smart_ports[index].motor));
    emu_smart_ports[index].type = type;
}
// Generic device
uint32_t vexDevicesGetNumber() {
    uint8_t count = 0;
    for (uint8_t i = 0; i < V5_MAX_DEVICE_PORTS; i++) {
        if (emu_smart_ports[i].exists) count++;
    }
    return count;
}

uint32_t vexDevicesGetNumberByType(V5_DeviceType type) {
    uint8_t count = 0;
    for (uint8_t i = 0; i < V5_MAX_DEVICE_PORTS; i++) {
        if (emu_smart_ports[i].exists && emu_smart_ports[i].type == type) count++;
    }
    return count;
}

V5_DeviceT vexDevicesGet() {
    return emu_smart_ports;
}

V5_DeviceT vexDeviceGetByIndex(uint32_t index) {
    if (index >= (V5_MAX_DEVICE_PORTS)) return 0;
    return &emu_smart_ports[index];
}

int32_t vexDeviceGetStatus(V5_DeviceType *buffer) {
    for(uint8_t i = 0; i < V5_MAX_DEVICE_PORTS; i++) {
        if(!emu_smart_ports[i].exists)
            buffer[i] = kDeviceTypeNoSensor;
        else
            buffer[i] = emu_smart_ports[i].type;
    }
    return 0;
}

int32_t vexDeviceGetTimestamp(V5_DeviceT device) {
    return device->timestamp;
}

int32_t vexDeviceGetTimestampByIndex(int32_t index) {
    if (index >= V5_MAX_DEVICE_PORTS) return -1;
    return emu_smart_ports[index].timestamp;
}

uint32_t vexDeviceGenericRadioReceiveAvail(V5_DeviceT device) {
return 0;
}

uint32_t vexDeviceGenericRadioReceiveFree(V5_DeviceT device) {
    return 0;
}

void vexDeviceGenericRadioConnection(V5_DeviceT device, char* link_id, int type, bool ov) {}

bool vexDeviceGenericRadioLinkStatus(V5_DeviceT device) { return false;}

int32_t vexDeviceGenericRadioWriteFree(V5_DeviceT device) { return 0;}

int32_t vexDeviceGenericRadioTransmit(V5_DeviceT device, uint8_t *data, uint16_t size) { return 0; }

int32_t vexDeviceGenericRadioReceive(V5_DeviceT device, uint8_t *data, uint16_t size) { return 0; }

// This is used by the port index functions to map an index to the device pointer
#define VEX_DEVICE_GET(device, index) V5_DeviceT device = vexDeviceGetByIndex(index)
