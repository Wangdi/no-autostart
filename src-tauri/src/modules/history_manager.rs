use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::Manager;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSnapshot {
    pub pid: u32,
    pub name: String,
    pub executable_path: String,
    pub startup_type: String,
    pub startup_location: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermanentActionBackup {
    #[serde(rename = "type")]
    pub action_type: String,
    pub backup_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRecord {
    pub id: String,
    pub timestamp: String,
    pub operation_type: String,
    pub process_snapshot: ProcessSnapshot,
    pub permanent_action: Option<PermanentActionBackup>,
    pub status: String,
    pub reverted_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationHistory {
    pub records: Vec<HistoryRecord>,
}

pub struct HistoryManager {
    history_path: PathBuf,
    history: OperationHistory,
}

impl HistoryManager {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let data_dir = app
            .path()
            .app_data_dir()
            .map_err(|e| format!("Failed to get app data dir: {}", e))?;

        let history_dir = data_dir.join("history");
        fs::create_dir_all(&history_dir)
            .map_err(|e| format!("Failed to create history dir: {}", e))?;

        let history_path = history_dir.join("history.json");

        let history = if history_path.exists() {
            let content = fs::read_to_string(&history_path)
                .map_err(|e| format!("Failed to read history: {}", e))?;
            serde_json::from_str(&content).unwrap_or_else(|_| OperationHistory::default())
        } else {
            OperationHistory::default()
        };

        Ok(Self {
            history_path,
            history,
        })
    }

    pub fn get_history(&self) -> &OperationHistory {
        &self.history
    }

    pub fn add_record(&mut self, mut record: HistoryRecord) -> Result<(), String> {
        if record.id.is_empty() {
            record.id = Uuid::new_v4().to_string();
        }
        if record.timestamp.is_empty() {
            record.timestamp = Utc::now().to_rfc3339();
        }
        self.history.records.insert(0, record);
        self.save()
    }

    pub fn revert_operation(&mut self, id: &str) -> Result<HistoryRecord, String> {
        let record = self
            .history
            .records
            .iter_mut()
            .find(|r| r.id == id)
            .ok_or("Record not found")?;

        if record.status == "reverted" {
            return Err("Operation already reverted".to_string());
        }

        record.status = String::from("reverted");
        record.reverted_at = Some(Utc::now().to_rfc3339());

        self.save()?;

        Ok(record.clone())
    }

    fn save(&self) -> Result<(), String> {
        let content = serde_json::to_string_pretty(&self.history)
            .map_err(|e| format!("Failed to serialize history: {}", e))?;

        fs::write(&self.history_path, content)
            .map_err(|e| format!("Failed to write history: {}", e))?;

        Ok(())
    }
}
