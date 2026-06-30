use crate::constants;
use crate::utils::cache::{Cache, RateLimiter};
use crate::utils::registry::{RegistryScanner, StartupType};
use crate::utils::security::SecurityPolicy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::Duration;
use sysinfo::{Pid, Process, System};

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
    pub local_description: Option<String>,
    pub is_known_process: bool,
    pub recommendation: Option<String>,
}

pub struct ProcessManager {
    system: System,
    registry_scanner: Mutex<RegistryScanner>,
    security_policy: SecurityPolicy,
    process_cache: Mutex<Cache<u32, ProcessInfo>>,
    rate_limiter: Mutex<RateLimiter>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let mut scanner = RegistryScanner::new();
        scanner.scan_all();

        Self {
            system,
            registry_scanner: Mutex::new(scanner),
            security_policy: SecurityPolicy::default(),
            process_cache: Mutex::new(Cache::new(Duration::from_secs(5))),
            rate_limiter: Mutex::new(RateLimiter::new(Duration::from_millis(500))),
        }
    }

    pub fn refresh(&mut self) {
        // Rate limit refresh calls
        if let Ok(mut limiter) = self.rate_limiter.lock() {
            if !limiter.try_call() {
                return;
            }
        }

        self.system.refresh_all();

        if let Ok(mut scanner) = self.registry_scanner.lock() {
            scanner.refresh();
        }

        // Clean expired cache entries
        if let Ok(mut cache) = self.process_cache.lock() {
            cache.cleanup_expired();
        }
    }

    pub fn get_all_processes(&mut self) -> Vec<ProcessInfo> {
        self.refresh();

        let mut processes: Vec<ProcessInfo> = self
            .system
            .processes()
            .iter()
            .map(|(pid, process)| self.process_to_info(*pid, process))
            .collect();

        processes.sort_by(|a, b| {
            let type_priority = |t: &str| -> i32 {
                match t {
                    "registry_run" => 0,
                    "registry_run_once" => 1,
                    "task_scheduler" => 2,
                    "windows_service" => 3,
                    "startup_folder" => 4,
                    "normal" => 5,
                    _ => 6,
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
        let (startup_type, startup_location) = self.detect_startup_type(process);
        let risk_level = self.assess_risk_level(process, &startup_type);
        let can_close = self.can_safely_close(process, &risk_level);
        let (local_description, is_known_process, recommendation) = self.get_process_knowledge(&process.name().to_string());

        ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            executable_path: process
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            publisher: None,
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            running_time: process.run_time(),
            startup_type,
            startup_location,
            risk_level,
            can_close,
            local_description,
            is_known_process,
            recommendation,
        }
    }

    fn detect_startup_type(&self, process: &Process) -> (String, Option<String>) {
        let name = process.name().to_string();
        let path = process
            .exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        if let Ok(scanner) = self.registry_scanner.lock() {
            let (startup_type, location) = scanner.get_startup_type(&name, &path);
            if startup_type != StartupType::Normal {
                return (startup_type.to_string(), location);
            }
        }

        if constants::is_system_process(&name) {
            return (String::from("windows_service"), None);
        }

        (String::from("normal"), None)
    }

    fn assess_risk_level(&self, process: &Process, startup_type: &str) -> String {
        let name = process.name().to_lowercase();

        if constants::has_suspicious_pattern(&name) {
            return String::from("dangerous");
        }

        if startup_type == "registry_run" || startup_type == "registry_run_once" {
            if let Some(exe_path) = process.exe() {
                let path_str = exe_path.to_string_lossy().to_lowercase();

                if path_str.contains("\\temp\\")
                    || path_str.contains("\\appdata\\local\\temp\\")
                    || path_str.contains("\\users\\public\\")
                {
                    return String::from("warning");
                }
            }
            return String::from("low");
        }

        if constants::is_system_process(&name) {
            return String::from("safe");
        }

        if startup_type == "normal" {
            return String::from("unknown");
        }

        String::from("low")
    }

    fn can_safely_close(&self, process: &Process, risk_level: &str) -> bool {
        let name = process.name().to_lowercase();

        if constants::is_critical_process(&name) {
            return false;
        }

        if risk_level == "safe" && constants::is_system_process(&name) {
            return false;
        }

        true
    }

    fn get_process_knowledge(&self, name: &str) -> (Option<String>, bool, Option<String>) {
        let name_lower = name.to_lowercase();
        
        let known_processes = [
            ("wechat.exe", "微信PC客户端", "可关闭。如不需要开机自启，建议禁用启动项"),
            ("qq.exe", "腾讯QQ", "可关闭。建议禁用开机自启"),
            ("chrome.exe", "Google Chrome浏览器", "可关闭。建议关闭后台运行"),
            ("msedge.exe", "Microsoft Edge浏览器", "可关闭。建议关闭后台运行"),
            ("code.exe", "Visual Studio Code", "可关闭"),
            ("svchost.exe", "Windows服务主机进程", "系统关键进程，请勿关闭"),
            ("explorer.exe", "Windows资源管理器", "系统关键进程，关闭会导致桌面消失"),
        ];

        for (proc_name, desc, rec) in &known_processes {
            if name_lower.contains(&proc_name.to_lowercase()) {
                let is_system = proc_name.contains("svchost") || proc_name.contains("explorer");
                return (Some(desc.to_string()), true, Some(rec.to_string()));
            }
        }

        (None, false, None)
    }

    pub fn close_process(&mut self, pid: u32) -> Result<(), String> {
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            let name = process.name().to_lowercase();

            if constants::is_critical_process(&name) {
                return Err(format!(
                    "Cannot close critical system process: {}",
                    process.name()
                ));
            }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager_new_creates_valid_instance() {
        let manager = ProcessManager::new();
        assert!(manager.system.processes().len() > 0);
    }

    #[test]
    fn test_get_all_processes_returns_sorted() {
        let mut manager = ProcessManager::new();
        let processes = manager.get_all_processes();

        assert!(!processes.is_empty());

        for i in 0..processes.len().saturating_sub(1) {
            let current = &processes[i];
            let next = &processes[i + 1];

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

            let current_priority = type_priority(&current.startup_type);
            let next_priority = type_priority(&next.startup_type);

            assert!(
                current_priority <= next_priority,
                "Processes should be sorted by startup type priority"
            );
        }
    }

    #[test]
    fn test_close_process_returns_error_for_nonexistent_pid() {
        let mut manager = ProcessManager::new();
        let result = manager.close_process(999999);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_process_info_struct_creation() {
        let info = ProcessInfo {
            pid: 1234,
            name: "test.exe".to_string(),
            executable_path: "C:\\test\\test.exe".to_string(),
            publisher: Some("Test Publisher".to_string()),
            cpu_usage: 15.5,
            memory_usage: 1024000,
            running_time: 3600,
            startup_type: "normal".to_string(),
            startup_location: None,
            risk_level: "unknown".to_string(),
            can_close: true,
            local_description: None,
            is_known_process: false,
            recommendation: None,
        };

        assert_eq!(info.pid, 1234);
        assert_eq!(info.name, "test.exe");
        assert!(info.can_close);
    }

    #[test]
    fn test_process_info_serialization() {
        let info = ProcessInfo {
            pid: 5678,
            name: "process.exe".to_string(),
            executable_path: "C:\\path\\process.exe".to_string(),
            publisher: None,
            cpu_usage: 5.0,
            memory_usage: 512000,
            running_time: 120,
            startup_type: "registry_run".to_string(),
            startup_location: Some("HKLM\\Run".to_string()),
            risk_level: "low".to_string(),
            can_close: false,
            local_description: Some("Test process".to_string()),
            is_known_process: true,
            recommendation: Some("Can be closed".to_string()),
        };

        let json = serde_json::to_string(&info).expect("Should serialize");
        let deserialized: ProcessInfo = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(info.pid, deserialized.pid);
        assert_eq!(info.name, deserialized.name);
        assert_eq!(info.local_description, deserialized.local_description);
    }

    #[test]
    fn test_process_info_pid_uniqueness() {
        let mut manager = ProcessManager::new();
        let processes = manager.get_all_processes();

        let mut pids: Vec<u32> = processes.iter().map(|p| p.pid).collect();
        pids.sort();
        pids.dedup();

        assert_eq!(pids.len(), processes.len());
    }
}
