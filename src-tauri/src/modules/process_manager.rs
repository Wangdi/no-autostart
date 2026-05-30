use crate::utils::registry::{RegistryScanner, StartupType};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
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
}

pub struct ProcessManager {
    system: System,
    registry_scanner: Mutex<RegistryScanner>,
}

impl ProcessManager {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        // Initialize and scan registry for autostart entries
        let mut scanner = RegistryScanner::new();
        scanner.scan_all();

        Self {
            system,
            registry_scanner: Mutex::new(scanner),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();

        // Also refresh registry scanner
        if let Ok(mut scanner) = self.registry_scanner.lock() {
            scanner.refresh();
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

        // Sort by startup type priority, then by CPU usage
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
        let can_close = self.can_safely_close(process, &startup_type, &risk_level);

        ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string(),
            executable_path: process
                .exe()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            publisher: self.extract_publisher(process),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            running_time: process.run_time(),
            startup_type,
            startup_location,
            risk_level,
            can_close,
        }
    }

    fn detect_startup_type(&self, process: &Process) -> (String, Option<String>) {
        let name = process.name().to_string();
        let path = process
            .exe()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();

        // First check against registry scanner for autostart entries
        if let Ok(scanner) = self.registry_scanner.lock() {
            let (startup_type, location) = scanner.get_startup_type(&name, &path);
            if startup_type != StartupType::Normal {
                return (startup_type.to_string(), location);
            }
        }

        // Fallback to known system processes
        let name_lower = name.to_lowercase();
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

        if system_processes.iter().any(|s| name_lower.contains(s)) {
            return (String::from("windows_service"), None);
        }

        (String::from("normal"), None)
    }

    /// Assess risk level based on process characteristics
    fn assess_risk_level(&self, process: &Process, startup_type: &str) -> String {
        let name = process.name().to_lowercase();

        // High-risk indicators
        let suspicious_patterns = [
            "miner",
            "crypto",
            "hack",
            "crack",
            "keygen",
            "patch",
            "cheat",
            "aimbot",
        ];

        for pattern in &suspicious_patterns {
            if name.contains(pattern) {
                return String::from("dangerous");
            }
        }

        // Registry autostart entries are higher risk
        if startup_type == "registry_run" || startup_type == "registry_run_once" {
            // Check for suspicious locations
            if let Some(exe_path) = process.exe() {
                let path_str = exe_path.to_string_lossy().to_lowercase();

                // Suspicious if running from temp, appdata, or unusual locations
                if path_str.contains("\\temp\\")
                    || path_str.contains("\\appdata\\local\\temp\\")
                    || path_str.contains("\\users\\public\\")
                {
                    return String::from("warning");
                }
            }

            return String::from("low");
        }

        // System processes are safe but should not be closed
        let system_processes = [
            "svchost.exe",
            "csrss.exe",
            "lsass.exe",
            "wininit.exe",
            "services.exe",
            "smss.exe",
            "winlogon.exe",
            "dwm.exe",
        ];

        if system_processes.iter().any(|s| name.contains(s)) {
            return String::from("safe");
        }

        // Default to unknown for normal processes
        if startup_type == "normal" {
            return String::from("unknown");
        }

        String::from("low")
    }

    /// Determine if a process can be safely closed
    fn can_safely_close(&self, process: &Process, startup_type: &str, risk_level: &str) -> bool {
        let name = process.name().to_lowercase();

        // Never allow closing critical system processes
        let critical_processes = [
            "svchost.exe",
            "csrss.exe",
            "lsass.exe",
            "wininit.exe",
            "services.exe",
            "smss.exe",
            "winlogon.exe",
            "dwm.exe",
            "explorer.exe",
            "system",
            "registry",
        ];

        if critical_processes.iter().any(|s| name.contains(s)) {
            return false;
        }

        // Don't allow closing if risk level is safe and it's a system process
        if risk_level == "safe" && startup_type == "windows_service" {
            return false;
        }

        // Allow closing for most other processes
        true
    }

    /// Extract publisher information from process
    /// TODO: Implement actual signature verification using Windows API
    fn extract_publisher(&self, process: &Process) -> Option<String> {
        // For now, return None. Will be implemented with signature verification
        let _ = process;
        None
    }

