#ifndef OSAL_SOCKET_H
#define OSAL_SOCKET_H

#include "dji/dji_platform.h"

T_DjiReturnCode Osal_Socket(E_DjiSocketMode mode, T_DjiSocketHandle *socketHandle);
T_DjiReturnCode Osal_Close(T_DjiSocketHandle socketHandle);
T_DjiReturnCode Osal_Bind(T_DjiSocketHandle socketHandle, const char *ipAddr, uint32_t port);
T_DjiReturnCode Osal_UdpSendData(T_DjiSocketHandle socketHandle, const char *ipAddr, uint32_t port,
                                 const uint8_t *buf, uint32_t len, uint32_t *realLen);
T_DjiReturnCode Osal_UdpRecvData(T_DjiSocketHandle socketHandle, char *ipAddr, uint32_t *port,
                                 uint8_t *buf, uint32_t len, uint32_t *realLen);
T_DjiReturnCode Osal_TcpListen(T_DjiSocketHandle socketHandle);
T_DjiReturnCode Osal_TcpAccept(T_DjiSocketHandle socketHandle, char *ipAddr, uint32_t *port,
                               T_DjiSocketHandle *outSocketHandle);
T_DjiReturnCode Osal_TcpConnect(T_DjiSocketHandle socketHandle, const char *ipAddr, uint32_t port);
T_DjiReturnCode Osal_TcpSendData(T_DjiSocketHandle socketHandle,
                                 const uint8_t *buf, uint32_t len, uint32_t *realLen);
T_DjiReturnCode Osal_TcpRecvData(T_DjiSocketHandle socketHandle,
                                 uint8_t *buf, uint32_t len, uint32_t *realLen);

#endif // OSAL_SOCKET_H
