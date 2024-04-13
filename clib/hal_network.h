#ifndef HAL_NETWORK_H
#define HAL_NETWORK_H

#include "dji/dji_platform.h"

#define DEV_NAME_STR_SIZE 128

extern char LINUX_NETWORK_DEV[DEV_NAME_STR_SIZE + 1]; // ""
void set_network_dev(const char *name);

extern uint16_t usb_net_adapter_vid;
extern uint16_t usb_net_adapter_pid;
void set_usb_net_adapter_id(uint16_t vid, uint16_t pid);

T_DjiReturnCode HalNetwork_Init(const char *ipAddr, const char *netMask, T_DjiNetworkHandle *halObj);
T_DjiReturnCode HalNetwork_DeInit(T_DjiNetworkHandle halObj);
T_DjiReturnCode HalNetwork_GetDeviceInfo(T_DjiHalNetworkDeviceInfo *deviceInfo);

#endif // HAL_NETWORK_H
