#ifndef HAL_UART_H
#define HAL_UART_H

#include "dji/dji_platform.h"

#define DEV_NAME_STR_SIZE 128

extern char LINUX_UART_DEV0[DEV_NAME_STR_SIZE + 1]; // "/dev/ttyUSB0"
extern char LINUX_UART_DEV1[DEV_NAME_STR_SIZE + 1]; // "/dev/ttyACM0"
void set_uart_dev0(const char *name);
void set_uart_dev1(const char *name);

T_DjiReturnCode HalUart_Init(E_DjiHalUartNum uartNum, uint32_t baudRate, T_DjiUartHandle *uartHandle);
T_DjiReturnCode HalUart_DeInit(T_DjiUartHandle uartHandle);
T_DjiReturnCode HalUart_WriteData(T_DjiUartHandle uartHandle, const uint8_t *buf, uint32_t len, uint32_t *realLen);
T_DjiReturnCode HalUart_ReadData(T_DjiUartHandle uartHandle, uint8_t *buf, uint32_t len, uint32_t *realLen);
T_DjiReturnCode HalUart_GetStatus(E_DjiHalUartNum uartNum, T_DjiUartStatus *status);

#endif // HAL_UART_H
