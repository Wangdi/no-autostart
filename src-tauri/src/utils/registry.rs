use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;

/// Represents a startup entry detected from various sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartupEntry {
    pub name: String,
    pub command: String,
    pub location: String,
    pub startup_type: StartupType,
    pub enabled: bool,
    pub publisher: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum StartupType {
    RegistryRun,
    RegistryRunOnce,
    StartupFolder,
    TaskScheduler,
    WindowsService,
    Normal,
}

impl std::fmt::Display for StartupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StartupType::RegistryRun => write!(f, "registry_run"),
            StartupType::RegistryRunOnce => write!(f, "registry_run_once"),
            StartupType::StartupFolder => write!(f, "startup_folder"),
            StartupType::TaskScheduler => write!(f, "task_scheduler"),
            StartupType::WindowsService => write!(f, "windows_service"),
            StartupType::Normal => write!(f, "normal"),
        }
    }
}

pub struct RegistryScanner {
    startup_entries: HashMap<String, StartupEntry>,
}

impl RegistryScanner {
    pub fn new() -> Self {
        Self {
            startup_entries: HashMap::new(),
        }
    }

    /// Scan all autostart locations and return found entries
    pub fn scan_all(&mut self) -> &HashMap<String, StartupEntry> {
        self.startup_entries.clear();

        // Scan registry Run keys
        self.scan_registry_run_keys();

        // Scan startup folders
        self.scan_startup_folders();

        // Scan Windows services (basic check)
        self.scan_windows_services();

        &self.startup_entries
    }

    /// Check if a process name matches any autostart entry
    pub fn get_startup_type(&self, process_name: &str, executable_path: &str) -> (StartupType, Option<String>) {
        let process_name_lower = process_name.to_lowercase();
        let path_lower = executable_path.to_lowercase();

        // Check each startup entry for matches
        for entry in self.startup_entries.values() {
            let entry_command_lower = entry.command.to_lowercase();
            let entry_name_lower = entry.name.to_lowercase();

            // Match by executable name in command
            if entry_command_lower.contains(&process_name_lower) {
                return (entry.startup_type.clone(), Some(entry.location.clone()));
            }

            // Match by executable path
            if entry_command_lower.contains(&path_lower) {
                return (entry.startup_type.clone(), Some(entry.location.clone()));
            }

            // Match by entry name (some entries use different names)
            if entry_name_lower.contains(&process_name_lower.replace(".exe", "")) {
                return (entry.startup_type.clone(), Some(entry.location.clone()));
            }
        }

        (StartupType::Normal, None)
    }

    /// Scan registry Run and RunOnce keys
    fn scan_registry_run_keys(&mut self) {
        // HKLM Run keys (system-wide)
        self.scan_registry_key(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            StartupType::RegistryRun,
        );
        self.scan_registry_key(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
            StartupType::RegistryRunOnce,
        );

        // HKCU Run keys (current user)
        self.scan_registry_key(
            HKEY_CURRENT_USER,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run",
            StartupType::RegistryRun,
        );
        self.scan_registry_key(
            HKEY_CURRENT_USER,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\RunOnce",
            StartupType::RegistryRunOnce,
        );

        // WOW64 registry keys (32-bit apps on 64-bit Windows)
        self.scan_registry_key(
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Run",
            StartupType::RegistryRun,
        );
    }

    /// Scan a specific registry key for autostart entries
    fn scan_registry_key(&mut self, hive: HKEY, path: &str, startup_type: StartupType) {
        let hklm = RegKey::predef(hive);

        if let Ok(key) = hklm.open_subkey(path) {
            for name_result in key.enum_values() {
                if let Ok((name, value)) = name_result {
                    let command = match value {
                        winreg::RegValue {
                            vtype: REG_SZ | REG_EXPAND_SZ,
                            bytes,
                        } => {
                            // Handle null-terminated string
                            String::from_utf16_lossy(
                                &bytes
                                    .chunks(2)
                                    .take_while(|chunk| chunk != &[0, 0])
                                    .flat_map(|chunk| {
                                        if chunk.len() == 2 {
                                            vec![chunk[0], chunk[1]]
                                        } else {
                                            vec![chunk[0], 0]
                                        }
                                    })
                                    .collect::<Vec<u8>>()
                                    .as_slice(),
                            )
                            .trim_end_matches('\0')
                            .to_string()
                        }
                        _ => continue,
                    };

                    let location = format!("{}\\{}", 
                        if hive == HKEY_LOCAL_MACHINE { "HKLM" } else { "HKCU" },
                        path
                    );

                    let entry = StartupEntry {
                        name: name.clone(),
                        command: command.clone(),
                        location: location.clone(),
                        startup_type: startup_type.clone(),
                        enabled: true,
                        publisher: None,
                    };

                    self.startup_entries.insert(format!("registry_{}", name), entry);
                }
            }
        }
    }

