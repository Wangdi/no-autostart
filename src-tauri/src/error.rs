use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Main error type for the NoAutoStart application
#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AppError {
    #[error("Process not found: {0}")]
    ProcessNotFound(u32),

    #[error("Cannot close critical system process: {0}")]
    CriticalProcess(String),

    #[error("Failed to open process: {0}")]
    ProcessOpenFailed(String),

    #[error("Failed to terminate process: {0}")]
    ProcessTerminateFailed(String),

    #[error("Failed to access registry: {0}")]
    RegistryAccessFailed(String),

    #[error("Failed to read registry value: {0}")]
    RegistryReadFailed(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Failed to read configuration file: {0}")]
    ConfigReadFailed(String),

    #[error("Failed to write configuration file: {0}")]
    ConfigWriteFailed(String),

    #[error("History error: {0}")]
    HistoryError(String),

    #[error("Failed to read history file: {0}")]
    HistoryReadFailed(String),

    #[error("Failed to write history file: {0}")]
    HistoryWriteFailed(String),

    #[error("Knowledge base error: {0}")]
    KnowledgeError(String),

    #[error("Failed to access app data directory: {0}")]
    DataDirAccessFailed(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        if err.contains("not found") {
            AppError::ProcessNotFound(0)
        } else if err.contains("Cannot close critical") {
            AppError::CriticalProcess(err)
        } else {
            AppError::Unknown(err)
        }
    }
}

/// Result type alias for AppError
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_error_display() {
        let err = AppError::ProcessNotFound(1234);
        assert_eq!(err.to_string(), "Process not found: 1234");

        let err = AppError::CriticalProcess("svchost.exe".to_string());
        assert_eq!(err.to_string(), "Cannot close critical system process: svchost.exe");
    }

    #[test]
    fn test_app_error_from_io_error() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::IoError(_)));
    }

    #[test]
    fn test_app_error_from_json_error() {
        let json_err = serde_json::from_str::<String>("invalid json");
        if let Err(e) = json_err {
            let app_err: AppError = e.into();
            assert!(matches!(app_err, AppError::SerializationError(_)));
        }
    }

    #[test]
    fn test_app_error_from_string() {
        let err = "Process 999 not found".to_string();
        let app_err: AppError = err.into();
        assert!(matches!(app_err, AppError::ProcessNotFound(_)));

        let err = "Cannot close critical system process".to_string();
        let app_err: AppError = err.into();
        assert!(matches!(app_err, AppError::CriticalProcess(_)));

        let err = "Some unknown error".to_string();
        let app_err: AppError = err.into();
        assert!(matches!(app_err, AppError::Unknown(_)));
    }

    #[test]
    fn test_app_result_ok() {
        let result: AppResult<i32> = Ok(42);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_app_result_err() {
        let result: AppResult<i32> = Err(AppError::ProcessNotFound(999));
        assert!(result.is_err());
    }

    #[test]
    fn test_app_error_serialization() {
        let err = AppError::ConfigError("test error".to_string());
        let json = serde_json::to_string(&err).expect("Should serialize");
        let deserialized: AppError =
            serde_json::from_str(&json).expect("Should deserialize");
        assert!(matches!(deserialized, AppError::ConfigError(_)));
    }

    #[test]
    fn test_all_error_variants() {
        let errors = vec![
            AppError::ProcessNotFound(1),
            AppError::CriticalProcess("test".to_string()),
            AppError::ProcessOpenFailed("test".to_string()),
            AppError::ProcessTerminateFailed("test".to_string()),
            AppError::RegistryAccessFailed("test".to_string()),
            AppError::RegistryReadFailed("test".to_string()),
            AppError::ConfigError("test".to_string()),
            AppError::ConfigReadFailed("test".to_string()),
            AppError::ConfigWriteFailed("test".to_string()),
            AppError::HistoryError("test".to_string()),
            AppError::HistoryReadFailed("test".to_string()),
            AppError::HistoryWriteFailed("test".to_string()),
            AppError::KnowledgeError("test".to_string()),
            AppError::DataDirAccessFailed("test".to_string()),
            AppError::IoError("test".to_string()),
            AppError::SerializationError("test".to_string()),
            AppError::PermissionDenied("test".to_string()),
            AppError::InvalidOperation("test".to_string()),
            AppError::Unknown("test".to_string()),
        ];

        for err in errors {
            let msg = err.to_string();
            assert!(!msg.is_empty());
        }
    }
}
