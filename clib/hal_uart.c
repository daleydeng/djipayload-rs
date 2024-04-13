#include "hal_uart.h"

#include <assert.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>
#include <string.h>
#include <stdlib.h>
#include "dji/dji_logger.h"

char LINUX_UART_DEV0[DEV_NAME_STR_SIZE + 1] = "/dev/ttyUSB0"; 
char LINUX_UART_DEV1[DEV_NAME_STR_SIZE + 1] = "/dev/ttyACM0"; 

void set_uart_dev0(const char *name)
{
    assert(strlen(name) <= DEV_NAME_STR_SIZE);
    strcpy(LINUX_UART_DEV0, name);
}

void set_uart_dev1(const char *name)
{
    assert(strlen(name) <= DEV_NAME_STR_SIZE);
    strcpy(LINUX_UART_DEV1, name);
}

struct uart_fd {
    int fd;
};

T_DjiReturnCode HalUart_Init(E_DjiHalUartNum uartNum, uint32_t baudRate, T_DjiUartHandle *uartHandle)
{
    struct uart_fd *uart_fd = NULL;
    struct termios options;
    struct flock lock;
    T_DjiReturnCode err = DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
    char uartName[DEV_NAME_STR_SIZE];
    int fd = -1;
    speed_t baud_speed;

    if (uartNum == DJI_HAL_UART_NUM_0) {
        strcpy(uartName, LINUX_UART_DEV0);
    } else if (uartNum == DJI_HAL_UART_NUM_1) {
        strcpy(uartName, LINUX_UART_DEV1);
    } else {
        fprintf(stderr, "uartNum(%d) invalid(!= {(%d), (%d)})", uartNum, DJI_HAL_UART_NUM_0, DJI_HAL_UART_NUM_1);
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
    }

    fd = open(uartName, (unsigned) O_RDWR | (unsigned) O_NOCTTY | (unsigned) O_NDELAY);
    if (fd == -1) {
        perror("open");
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
    }

    // Forbid multiple psdk programs to access the serial port
    memset(&lock, 0, sizeof(lock));
    lock.l_type = F_WRLCK;
    lock.l_whence = SEEK_SET;
    lock.l_start = 0;
    lock.l_len = 0;

    if (fcntl(fd, F_GETLK, &lock) < 0) {
        perror("fcntl - F_GETLK");
        err = DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
        goto close_fd;
    }
    
    if (lock.l_type != F_UNLCK) {
        fprintf(stderr, "unable to lock file, lock head by process %d\n", lock.l_pid);
        err = DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
        goto close_fd;
    }
    
    lock.l_type = F_WRLCK;
    lock.l_pid = getpid();

    if (fcntl(fd, F_SETLKW, &lock) < 0) {
        perror("fcntl - F_SETLK");
        err = DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
        goto close_fd;
    }

    if (tcgetattr(fd, &options) != 0) {
        perror("tcgetattr");
        err = DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
        goto close_fd;
    }

    switch (baudRate) {
        case 115200:
            baud_speed = B115200;
            break;
        case 230400:
            baud_speed = B230400;
            break;
        case 460800:
            baud_speed = B460800;
            break;
        case 921600:
            baud_speed = B921600;
            break;
        case 1000000:
            baud_speed = B1000000;
            break;
        default:
            err = DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
            goto close_fd;
    }

    cfsetispeed(&options, baud_speed);
    cfsetospeed(&options, baud_speed);

    options.c_cflag |=  (unsigned) CLOCAL;
    options.c_cflag |=  (unsigned) CREAD;
    options.c_cflag &=~ (unsigned) CRTSCTS;
    options.c_cflag &=~ (unsigned) CSIZE;
    options.c_cflag |=  (unsigned) CS8;
    options.c_cflag &=~ (unsigned) PARENB;
    options.c_iflag &=~ (unsigned) INPCK;
    options.c_cflag &=~ (unsigned) CSTOPB;
    options.c_oflag &=~ (unsigned) OPOST;
    options.c_lflag &=~ ((unsigned) ICANON | (unsigned) ECHO | (unsigned) ECHOE | (unsigned) ISIG);
    options.c_iflag &=~ ((unsigned) BRKINT | (unsigned) ICRNL | (unsigned) INPCK | (unsigned) ISTRIP | (unsigned) IXON);
    options.c_cc[VTIME] = 0;
    options.c_cc[VMIN] = 0;

    tcflush(fd, TCIFLUSH);

    if (tcsetattr(fd, TCSANOW, &options) != 0) {
        perror("tcsetattr - TCSANOW");
        err = DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;
        goto close_fd;
    }

    uart_fd = malloc(sizeof(*uart_fd));
    if (uart_fd == NULL) {
        perror("malloc struct uart_fd");
        err = DJI_ERROR_SYSTEM_MODULE_CODE_MEMORY_ALLOC_FAILED;
        goto close_fd;
    }
    uart_fd->fd = fd;
    *uartHandle = uart_fd;

    return err;

close_fd:
    close(fd);
    return err;
}

T_DjiReturnCode HalUart_DeInit(T_DjiUartHandle uartHandle)
{
    int ret;
    struct uart_fd *uart_fd = (struct uart_fd *) uartHandle;

    if (uartHandle == NULL)
        return DJI_ERROR_SYSTEM_MODULE_CODE_UNKNOWN;

    ret = close(uart_fd->fd);
    if (ret < 0)
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;

    free(uart_fd);

    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}

T_DjiReturnCode HalUart_WriteData(T_DjiUartHandle uartHandle, const uint8_t *buf, uint32_t len, uint32_t *realLen)
{
    int ret;
    struct uart_fd *uart_fd = (struct uart_fd *) uartHandle;

    if (uartHandle == NULL || buf == NULL || len == 0 || realLen == NULL)
        return DJI_ERROR_SYSTEM_MODULE_CODE_INVALID_PARAMETER;

    ret = write(uart_fd->fd, buf, len);
    if (ret >= 0)
        *realLen = ret;
    else 
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;

    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}

T_DjiReturnCode HalUart_ReadData(T_DjiUartHandle uartHandle, uint8_t *buf, uint32_t len, uint32_t *realLen)
{
    int ret;
    struct uart_fd *uart_fd = (struct uart_fd *) uartHandle;

    if (uartHandle == NULL || buf == NULL || len == 0 || realLen == NULL)
        return DJI_ERROR_SYSTEM_MODULE_CODE_INVALID_PARAMETER;

    ret = read(uart_fd->fd, buf, len);
    if (ret >= 0)
        *realLen = ret;
    else
        return DJI_ERROR_SYSTEM_MODULE_CODE_SYSTEM_ERROR;

    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}

T_DjiReturnCode HalUart_GetStatus(E_DjiHalUartNum uartNum, T_DjiUartStatus *status)
{
    if (uartNum == DJI_HAL_UART_NUM_0) 
        status->isConnect = true;
    else if (uartNum == DJI_HAL_UART_NUM_1) 
        status->isConnect = true;
    else
        return DJI_ERROR_SYSTEM_MODULE_CODE_INVALID_PARAMETER;

    return DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS;
}