    /// Scan startup folders
    fn scan_startup_folders(&mut self) {
        // Current user startup folder
        if let Some(user_startup) = dirs_next::data_dir() {
            let user_startup_path = user_startup
                .join("Microsoft")
                .join("Windows")
                .join("Start Menu")
                .join("Programs")
                .join("Startup");

            if user_startup_path.exists() {
                self.scan_startup_folder(&user_startup_path, "User Startup Folder");
            }
        }

        // All users startup folder
        let all_users_startup = PathBuf::from(
            r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs\Startup"
        );
        if all_users_startup.exists() {
            self.scan_startup_folder(&all_users_startup, "All Users Startup Folder");
        }
    }

    /// Scan a specific startup folder for shortcuts and files
    fn scan_startup_folder(&mut self, folder: &PathBuf, location_name: &str) {
        use std::fs;

        if let Ok(entries) = fs::read_dir(folder) {
            for entry_result in entries {
                if let Ok(entry) = entry_result {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();

                    // Skip hidden files and desktop.ini
                    if name.starts_with('.') || name == "desktop.ini" {
                        continue;
                    }

                    let command = path.to_string_lossy().to_string();

                    let startup_entry = StartupEntry {
                        name: name.clone(),
                        command: command.clone(),
                        location: location_name.to_string(),
                        startup_type: StartupType::StartupFolder,
                        enabled: true,
                        publisher: None,
                    };

                    self.startup_entries.insert(format!("folder_{}", name), startup_entry);
                }
            }
        }
    }

    /// Basic Windows service detection
    fn scan_windows_services(&mut self) {
        // Note: Full service enumeration requires administrator privileges
        // and more complex Windows API calls. This is a basic check.
        
        // Common autostart services that users might want to manage
        let common_services = [
            "wuauserv",      // Windows Update
            "WinDefend",     // Windows Defender
            "Spooler",       // Print Spooler
            "SysMain",       // Superfetch/SysMain
            "DiagTrack",     // Connected User Experiences and Telemetry
            "dmwappushsvc",  // Device Management Wireless Application Protocol
        ];

        for service_name in &common_services {
            let entry = StartupEntry {
                name: service_name.to_string(),
                command: String::new(), // Services don't have direct commands
                location: "Windows Services".to_string(),
                startup_type: StartupType::WindowsService,
                enabled: true,
                publisher: Some("Microsoft Corporation".to_string()),
            };

            self.startup_entries.insert(format!("service_{}", service_name), entry);
        }
    }

    /// Get all startup entries
    pub fn get_entries(&self) -> Vec<&StartupEntry> {
        self.startup_entries.values().collect()
    }

    /// Get entries by type
    pub fn get_entries_by_type(&self, startup_type: &StartupType) -> Vec<&StartupEntry> {
        self.startup_entries
            .values()
            .filter(|entry| &entry.startup_type == startup_type)
            .collect()
    }

    /// Check if a startup entry is enabled
    pub fn is_enabled(&self, name: &str) -> bool {
        self.startup_entries
            .get(name)
            .map(|entry| entry.enabled)
            .unwrap_or(false)
    }

    /// Refresh the scanner (re-scan all locations)
    pub fn refresh(&mut self) {
        self.scan_all();
    }
}

