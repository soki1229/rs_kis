use crate::error::BotError;
use crate::types::{KillSwitchFile, KillSwitchMode};
use chrono::{TimeZone, Utc};

pub struct KillSwitch {
    path: String,
}

impl KillSwitch {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    /// 파일 존재 시 파싱. None = Kill Switch 비활성.
    pub fn current_mode(&self) -> Option<KillSwitchMode> {
        let content = std::fs::read_to_string(&self.path).ok()?;
        let file: KillSwitchFile = serde_json::from_str(&content).ok()?;
        Some(file.mode)
    }

    /// Kill Switch 발동. 파일에 JSON으로 기록.
    pub fn activate(
        &self,
        mode: KillSwitchMode,
        reason: &str,
        details: &str,
    ) -> Result<(), BotError> {
        let kst = chrono::FixedOffset::east_opt(9 * 3600)
            .unwrap()
            .from_utc_datetime(&Utc::now().naive_utc());
        let file = KillSwitchFile {
            mode,
            reason: reason.to_string(),
            triggered_at: kst,
            details: details.to_string(),
        };
        let json = serde_json::to_string_pretty(&file).map_err(BotError::Json)?;
        std::fs::write(&self.path, json).map_err(BotError::Io)
    }

    /// Kill Switch 해제. 파일 삭제.
    pub fn clear(&self) -> Result<(), BotError> {
        if std::path::Path::new(&self.path).exists() {
            std::fs::remove_file(&self.path).map_err(BotError::Io)?;
        }
        Ok(())
    }
}
