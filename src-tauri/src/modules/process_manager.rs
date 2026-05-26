use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sysinfo::{Pid, Process, ProcessStatus, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub executable_path: String,
    pub publisher: Option<String>,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub running_time: u64,
    pub startup_type: String,
    pub startup_location: Option<String>,
    pub risk_level: String,
    pub can_close: bool,
}

pub struct ProcessManager {
    system: System,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    pub fn get_all_processes(&mut self) -> Vec<ProcessInfo> {
        self.refresh();

        let mut processes: Vec<ProcessInfo> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| self.process_to_info(*pid, process))
            .collect();

        // Sort by startup type priority, then by CPU usage
        processes.sort_by(|a, b| {
            let type_priority = |t: &str| -> i32 {
                match t {
                    "registry_run" => 0,
                    "task_scheduler" => 1,
                    "windows_service" => 2,
                    "startup_folder" => 3,
                    "normal" => 4,
                    _ => 5,
                }
            };
            let priority_a = type_priority(&a.startup_type);
            let priority_b = type_priority(&b.startup_type);

            if priority_a != priority_b {
                priority_a.cmp(&priority_b)
            } else {
                b.cpu_usage
                    .partial_cmp(&a.cpu_usage)
                    .unwrap_or(std::cmp::Ordering::Equal)
            }
        });

        processes
    }

    fn process_to_info(&self, pid: Pid, process: &Process) -> ProcessInfo {
        let startup_info = self.detect_startup_type(process);

        ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            executable_path: process
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            publisher: None, // Will be filled by signature verification
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            running_time: process.run_time(),
            startup_type: startup_info.0,
            startup_location: startup_info.1,
            risk_level: String::from("unknown"),
            can_close: true,
        }
    }

    fn detect_startup_type(&self, process: &Process) -> (String, Option<String>) {
        // This will be enhanced with actual registry and task scheduler checks
        let name = process.name().to_string_lossy().to_lowercase();

        // Check for known system processes
        let system_processes = [
            "svchost.exe",
            "csrss.exe",
            "lsass.exe",
            "wininit.exe",
            "services.exe",
            "smss.exe",
            "winlogon.exe",
            "dwm.exe",
            "explorer.exe",
            "runtimebroker.exe",
            "taskhostw.exe",
        ];

        if system_processes.iter().any(|s| name.contains(s)) {
            return (String::from("windows_service"), None);
        }

        (String::from("normal"), None)
    }

    pub fn close_process(&mut self, pid: u32) -> Result<(), String> {
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            process.kill();
            Ok(())
        } else {
            Err(format!("Process {} not found", pid))
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
