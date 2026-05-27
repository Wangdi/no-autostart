use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessKnowledgeItem {
    pub process_name: String,
    pub description: String,
    pub function: String,
    pub startup_method: String,
    pub performance_impact: String,
    pub can_close: bool,
    pub recommendation: String,
    pub risk_level: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessKnowledge {
    pub processes: Vec<ProcessKnowledgeItem>,
}

impl Default for ProcessKnowledge {
    fn default() -> Self {
        Self {
            processes: Vec::new(),
        }
    }
}

pub struct KnowledgeManager {
    knowledge: HashMap<String, ProcessKnowledgeItem>,
    knowledge_path: PathBuf,
}

impl KnowledgeManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        Self::new_with_path(data_dir)
    }

    // Test-friendly constructor
    pub fn new_with_path(data_dir: PathBuf) -> Result<Self, String> {
        // Ensure data directory exists
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create data dir: {}", e))?;

        let knowledge_path = data_dir.join("process_knowledge.json");

        let knowledge = if knowledge_path.exists() {
            let content = fs::read_to_string(&knowledge_path)
                .map_err(|e| format!("Failed to read knowledge: {}", e))?;
            let parsed: ProcessKnowledge =
                serde_json::from_str(&content).unwrap_or_else(|_| ProcessKnowledge::default());
            parsed
                .processes
                .into_iter()
                .map(|item| (item.process_name.to_lowercase(), item))
                .collect()
        } else {
            // Load default knowledge
            let default_knowledge = Self::get_default_knowledge();
            fs::write(
                &knowledge_path,
                serde_json::to_string_pretty(&default_knowledge).unwrap(),
            )
            .ok();
            default_knowledge
                .processes
                .into_iter()
                .map(|item| (item.process_name.to_lowercase(), item))
                .collect()
        };

        Ok(Self {
            knowledge,
            knowledge_path,
        })
    }

    pub fn lookup(&self, process_name: &str) -> Option<&ProcessKnowledgeItem> {
        self.knowledge.get(&process_name.to_lowercase())
    }

    pub fn get_all(&self) -> Vec<&ProcessKnowledgeItem> {
        self.knowledge.values().collect()
    }

    pub fn reload(&mut self) -> Result<(), String> {
        if self.knowledge_path.exists() {
            let content = fs::read_to_string(&self.knowledge_path)
                .map_err(|e| format!("Failed to read knowledge: {}", e))?;
            let parsed: ProcessKnowledge =
                serde_json::from_str(&content).unwrap_or_else(|_| ProcessKnowledge::default());
            self.knowledge = parsed
                .processes
                .into_iter()
                .map(|item| (item.process_name.to_lowercase(), item))
                .collect();
        }
        Ok(())
    }

    fn get_default_knowledge() -> ProcessKnowledge {
        ProcessKnowledge {
            processes: vec![
                ProcessKnowledgeItem {
                    process_name: "WeChat.exe".to_string(),
                    description: "微信PC客户端".to_string(),
                    function: "即时通讯和社交".to_string(),
                    startup_method: "注册表 Run 键，开机自动启动".to_string(),
                    performance_impact: "内存占用中等，CPU使用低，后台静默时影响较小".to_string(),
                    can_close: true,
                    recommendation: "可关闭。如不需要开机自启，建议禁用启动项".to_string(),
                    risk_level: "safe".to_string(),
                    tags: vec!["社交".to_string(), "通讯".to_string()],
                },
                ProcessKnowledgeItem {
                    process_name: "svchost.exe".to_string(),
                    description: "Windows服务主机进程".to_string(),
                    function: "承载多个Windows系统服务".to_string(),
                    startup_method: "系统核心进程".to_string(),
                    performance_impact: "正常情况下资源占用低".to_string(),
                    can_close: false,
                    recommendation: "系统关键进程，请勿关闭".to_string(),
                    risk_level: "warning".to_string(),
                    tags: vec!["系统".to_string(), "核心".to_string()],
                },
                ProcessKnowledgeItem {
                    process_name: "explorer.exe".to_string(),
                    description: "Windows资源管理器".to_string(),
                    function: "Windows桌面和文件管理".to_string(),
                    startup_method: "系统核心进程".to_string(),
                    performance_impact: "正常资源占用".to_string(),
                    can_close: false,
                    recommendation: "系统关键进程，关闭会导致桌面消失".to_string(),
                    risk_level: "warning".to_string(),
                    tags: vec!["系统".to_string(), "核心".to_string()],
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn get_test_dir() -> PathBuf {
        let counter = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let mut path = env::temp_dir();
        path.push(format!("no_autostart_knowledge_test_{}_{}", std::process::id(), counter));
        path
    }

    fn cleanup_test_dir(path: &PathBuf) {
        let _ = fs::remove_dir_all(path);
    }

    #[test]
    fn test_lookup_returns_correct_item() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        // Test exact case lookup
        let result = manager.lookup("WeChat.exe");
        assert!(result.is_some(), "Should find WeChat.exe");
        assert_eq!(result.unwrap().process_name, "WeChat.exe");

        // Test svchost lookup
        let result = manager.lookup("svchost.exe");
        assert!(result.is_some(), "Should find svchost.exe");
        let item = result.unwrap();
        assert_eq!(item.process_name, "svchost.exe");
        assert!(!item.can_close);
        assert_eq!(item.risk_level, "warning");

        // Test explorer lookup
        let result = manager.lookup("explorer.exe");
        assert!(result.is_some(), "Should find explorer.exe");
        let item = result.unwrap();
        assert_eq!(item.process_name, "explorer.exe");
        assert!(!item.can_close);

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_lookup_is_case_insensitive() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        // Various case variations
        let variations = vec![
            "wechat.exe",
            "WECHAT.EXE",
            "WeChat.exe",
            "weCHAT.exe",
            "SVCHOST.EXE",
            "svchost.exe",
            "Svchost.exe",
            "EXPLORER.EXE",
            "explorer.exe",
            "Explorer.exe",
        ];

        for variation in variations {
            let result = manager.lookup(variation);
            assert!(
                result.is_some(),
                "Should find process with case-insensitive lookup for '{}'",
                variation
            );
        }

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_lookup_returns_none_for_unknown_process() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        let result = manager.lookup("unknown-process.exe");
        assert!(result.is_none(), "Should return None for unknown process");

        let result = manager.lookup("");
        assert!(result.is_none(), "Should return None for empty string");

        let result = manager.lookup("not-in-database.exe");
        assert!(result.is_none(), "Should return None for non-existent process");

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_get_all_returns_all_items() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        let all_items = manager.get_all();
        assert_eq!(all_items.len(), 3, "Should return all 3 default items");

        // Verify all expected processes are present
        let process_names: Vec<&str> = all_items
            .iter()
            .map(|item| item.process_name.as_str())
            .collect();

        assert!(process_names.contains(&"WeChat.exe"));
        assert!(process_names.contains(&"svchost.exe"));
        assert!(process_names.contains(&"explorer.exe"));

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_knowledge_manager_creates_default_knowledge_file() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        // Create new manager
        let _manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        // Verify file was created
        let knowledge_file = test_dir.join("process_knowledge.json");
        assert!(knowledge_file.exists(), "Should create knowledge file");

        // Verify file content
        let content = fs::read_to_string(&knowledge_file).unwrap();
        let knowledge: ProcessKnowledge = serde_json::from_str(&content).unwrap();
        assert_eq!(knowledge.processes.len(), 3);

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_knowledge_manager_loads_existing_file() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        fs::create_dir_all(&test_dir).unwrap();

        // Create a custom knowledge file
        let custom_knowledge = ProcessKnowledge {
            processes: vec![
                ProcessKnowledgeItem {
                    process_name: "CustomProcess.exe".to_string(),
                    description: "Custom process for testing".to_string(),
                    function: "Testing".to_string(),
                    startup_method: "Manual".to_string(),
                    performance_impact: "None".to_string(),
                    can_close: true,
                    recommendation: "Can be closed".to_string(),
                    risk_level: "safe".to_string(),
                    tags: vec!["test".to_string()],
                },
            ],
        };

        let knowledge_file = test_dir.join("process_knowledge.json");
        fs::write(
            &knowledge_file,
            serde_json::to_string_pretty(&custom_knowledge).unwrap(),
        )
        .unwrap();

        // Load manager - should load the custom file
        let manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        let all_items = manager.get_all();
        assert_eq!(all_items.len(), 1);
        assert_eq!(all_items[0].process_name, "CustomProcess.exe");

        // Default items should not be present
        assert!(manager.lookup("WeChat.exe").is_none());

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_process_knowledge_item_creation() {
        let item = ProcessKnowledgeItem {
            process_name: "TestProcess.exe".to_string(),
            description: "Test process".to_string(),
            function: "Testing things".to_string(),
            startup_method: "Manual start".to_string(),
            performance_impact: "Low impact".to_string(),
            can_close: true,
            recommendation: "Safe to close".to_string(),
            risk_level: "safe".to_string(),
            tags: vec!["test".to_string(), "utility".to_string()],
        };

        assert_eq!(item.process_name, "TestProcess.exe");
        assert_eq!(item.description, "Test process");
        assert_eq!(item.function, "Testing things");
        assert_eq!(item.startup_method, "Manual start");
        assert_eq!(item.performance_impact, "Low impact");
        assert!(item.can_close);
        assert_eq!(item.recommendation, "Safe to close");
        assert_eq!(item.risk_level, "safe");
        assert_eq!(item.tags, vec!["test", "utility"]);
    }

    #[test]
    fn test_process_knowledge_serialization() {
        let item = ProcessKnowledgeItem {
            process_name: "Serializable.exe".to_string(),
            description: "Test".to_string(),
            function: "Test".to_string(),
            startup_method: "Test".to_string(),
            performance_impact: "Test".to_string(),
            can_close: false,
            recommendation: "Don't close".to_string(),
            risk_level: "dangerous".to_string(),
            tags: vec!["test".to_string()],
        };

        let json = serde_json::to_string(&item).expect("Should serialize");
        let deserialized: ProcessKnowledgeItem =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(item.process_name, deserialized.process_name);
        assert_eq!(item.can_close, deserialized.can_close);
        assert_eq!(item.risk_level, deserialized.risk_level);
        assert_eq!(item.tags, deserialized.tags);
    }

    #[test]
    fn test_reload_reloads_from_file() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        // Create manager with default knowledge
        let mut manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();
        assert_eq!(manager.get_all().len(), 3);

        // Modify the file directly
        let new_knowledge = ProcessKnowledge {
            processes: vec![ProcessKnowledgeItem {
                process_name: "NewProcess.exe".to_string(),
                description: "New".to_string(),
                function: "New".to_string(),
                startup_method: "New".to_string(),
                performance_impact: "New".to_string(),
                can_close: true,
                recommendation: "New".to_string(),
                risk_level: "safe".to_string(),
                tags: vec![],
            }],
        };

        let knowledge_file = test_dir.join("process_knowledge.json");
        fs::write(
            &knowledge_file,
            serde_json::to_string_pretty(&new_knowledge).unwrap(),
        )
        .unwrap();

        // Reload and verify
        manager.reload().unwrap();
        assert_eq!(manager.get_all().len(), 1);
        assert!(manager.lookup("NewProcess.exe").is_some());

        cleanup_test_dir(&test_dir);
    }

    #[test]
    fn test_default_knowledge() {
        let test_dir = get_test_dir();
        cleanup_test_dir(&test_dir);

        let manager = KnowledgeManager::new_with_path(test_dir.clone()).unwrap();

        // Verify WeChat entry specific details
        let wechat = manager.lookup("WeChat.exe").unwrap();
        assert_eq!(wechat.process_name, "WeChat.exe");
        assert_eq!(wechat.function, "即时通讯和社交");
        assert!(wechat.can_close);
        assert_eq!(wechat.risk_level, "safe");
        assert_eq!(wechat.tags, vec!["社交", "通讯"]);

        // Verify svchost specific details
        let svchost = manager.lookup("svchost.exe").unwrap();
        assert_eq!(svchost.process_name, "svchost.exe");
        assert_eq!(svchost.function, "承载多个Windows系统服务");
        assert!(!svchost.can_close);
        assert_eq!(svchost.risk_level, "warning");

        // Verify explorer specific details
        let explorer = manager.lookup("explorer.exe").unwrap();
        assert_eq!(explorer.process_name, "explorer.exe");
        assert!(!explorer.can_close);

        cleanup_test_dir(&test_dir);
    }
}