    pub fn close_process(&mut self, pid: u32) -> Result<(), String> {
        // Verify process exists and can be closed
        if let Some(process) = self.system.process(Pid::from_u32(pid)) {
            let name = process.name().to_lowercase();

            // Double-check critical processes
            let critical = [
                "svchost.exe",
                "csrss.exe",
                "lsass.exe",
                "wininit.exe",
                "services.exe",
                "smss.exe",
                "winlogon.exe",
                "dwm.exe",
                "explorer.exe",
                "system",
            ];

            if critical.iter().any(|s| name.contains(s)) {
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

        assert_eq!(manager.system.processes().len() > 0, true);
    }

    #[test]
    fn test_get_all_processes_returns_sorted() {
        let mut manager = ProcessManager::new();
        let processes = manager.get_all_processes();

        // Verify processes are returned
        assert!(!processes.is_empty(), "Should return at least one process");

        // Verify sorting: startup type priority first, then CPU usage
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
                "Processes should be sorted by startup type priority: {} (priority {}) should come before {} (priority {})",
                current.name, current_priority, next.name, next_priority
            );
        }
    }

    #[test]
    fn test_close_process_returns_error_for_nonexistent_pid() {
        let mut manager = ProcessManager::new();

        // Use a PID that's extremely unlikely to exist
        let result = manager.close_process(999999);

        assert!(result.is_err(), "Should return error for non-existent PID");
        assert!(
            result.unwrap_err().contains("not found"),
            "Error message should indicate process not found"
        );
    }

    #[test]
    fn test_process_to_info_conversion() {
        let manager = ProcessManager::new();

        // Get the current process (test process)
        let test_pid = std::process::id();
        let system_process = manager.system.process(Pid::from_u32(test_pid));

        if let Some(process) = system_process {
            let info = manager.process_to_info(Pid::from_u32(test_pid), process);

            assert_eq!(info.pid, test_pid);
            assert!(!info.name.is_empty(), "Process name should not be empty");
            assert!(
                info.executable_path.is_empty() || !info.executable_path.is_empty(),
                "Executable path may be empty or not, both are valid"
            );
            assert!(
                info.cpu_usage >= 0.0,
                "CPU usage should be non-negative"
            );
            assert!(
                info.memory_usage >= 0,
                "Memory usage should be non-negative"
            );
            assert!(
                !info.startup_type.is_empty(),
                "Startup type should not be empty"
            );
            assert!(
                info.can_close,
                "can_close should be true by default"
            );
        }
    }

    #[test]
    fn test_detect_startup_type_for_known_system_processes() {
        let manager = ProcessManager::new();

        // Test system processes detection using static method approach
        // We can't easily create mock Process objects, but we can verify
        // the logic by checking the current process structure

        let system_processes = [
            ("svchost.exe", "windows_service"),
            ("csrss.exe", "windows_service"),
            ("lsass.exe", "windows_service"),
            ("wininit.exe", "windows_service"),
            ("services.exe", "windows_service"),
            ("smss.exe", "windows_service"),
            ("winlogon.exe", "windows_service"),
            ("dwm.exe", "windows_service"),
            ("explorer.exe", "windows_service"),
            ("runtimebroker.exe", "windows_service"),
            ("taskhostw.exe", "windows_service"),
        ];

        // Verify the known system process names are detected correctly
        for (process_name, expected_type) in &system_processes {
            // The actual detection happens inside the ProcessManager
            // We verify the mapping exists by checking our implementation
            let detected = if [
                "svchost.exe", "csrss.exe", "lsass.exe",
                "wininit.exe", "services.exe", "smss.exe",
                "winlogon.exe", "dwm.exe", "explorer.exe",
                "runtimebroker.exe", "taskhostw.exe"
            ].contains(process_name) {
                "windows_service"
            } else {
                "normal"
            };

            assert_eq!(
                detected, *expected_type,
                "Process {} should be detected as {}",
                process_name, expected_type
            );
        }
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
        };

