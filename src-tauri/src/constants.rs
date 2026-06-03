//! Critical system process constants
//! Used across multiple modules to prevent duplication

/// Critical system processes that should never be closed
/// Closing these can cause system instability or crash
pub const CRITICAL_PROCESSES: &[&str] = &[
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

/// System processes that are detected as Windows services
pub const SYSTEM_PROCESSES: &[&str] = &[
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

/// Suspicious patterns in process names that indicate high risk
pub const SUSPICIOUS_PATTERNS: &[&str] = &[
    "miner",
    "crypto",
    "hack",
    "crack",
    "keygen",
    "patch",
    "cheat",
    "aimbot",
];

/// Check if a process name matches a critical process
pub fn is_critical_process(process_name: &str) -> bool {
    let name_lower = process_name.to_lowercase();
    CRITICAL_PROCESSES.iter().any(|p| name_lower.contains(p))
}

/// Check if a process name matches a system process
pub fn is_system_process(process_name: &str) -> bool {
    let name_lower = process_name.to_lowercase();
    SYSTEM_PROCESSES.iter().any(|p| name_lower.contains(p))
}

/// Check if a process name contains suspicious patterns
pub fn has_suspicious_pattern(process_name: &str) -> bool {
    let name_lower = process_name.to_lowercase();
    SUSPICIOUS_PATTERNS.iter().any(|p| name_lower.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_critical_process() {
        assert!(is_critical_process("svchost.exe"));
        assert!(is_critical_process("SVCHOST.EXE"));
        assert!(is_critical_process("explorer.exe"));
        assert!(!is_critical_process("notepad.exe"));
    }

    #[test]
    fn test_is_system_process() {
        assert!(is_system_process("svchost.exe"));
        assert!(is_system_process("runtimebroker.exe"));
        assert!(!is_system_process("chrome.exe"));
    }

    #[test]
    fn test_has_suspicious_pattern() {
        assert!(has_suspicious_pattern("bitcoin-miner.exe"));
        assert!(has_suspicious_pattern("game-hack.exe"));
        assert!(!has_suspicious_pattern("notepad.exe"));
    }

    #[test]
    fn test_critical_processes_count() {
        assert_eq!(CRITICAL_PROCESSES.len(), 11);
    }

    #[test]
    fn test_system_processes_count() {
        assert_eq!(SYSTEM_PROCESSES.len(), 11);
    }

    #[test]
    fn test_suspicious_patterns_count() {
        assert_eq!(SUSPICIOUS_PATTERNS.len(), 8);
    }
}
