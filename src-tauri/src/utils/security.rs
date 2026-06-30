use crate::constants;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Security policy for process management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    /// Processes that should never be closed (whitelist)
    pub protected_processes: HashSet<String>,
    /// Processes that require confirmation before closing
    pub confirmation_required: HashSet<String>,
    /// Maximum number of processes that can be closed in one batch
    pub max_batch_close: usize,
    /// Enable protection for system processes
    pub protect_system_processes: bool,
    /// Enable protection for user-critical processes
    pub protect_user_processes: bool,
    /// Risk levels that require confirmation
    pub risky_levels_require_confirmation: HashSet<String>,
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        // Use constants for critical processes
        let protected: HashSet<String> = constants::CRITICAL_PROCESSES
            .iter()
            .map(|s| s.to_lowercase())
            .collect();

        let mut confirmation = HashSet::new();
        let confirmation_list = [
            "chrome.exe",
            "firefox.exe",
            "msedge.exe",
            "code.exe",
            "idea64.exe",
            "devenv.exe",
            "slack.exe",
            "teams.exe",
            "zoom.exe",
            "discord.exe",
            "wechat.exe",
            "qq.exe",
            "tim.exe",
            "dingtalk.exe",
            "feishu.exe",
        ];
        for proc in confirmation_list {
            confirmation.insert(proc.to_lowercase());
        }

        let mut risky_levels = HashSet::new();
        risky_levels.insert("dangerous".to_string());
        risky_levels.insert("warning".to_string());

        Self {
            protected_processes: protected,
            confirmation_required: confirmation,
            max_batch_close: 10,
            protect_system_processes: true,
            protect_user_processes: true,
            risky_levels_require_confirmation: risky_levels,
        }
    }
}

impl SecurityPolicy {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if a process is protected
    pub fn is_protected(&self, process_name: &str) -> bool {
        self.protected_processes.contains(&process_name.to_lowercase())
    }

    /// Check if closing a process requires confirmation
    pub fn requires_confirmation(&self, process_name: &str, risk_level: &str) -> bool {
        self.confirmation_required.contains(&process_name.to_lowercase())
            || self.risky_levels_require_confirmation.contains(risk_level)
    }

    /// Check if batch close operation is allowed
    pub fn can_batch_close(&self, count: usize) -> bool {
        count <= self.max_batch_close
    }

    /// Add a process to the protected list
    pub fn protect(&mut self, process_name: String) {
        self.protected_processes.insert(process_name.to_lowercase());
    }

    /// Remove a process from the protected list
    pub fn unprotect(&mut self, process_name: &str) -> bool {
        self.protected_processes.remove(&process_name.to_lowercase())
    }

    /// Add a process to the confirmation required list
    pub fn require_confirmation(&mut self, process_name: String) {
        self.confirmation_required.insert(process_name.to_lowercase());
    }

    /// Remove a process from the confirmation required list
    pub fn remove_confirmation_requirement(&mut self, process_name: &str) -> bool {
        self.confirmation_required.remove(&process_name.to_lowercase())
    }

    /// Validate if a process can be closed
    pub fn validate_close(&self, process_name: &str, risk_level: &str) -> CloseValidation {
        if self.is_protected(process_name) {
            return CloseValidation::Protected;
        }

        if self.requires_confirmation(process_name, risk_level) {
            return CloseValidation::RequiresConfirmation;
        }

        CloseValidation::Allowed
    }
}

/// Result of validating a close operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CloseValidation {
    /// The process can be closed
    Allowed,
    /// The process is protected and cannot be closed
    Protected,
    /// The process can be closed but requires confirmation
    RequiresConfirmation,
}

/// Process whitelist manager
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhitelistManager {
    /// Processes that are always allowed to run
    pub allowed_processes: HashSet<String>,
    /// Processes that are known safe (from knowledge base)
    pub known_safe: HashSet<String>,
}

impl Default for WhitelistManager {
    fn default() -> Self {
        let allowed: HashSet<String> = constants::SYSTEM_PROCESSES
            .iter()
            .filter(|p| {
                ["svchost.exe", "explorer.exe", "dwm.exe", "runtimebroker.exe", "taskhostw.exe"]
                    .contains(p)
            })
            .map(|s| s.to_lowercase())
            .collect();

        Self {
            allowed_processes: allowed,
            known_safe: HashSet::new(),
        }
    }
}

