#include "hal_network.h"

#include <string.h>
#include <stdlib.h>
#include <stdio.h>
#include <assert.h>
#include "dji/dji_logger.h"

char LINUX_NETWORK_DEV[DEV_NAME_STR_SIZE + 1] = {0};

void set_network_dev(const char *name)
{
    assert(strlen(name) <= DEV_NAME_STR_SIZE);
    strcpy(LINUX_NETWORK_DEV, name);
}

uint16_t usb_net_adapter_vid = 0;
uint16_t usb_net_adapter_pid = 0;

void set_usb_net_adapter_id(uint16_t vid, uint16_t pid)
{
    usb_net_adapter_vid = vid;
    usb_net_adapter_pid = pid;
}

T_DjiReturnCode HalNetwork_Init(const char *ipAddr, const char *netMask, T_DjiNetworkHandle */*halObj*/)
{
    int ret;
    char cmdStr[256];

    if (ipAddr == NULL || netMask == NULL) {
        USER_LOG_ERROR("hal network config param error");
        return DJI_ERROR_SYSTEM_MODULE_CODE_INVALID_PARAMETER;
    }

    //Attention: need root permission to config ip addr and netmask.
    memset(cmdStr, 0, sizeof(cmdStr));
    snprintf(cmdStr, sizeof(cmdStr), "sudo ifconfig %s up", LINUX_NETWORK_DEV);
    ret = system(cmdStr);
    if (ret != DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS) {
        USER_LOG_ERROR("Can't open the network."
                       "Probably the program not execute with root permission."
                       "Please use the root permission to execute the program.");
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
    }

    snprintf(cmdStr, sizeof(cmdStr), "sudo ifconfig %s %s netmask %s", LINUX_NETWORK_DEV, ipAddr, netMask);
    ret = system(cmdStr);
    if (ret != DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS) {
        USER_LOG_ERROR("Can't config the ip address of network."
                       "Probably the program not execute with root permission."
                       "Please use the root permission to execute the program.");
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
    }

    USER_LOG_INFO("network %s set to %s %s", LINUX_NETWORK_DEV, ipAddr, netMask);
    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}

T_DjiReturnCode HalNetwork_DeInit(T_DjiNetworkHandle /*halObj*/)
{
    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}

T_DjiReturnCode HalNetwork_GetDeviceInfo(T_DjiHalNetworkDeviceInfo *deviceInfo)
{
    deviceInfo->usbNetAdapter.vid = usb_net_adapter_vid;
    deviceInfo->usbNetAdapter.pid = usb_net_adapter_pid;
    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}

