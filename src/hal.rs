use std::ffi::{CString, CStr};
// use libc::{c_char, c_void};
use crate::bindings;
use crate::bindings::*;

pub fn set_uart_dev(dev_no: i32, name: &str) {
    let c_name = CString::new(name).unwrap();
    let c_name = c_name.as_c_str().as_ptr();
    unsafe {
        if dev_no == 0 {
            bindings::set_uart_dev0(c_name);
        } else if dev_no == 1 {
            bindings::set_uart_dev1(c_name);
        } else {
            panic!("unsupported dev_no {}", dev_no);
        }
    }
}

pub fn get_uart_dev(dev_no: i32) -> String {
    let c_name = if dev_no == 0 {
        unsafe { CStr::from_ptr(LINUX_UART_DEV0.as_ptr()) }
    } else if dev_no == 1 {
        unsafe { CStr::from_ptr(LINUX_UART_DEV1.as_ptr()) }
    } else {
        panic!("unsupported dev_no {}", dev_no);
    };
    c_name.to_str().unwrap().to_owned()
}

pub fn get_uart_handler() -> T_DjiHalUartHandler {
    T_DjiHalUartHandler {
        UartInit: Some(HalUart_Init),
        UartDeInit: Some(HalUart_DeInit),
        UartWriteData: Some(HalUart_WriteData),
        UartReadData: Some(HalUart_ReadData),
        UartGetStatus: Some(HalUart_GetStatus),
    }
}

pub fn set_network_dev(name: &str) {
    let c_name = CString::new(name).unwrap();
    let c_name = c_name.as_c_str().as_ptr();
    unsafe {bindings::set_network_dev(c_name);}
}

pub fn get_network_dev() -> String {
    let c_name = unsafe { CStr::from_ptr(LINUX_NETWORK_DEV.as_ptr()) };
    c_name.to_str().unwrap().to_owned()
}

pub fn get_network_handler() -> T_DjiHalNetworkHandler {
    T_DjiHalNetworkHandler {
        NetworkInit: Some(HalNetwork_Init),
        NetworkDeInit: Some(HalNetwork_DeInit),
        NetworkGetDeviceInfo: Some(HalNetwork_GetDeviceInfo),
    }
}