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
