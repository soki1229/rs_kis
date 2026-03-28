/// HARD BLOCK 발동 사유. 해당되면 Setup Score 계산 없이 즉시 종료.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardBlockReason {
    EarningsEvent,
    FomcOrCpi,
    OpenBlackout,
    CloseBlackout,
    DailyGainExceeded, // > +10%
    DailyLossExceeded, // < -3%
}

pub struct HardBlockInput {
    pub daily_change_pct: f64,
    pub mins_since_open: i64,
    pub mins_until_close: i64,
    pub has_earnings_event: bool,
    pub has_fomc_today: bool,
    pub entry_blackout_open_mins: i64,
    pub entry_blackout_close_mins: i64,
}

pub struct HardBlockChecker;

impl HardBlockChecker {
    /// None = 통과. Some(reason) = 차단.
    pub fn check(&self, input: &HardBlockInput) -> Option<HardBlockReason> {
        if input.has_earnings_event {
            return Some(HardBlockReason::EarningsEvent);
        }
        if input.has_fomc_today {
            return Some(HardBlockReason::FomcOrCpi);
        }
        if input.mins_since_open < input.entry_blackout_open_mins {
            return Some(HardBlockReason::OpenBlackout);
        }
        if input.mins_until_close < input.entry_blackout_close_mins {
            return Some(HardBlockReason::CloseBlackout);
        }
        if input.daily_change_pct > 10.0 {
            return Some(HardBlockReason::DailyGainExceeded);
        }
        if input.daily_change_pct < -3.0 {
            return Some(HardBlockReason::DailyLossExceeded);
        }
        None
    }
}
