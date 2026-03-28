use crate::error::BotError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BotConfig {
    pub profile: ProfileSection,
    pub risk: RiskSection,
    pub signal: SignalSection,
    pub llm: LlmSection,
    pub position: PositionSection,
    pub trading_hours: TradingHoursSection,
    pub monitoring: MonitoringSection,
    pub finnhub: FinnhubSection,
    pub state: StateSection,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProfileSection {
    pub active: ProfileName,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ProfileName {
    Default,
    Conservative,
    Aggressive,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RiskSection {
    pub risk_per_trade: f64,
    pub max_open_positions: u32,
    pub max_position_pct: f64,
    pub max_sector_exposure: f64,
    pub daily_loss_limit: f64,
    pub consecutive_loss_limit: u32,
    pub atr_stop_multiplier: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SignalSection {
    pub setup_score_threshold_entry: i32,
    pub setup_score_threshold_llm: i32,
    pub rule_strength_threshold: f64,
    pub fallback_rule_strength: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LlmSection {
    pub model: String,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PositionSection {
    pub profit_target_1_atr: f64,
    pub profit_target_2_atr: f64,
    pub trailing_atr_trending: f64,
    pub trailing_atr_volatile: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TradingHoursSection {
    pub entry_blackout_open_mins: i64,
    pub entry_blackout_close_mins: i64,
    pub eod_liquidate_mins: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitoringSection {
    pub regime_consecutive_loss_limit: u32,
    pub regime_suspend_days: u32,
    pub llm_underperformance_weeks: u32,
    pub cumulative_r_alert_threshold: f64,
    pub mdd_alert_pct: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinnhubSection {
    pub api_key: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StateSection {
    pub db_path: String,
    pub kill_switch_path: String,
}

impl BotConfig {
    pub fn from_file(path: &Path) -> Result<Self, BotError> {
        let content = std::fs::read_to_string(path).map_err(BotError::Io)?;
        toml::from_str(&content).map_err(|e| BotError::Config(e.to_string()))
    }

    /// 현재 프로파일에 따른 `risk_per_trade` 값 반환.
    /// Conservative: 0.003, Aggressive: 0.008, Default: config 값 그대로.
    pub fn effective_risk_per_trade(&self) -> f64 {
        match self.profile.active {
            ProfileName::Conservative => 0.003_f64.min(self.risk.risk_per_trade),
            ProfileName::Aggressive => 0.008_f64.max(self.risk.risk_per_trade),
            ProfileName::Default => self.risk.risk_per_trade,
        }
    }

    /// 현재 프로파일에 따른 Setup Score 진입 임계값.
    pub fn effective_score_threshold_entry(&self) -> i32 {
        match self.profile.active {
            ProfileName::Conservative => 70,
            ProfileName::Aggressive => 55,
            ProfileName::Default => self.signal.setup_score_threshold_entry,
        }
    }

    /// 현재 프로파일에 따른 LLM 호출 임계값.
    pub fn effective_score_threshold_llm(&self) -> i32 {
        match self.profile.active {
            ProfileName::Conservative => 85,
            ProfileName::Aggressive => 75,
            ProfileName::Default => self.signal.setup_score_threshold_llm,
        }
    }

    /// 현재 프로파일에 따른 consecutive_loss_limit.
    pub fn effective_consecutive_loss_limit(&self) -> u32 {
        match self.profile.active {
            ProfileName::Conservative => 2,
            ProfileName::Aggressive => 4,
            ProfileName::Default => self.risk.consecutive_loss_limit,
        }
    }

    /// 현재 프로파일에 따른 atr_stop_multiplier.
    pub fn effective_atr_stop_multiplier(&self) -> f64 {
        match self.profile.active {
            ProfileName::Conservative => 2.0,
            ProfileName::Aggressive => 1.2,
            ProfileName::Default => self.risk.atr_stop_multiplier,
        }
    }
}
