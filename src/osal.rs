use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::ffi::CStr;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
use glob::glob;
use libc::{c_char, c_void};
use chrono::Local;
use crate::{bindings::*, hal::{get_network_handler, get_uart_handler}};

pub const DJI_LOG_FOLDER_NAME: &str = "Logs";
pub const DJI_LOG_INDEX_FILE_NAME: &str = "index";
pub const DJI_LOG_PATH: &str = "DJI";
pub const DJI_LOG_MAX_COUNT: i32 = 10;

static mut LOG_FILE: Option<File> = None;

#[no_mangle]
unsafe extern "C" fn osal_task_create(
        _name: *const c_char, 
        _task_func: Option<unsafe extern "C" fn(arg1: *mut c_void) -> *mut c_void>,
        _stack_size: u32,
        _arg: *mut c_void,
        _task: *mut T_DjiTaskHandle
    ) -> T_DjiReturnCode 
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_task_destroy(_task: T_DjiTaskHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_task_sleep_ms(_time_ms: u32) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_mutex_create(_mutex: *mut T_DjiMutexHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_mutex_destroy(_mutex: T_DjiMutexHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_mutex_lock(_mutex: T_DjiMutexHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_mutex_unlock(_mutex: T_DjiMutexHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_sem_create(_init_val: u32, _sem: *mut T_DjiSemaHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_sem_destroy(_sem: T_DjiSemaHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_sem_wait(_sem: T_DjiSemaHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_sem_timed_wait(_sem: T_DjiSemaHandle, _wait_time_ms: u32) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_sem_post(_sem: T_DjiSemaHandle) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_get_time_ms(_ms: *mut u32) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_get_time_us(_ms: *mut u64) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_get_random_num(_n: *mut u16) -> T_DjiReturnCode
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_malloc(_size: u32) -> *mut c_void
{
    todo!();
}

#[no_mangle]
unsafe extern "C" fn osal_free(_ptr: *mut c_void)
{
    todo!();
}

fn get_osal_handler() -> T_DjiOsalHandler {
    T_DjiOsalHandler {
        // TaskCreate: Some(osal_task_create),
        // TaskDestroy: Some(osal_task_destroy),
        // TaskSleepMs: Some(osal_task_sleep_ms),
        // MutexCreate: Some(osal_mutex_create),
        // MutexDestroy: Some(osal_mutex_destroy),
        // MutexLock: Some(osal_mutex_lock),
        // MutexUnlock: Some(osal_mutex_unlock),
        // SemaphoreCreate: Some(osal_sem_create),
        // SemaphoreDestroy: Some(osal_sem_destroy),
        // SemaphoreWait: Some(osal_sem_wait),
        // SemaphoreTimedWait: Some(osal_sem_timed_wait),
        // SemaphorePost: Some(osal_sem_post),
        // GetTimeMs: Some(osal_get_time_ms),
        // GetTimeUs: Some(osal_get_time_us),
        // GetRandomNum: Some(osal_get_random_num),
        // Malloc: Some(osal_malloc),
        // Free: Some(osal_free),
        TaskCreate: Some(Osal_TaskCreate),
        TaskDestroy: Some(Osal_TaskDestroy),
        TaskSleepMs: Some(Osal_TaskSleepMs),
        MutexCreate: Some(Osal_MutexCreate),
        MutexDestroy: Some(Osal_MutexDestroy),
        MutexLock: Some(Osal_MutexLock),
        MutexUnlock: Some(Osal_MutexUnlock),
        SemaphoreCreate: Some(Osal_SemaphoreCreate),
        SemaphoreDestroy: Some(Osal_SemaphoreDestroy),
        SemaphoreWait: Some(Osal_SemaphoreWait),
        SemaphoreTimedWait: Some(Osal_SemaphoreTimedWait),
        SemaphorePost: Some(Osal_SemaphorePost),
        GetTimeMs: Some(Osal_GetTimeMs),
        GetTimeUs: Some(Osal_GetTimeUs),
        GetRandomNum: Some(Osal_GetRandomNum),
        Malloc: Some(Osal_Malloc),
        Free: Some(Osal_Free),
    }
}

#[no_mangle]
unsafe extern "C" fn print_console(data: *const u8, _len: u16) -> T_DjiReturnCode  {
    println!("{}", CStr::from_ptr(data as *const i8).to_str().unwrap());
    DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS
}

fn get_print_console() -> T_DjiLoggerConsole {
    T_DjiLoggerConsole {
        func: Some(print_console),
        consoleLevel: DJI_LOGGER_CONSOLE_LOG_LEVEL_INFO as u8,
        isSupportColor: true,
    }
}


#[no_mangle]
unsafe extern "C" fn local_write(data: *const u8, len: u16) -> T_DjiReturnCode  {
    if LOG_FILE.is_none() {
        return DJI_ERROR_SYSTEM_MODULE_CODE_UNKNOWN;
    }
    let fp = LOG_FILE.as_mut().unwrap();
    let buf = std::slice::from_raw_parts(data, len as usize);
    fp.write_all(buf).unwrap();
    fp.flush().unwrap();
    DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS
}

fn get_local_record_console() -> T_DjiLoggerConsole {
    T_DjiLoggerConsole {
        func: Some(local_write),
        consoleLevel: DJI_LOGGER_CONSOLE_LOG_LEVEL_DEBUG as u8,
        isSupportColor: true,
    }
}

fn get_file_system_handler() -> T_DjiFileSystemHandler {
    T_DjiFileSystemHandler {
        FileOpen: Some(Osal_FileOpen),
        FileClose: Some(Osal_FileClose),
        FileWrite: Some(Osal_FileWrite),
        FileRead: Some(Osal_FileRead),
        FileSync: Some(Osal_FileSync),
        FileSeek: Some(Osal_FileSeek),
        DirOpen: Some(Osal_DirOpen),
        DirClose: Some(Osal_DirClose),
        DirRead: Some(Osal_DirRead),
        Mkdir: Some(Osal_Mkdir),
        Unlink: Some(Osal_Unlink),
        Rename: Some(Osal_Rename),
        Stat: Some(Osal_Stat),
    }
}

fn get_socket_handler() -> T_DjiSocketHandler {
    T_DjiSocketHandler {
        Socket: Some(Osal_Socket),
        Bind: Some(Osal_Bind),
        Close: Some(Osal_Close),
        UdpSendData: Some(Osal_UdpSendData),
        UdpRecvData: Some(Osal_UdpRecvData),
        TcpListen: Some(Osal_TcpListen),
        TcpAccept: Some(Osal_TcpAccept),
        TcpConnect: Some(Osal_TcpConnect),
        TcpSendData: Some(Osal_TcpSendData),
        TcpRecvData: Some(Osal_TcpRecvData),
    }
}

fn check(err: T_DjiReturnCode) {
    if err != DJI_ERROR_SYSTEM_MODULE_CODE_SUCCESS {
        panic!("Error {}", err);
    }
}

pub fn local_log_init()
{
    let local_time = Local::now();
    let mut log_file_idx: u16 = 0;

    let log_folder = Path::new(DJI_LOG_FOLDER_NAME);
    if !log_folder.exists() {
        fs::create_dir_all(log_folder).unwrap();
    }

    let log_path = log_folder.join(DJI_LOG_PATH);
    let log_path = log_path.to_str().unwrap();

    let index_fname = log_folder.join(DJI_LOG_INDEX_FILE_NAME);
    let mut index_fp: File;
    if index_fname.exists() {
        index_fp = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(index_fname).unwrap()
        ;

        index_fp.seek(SeekFrom::Start(0)).unwrap();
        let mut content = String::new();
        index_fp.read_to_string(&mut content).unwrap();
        log_file_idx = content.parse::<u16>().unwrap();

        index_fp.seek(SeekFrom::Start(0)).unwrap();
        index_fp.write_all(b"").unwrap();

    } else {
        index_fp = File::create(index_fname).unwrap();
    }

    let current_index  = log_file_idx;
    log_file_idx += 1;

    let content = log_file_idx.to_string();
    index_fp.write_all(content.as_bytes()).unwrap();

    let file_path = PathBuf::from(format!("{}_{:04}_{}.log", 
        log_path, current_index, local_time.format("%Y%m%d_%H-%M-%S")));

    let log_fp = File::create(&file_path).unwrap();
    if log_file_idx as i32 > DJI_LOG_MAX_COUNT {
        let pattern = format!("{}_{:04}*", log_path, current_index as i32 - DJI_LOG_MAX_COUNT);
        for entry in glob(&pattern).unwrap() {
            fs::remove_file(entry.unwrap()).unwrap();
        }
    }

    unsafe { LOG_FILE = Some(log_fp) };
    let sym_dst = log_folder.join("latest.log");
    if sym_dst.exists() {
        fs::remove_file(&sym_dst).unwrap();
    }
    symlink(file_path.file_name().unwrap(), &sym_dst).unwrap();
}

pub fn prepare_system_environment() 
{
    let osal_handler = get_osal_handler();
    let mut print_console = get_print_console();
    let mut local_record_console = get_local_record_console();
    let uart_handler = get_uart_handler();
    let network_handler = get_network_handler();
    let sock_handler = get_socket_handler();
    let fs_handler = get_file_system_handler();

    local_log_init();

    unsafe {
        check(DjiPlatform_RegOsalHandler(&osal_handler));
        check(DjiPlatform_RegHalUartHandler(&uart_handler));
        check(DjiLogger_AddConsole(&mut print_console));
        check(DjiLogger_AddConsole(&mut local_record_console));
        check(DjiPlatform_RegHalNetworkHandler(&network_handler));
        check(DjiPlatform_RegSocketHandler(&sock_handler));
        check(DjiPlatform_RegFileSystemHandler(&fs_handler));
    }

}