impl WhitelistManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_allowed(&self, process_name: &str) -> bool {
        self.allowed_processes.contains(&process_name.to_lowercase())
    }

    pub fn is_known_safe(&self, process_name: &str) -> bool {
        self.known_safe.contains(&process_name.to_lowercase())
    }

    pub fn add_allowed(&mut self, process_name: String) {
        self.allowed_processes.insert(process_name.to_lowercase());
    }

    pub fn remove_allowed(&mut self, process_name: &str) -> bool {
        self.allowed_processes.remove(&process_name.to_lowercase())
    }

    pub fn add_known_safe(&mut self, process_name: String) {
        self.known_safe.insert(process_name.to_lowercase());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_policy_default() {
        let policy = SecurityPolicy::default();
        assert!(policy.protect_system_processes);
        assert!(policy.protect_user_processes);
        assert!(!policy.protected_processes.is_empty());
    }

    #[test]
    fn test_is_protected() {
        let policy = SecurityPolicy::default();
        assert!(policy.is_protected("svchost.exe"));
        assert!(policy.is_protected("SVCHOST.EXE"));
        assert!(policy.is_protected("explorer.exe"));
        assert!(!policy.is_protected("notepad.exe"));
    }

    #[test]
    fn test_requires_confirmation() {
        let policy = SecurityPolicy::default();
        assert!(policy.requires_confirmation("chrome.exe", "safe"));
        assert!(policy.requires_confirmation("wechat.exe", "safe"));
        assert!(!policy.requires_confirmation("notepad.exe", "safe"));
        assert!(policy.requires_confirmation("suspicious.exe", "dangerous"));
    }

    #[test]
    fn test_can_batch_close() {
        let policy = SecurityPolicy::default();
        assert!(policy.can_batch_close(5));
        assert!(policy.can_batch_close(10));
        assert!(!policy.can_batch_close(15));
    }

    #[test]
    fn test_protect_unprotect() {
        let mut policy = SecurityPolicy::default();

        policy.protect("test.exe".to_string());
        assert!(policy.is_protected("test.exe"));

        assert!(policy.unprotect("test.exe"));
        assert!(!policy.is_protected("test.exe"));

        assert!(!policy.unprotect("nonexistent.exe"));
    }

    #[test]
    fn test_validate_close() {
        let policy = SecurityPolicy::default();

        assert_eq!(
            policy.validate_close("svchost.exe", "safe"),
            CloseValidation::Protected
        );

        assert_eq!(
            policy.validate_close("chrome.exe", "safe"),
            CloseValidation::RequiresConfirmation
        );

        assert_eq!(
            policy.validate_close("notepad.exe", "safe"),
            CloseValidation::Allowed
        );

        assert_eq!(
            policy.validate_close("unknown.exe", "dangerous"),
            CloseValidation::RequiresConfirmation
        );
    }

    #[test]
    fn test_whitelist_manager_default() {
        let manager = WhitelistManager::default();
        assert!(!manager.allowed_processes.is_empty());
    }

    #[test]
    fn test_whitelist_operations() {
        let mut manager = WhitelistManager::new();

        manager.add_allowed("test.exe".to_string());
        assert!(manager.is_allowed("test.exe"));

        assert!(manager.remove_allowed("test.exe"));
        assert!(!manager.is_allowed("test.exe"));

        manager.add_known_safe("safe.exe".to_string());
        assert!(manager.is_known_safe("safe.exe"));
    }

    #[test]
    fn test_security_policy_serialization() {
        let policy = SecurityPolicy::default();
        let json = serde_json::to_string(&policy).expect("Should serialize");
        let deserialized: SecurityPolicy =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(
            policy.protected_processes.len(),
            deserialized.protected_processes.len()
        );
    }

    #[test]
    fn test_whitelist_serialization() {
        let mut manager = WhitelistManager::new();
        manager.add_allowed("test.exe".to_string());

        let json = serde_json::to_string(&manager).expect("Should serialize");
        let deserialized: WhitelistManager =
            serde_json::from_str(&json).expect("Should deserialize");

        assert!(deserialized.is_allowed("test.exe"));
    }
}