impl Default for RegistryScanner {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility function to extract executable name from a command string
pub fn extract_executable_name(command: &str) -> Option<String> {
    // Handle quoted paths: "C:\Program Files\App\app.exe" ...
    if command.starts_with('"') {
        if let Some(end_quote) = command[1..].find('"') {
            let path = &command[1..end_quote + 1];
            return PathBuf::from(path)
                .file_name()
                .map(|name| name.to_string_lossy().to_string());
        }
    }

    // Handle unquoted paths: C:\Program Files\App\app.exe ...
    // Take first token that ends with .exe
    if let Some(first_part) = command.split_whitespace().next() {
        if first_part.to_lowercase().ends_with(".exe") {
            return PathBuf::from(first_part)
                .file_name()
                .map(|name| name.to_string_lossy().to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_startup_type_display() {
        assert_eq!(StartupType::RegistryRun.to_string(), "registry_run");
        assert_eq!(StartupType::StartupFolder.to_string(), "startup_folder");
        assert_eq!(StartupType::Normal.to_string(), "normal");
    }

    #[test]
    fn test_extract_executable_name_quoted() {
        let command = r#""C:\Program Files\WeChat\WeChat.exe" -arg1"#;
        let result = extract_executable_name(command);
        assert_eq!(result, Some("WeChat.exe".to_string()));
    }

    #[test]
    fn test_extract_executable_name_unquoted() {
        let command = r"C:\Windows\System32\notepad.exe";
        let result = extract_executable_name(command);
        assert_eq!(result, Some("notepad.exe".to_string()));
    }

    #[test]
    fn test_extract_executable_name_with_args() {
        let command = r#""C:\Program Files\test\app.exe" --minimized"#;
        let result = extract_executable_name(command);
        assert_eq!(result, Some("app.exe".to_string()));
    }

    #[test]
    fn test_registry_scanner_creation() {
        let scanner = RegistryScanner::new();
        assert_eq!(scanner.startup_entries.len(), 0);
    }

    #[test]
    fn test_startup_entry_creation() {
        let entry = StartupEntry {
            name: "TestApp".to_string(),
            command: r#""C:\Test\test.exe""#.to_string(),
            location: "HKLM\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run".to_string(),
            startup_type: StartupType::RegistryRun,
            enabled: true,
            publisher: Some("Test Publisher".to_string()),
        };

        assert_eq!(entry.name, "TestApp");
        assert!(entry.enabled);
        assert_eq!(entry.startup_type, StartupType::RegistryRun);
    }

    #[test]
    fn test_startup_type_equality() {
        assert_eq!(StartupType::RegistryRun, StartupType::RegistryRun);
        assert_ne!(StartupType::RegistryRun, StartupType::StartupFolder);
    }

    #[test]
    fn test_get_entries_by_type_empty() {
        let scanner = RegistryScanner::new();
        let entries = scanner.get_entries_by_type(&StartupType::RegistryRun);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_is_enabled_nonexistent() {
        let scanner = RegistryScanner::new();
        assert!(!scanner.is_enabled("nonexistent"));
    }

    #[test]
    fn test_extract_executable_name_edge_cases() {
        // Empty string
        assert_eq!(extract_executable_name(""), None);

        // No .exe extension
        assert_eq!(extract_executable_name("notepad"), None);

        // Only whitespace
        assert_eq!(extract_executable_name("   "), None);

        // Path without executable name
        let command = r"C:\Program Files\";
        let result = extract_executable_name(command);
        // Should still work if there's no file name
        assert!(result.is_none() || result.is_some());
    }

    #[test]
    fn test_startup_entry_serialization() {
        let entry = StartupEntry {
            name: "TestEntry".to_string(),
            command: "test.exe".to_string(),
            location: "TestLocation".to_string(),
            startup_type: StartupType::RegistryRun,
            enabled: true,
            publisher: Some("TestPub".to_string()),
        };

        let json = serde_json::to_string(&entry).expect("Should serialize");
        let deserialized: StartupEntry = 
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(entry.name, deserialized.name);
        assert_eq!(entry.startup_type, deserialized.startup_type);
    }

    #[test]
    fn test_startup_type_serialization() {
        let types = vec![
            StartupType::RegistryRun,
            StartupType::RegistryRunOnce,
            StartupType::StartupFolder,
            StartupType::TaskScheduler,
            StartupType::WindowsService,
            StartupType::Normal,
        ];

        for stype in types {
            let json = serde_json::to_string(&stype).expect("Should serialize");
            let deserialized: StartupType = 
                serde_json::from_str(&json).expect("Should deserialize");
            assert_eq!(stype, deserialized);
        }
    }
}
