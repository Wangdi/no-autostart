use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use windows::Win32::Foundation::{CloseHandle, HANDLE, MAX_PATH};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, TerminateProcess,
    PROCESSENTRY32W, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE, PROCESS_VM_READ,
};
use windows::Win32::System::ProcessStatus::{GetProcessMemoryInfo, PROCESS_MEMORY_COUNTERS};

pub struct ProcessHandle(HANDLE);

impl ProcessHandle {
    pub fn open(pid: u32) -> Result<Self, String> {
        unsafe {
            let handle = OpenProcess(
                PROCESS_QUERY_INFORMATION | PROCESS_VM_READ | PROCESS_TERMINATE,
                false,
                pid,
            )
            .map_err(|e| format!("Failed to open process {}: {}", pid, e))?;
            Ok(ProcessHandle(handle))
        }
    }

    pub fn get_executable_path(&self) -> Result<String, String> {
        unsafe {
            let mut buffer = [0u16; MAX_PATH as usize];
            let mut size = MAX_PATH as u32;

            QueryFullProcessImageNameW(self.0, 0, &mut buffer, &mut size as *mut u32)
                .map_err(|e| format!("Failed to get process path: {}", e))?;

            let path = String::from_utf16_lossy(&buffer[..size as usize]);
            Ok(path)
        }
    }

    pub fn get_memory_info(&self) -> Result<u64, String> {
        unsafe {
            let mut counters = PROCESS_MEMORY_COUNTERS::default();
            GetProcessMemoryInfo(
                self.0,
                &mut counters,
                std::mem::size_of::<PROCESS_MEMORY_COUNTERS>() as u32,
            )
            .map_err(|e| format!("Failed to get memory info: {}", e))?;
            Ok(counters.WorkingSetSize as u64)
        }
    }

    pub fn terminate(&self) -> Result<(), String> {
        unsafe {
            TerminateProcess(self.0, 0).map_err(|e| format!("Failed to terminate process: {}", e))?;
            Ok(())
        }
    }
}

impl Drop for ProcessHandle {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn format_memory(bytes: u64) -> String {
    const MB: u64 = 1024 * 1024;
    if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else {
        format!("{} KB", bytes / 1024)
    }
}

pub fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;

    if hours > 0 {
        format!("{}小时{}分", hours, minutes)
    } else if minutes > 0 {
        format!("{}分{}秒", minutes, secs)
    } else {
        format!("{}秒", secs)
    }
}