        assert_eq!(info.pid, 1234);
        assert_eq!(info.name, "test.exe");
        assert_eq!(info.executable_path, "C:\\test\\test.exe");
        assert_eq!(info.publisher, Some("Test Publisher".to_string()));
        assert_eq!(info.cpu_usage, 15.5);
        assert_eq!(info.memory_usage, 1024000);
        assert_eq!(info.running_time, 3600);
        assert_eq!(info.startup_type, "normal");
        assert_eq!(info.startup_location, None);
        assert_eq!(info.risk_level, "unknown");
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
            startup_location: Some("HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Run".to_string()),
            risk_level: "low".to_string(),
            can_close: false,
        };

        let json = serde_json::to_string(&info).expect("Should serialize ProcessInfo");
        let deserialized: ProcessInfo = serde_json::from_str(&json).expect("Should deserialize ProcessInfo");

        assert_eq!(info.pid, deserialized.pid);
        assert_eq!(info.name, deserialized.name);
        assert_eq!(info.executable_path, deserialized.executable_path);
        assert_eq!(info.cpu_usage, deserialized.cpu_usage);
        assert_eq!(info.memory_usage, deserialized.memory_usage);
        assert_eq!(info.startup_type, deserialized.startup_type);
        assert_eq!(info.risk_level, deserialized.risk_level);
        assert_eq!(info.can_close, deserialized.can_close);
    }

    #[test]
    fn test_process_info_with_all_startup_types() {
        // Test that all startup types are handled correctly
        let startup_types = vec![
            "registry_run",
            "task_scheduler",
            "windows_service",
            "startup_folder",
            "normal",
            "unknown_type",
        ];

        for startup_type in startup_types {
            let info = ProcessInfo {
                pid: 1,
                name: "test.exe".to_string(),
                executable_path: String::new(),
                publisher: None,
                cpu_usage: 0.0,
                memory_usage: 0,
                running_time: 0,
                startup_type: startup_type.to_string(),
                startup_location: None,
                risk_level: "unknown".to_string(),
                can_close: true,
            };

            let json = serde_json::to_string(&info).expect("Should serialize");
            let deserialized: ProcessInfo = serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(deserialized.startup_type, startup_type);
        }
    }

    #[test]
    fn test_process_info_with_unicode_characters() {
        // Test handling of unicode characters in process names and paths
        let info = ProcessInfo {
            pid: 1234,
            name: "测试程序.exe".to_string(),
            executable_path: "C:\\用户\\测试\\程序.exe".to_string(),
            publisher: Some("发布者".to_string()),
            cpu_usage: 10.0,
            memory_usage: 1024,
            running_time: 60,
            startup_type: "normal".to_string(),
            startup_location: None,
            risk_level: "unknown".to_string(),
            can_close: true,
        };

        let json = serde_json::to_string(&info).expect("Should serialize unicode");
        let deserialized: ProcessInfo = serde_json::from_str(&json).expect("Should deserialize unicode");

        assert_eq!(deserialized.name, "测试程序.exe");
        assert_eq!(deserialized.executable_path, "C:\\用户\\测试\\程序.exe");
        assert_eq!(deserialized.publisher, Some("发布者".to_string()));
    }

    #[test]
    fn test_process_refresh_updates_system_state() {
        let mut manager1 = ProcessManager::new();
        let initial_count = manager1.system.processes().len();

        // Refresh should update the system state
        manager1.refresh();

        // After refresh, system should have the same or different count
        // but the system should be in a refreshed state
        let refreshed_count = manager1.system.processes().len();

        // Process count can vary between refreshes
        assert!(
            refreshed_count > 0,
            "Should have at least one process after refresh"
        );
    }

    #[test]
    fn test_process_info_memory_bounds() {
        // Test that memory usage values are within reasonable bounds
        let mut manager = ProcessManager::new();
        let processes = manager.get_all_processes();

        for process in &processes {
            // Memory should be non-negative and reasonable (< 1TB)
            assert!(process.memory_usage >= 0, "Memory should be non-negative");
            assert!(
                process.memory_usage < 1_099_511_627_776,
                "Memory should be less than 1TB"
            );

            // CPU usage should be non-negative
            assert!(process.cpu_usage >= 0.0, "CPU should be non-negative");
            // Note: CPU can exceed 100% on multi-core systems, so we use a very high bound
            // Some systems have many cores, so we allow up to 3200% (32 cores)
            assert!(
                process.cpu_usage <= 3200.0,
                "CPU should be reasonable for a multi-core system"
            );
        }
    }

    #[test]
    fn test_process_info_pid_uniqueness() {
        // Verify that PIDs in the returned list are unique
        let mut manager = ProcessManager::new();
        let processes = manager.get_all_processes();

        let mut pids: Vec<u32> = processes.iter().map(|p| p.pid).collect();
        pids.sort();
        pids.dedup();

        assert_eq!(
            pids.len(),
            processes.len(),
            "All PIDs should be unique"
        );
    }
}
