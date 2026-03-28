use crate::error::BotError;
use rust_decimal::Decimal;

pub struct SessionStats {
    pub open_positions: u32,
    pub daily_pnl: Decimal,
    pub consecutive_losses: u32,
    pub account_balance: Decimal,
    pub pending_order_value: Decimal,
}

pub struct RiskGuard {
    max_open_positions: u32,
    daily_loss_limit: Decimal,
    consecutive_loss_limit: u32,
    pending_order_limit_pct: Decimal,
}

impl RiskGuard {
    pub fn new(
        max_open_positions: u32,
        daily_loss_limit: Decimal,
        consecutive_loss_limit: u32,
        pending_order_limit_pct: Decimal,
    ) -> Self {
        Self {
            max_open_positions,
            daily_loss_limit,
            consecutive_loss_limit,
            pending_order_limit_pct,
        }
    }

    /// 신규 진입 가능 여부. Err = 차단.
    pub fn check_entry(&self, stats: &SessionStats) -> Result<(), BotError> {
        if stats.open_positions >= self.max_open_positions {
            return Err(BotError::RiskBlocked {
                reason: format!("max_open_positions {} reached", self.max_open_positions),
            });
        }

        let loss_limit_abs = stats.account_balance * self.daily_loss_limit;
        if stats.daily_pnl < -loss_limit_abs {
            return Err(BotError::RiskBlocked {
                reason: format!(
                    "daily_loss_limit exceeded: pnl={}, limit=-{}",
                    stats.daily_pnl, loss_limit_abs
                ),
            });
        }

        if stats.consecutive_losses >= self.consecutive_loss_limit {
            return Err(BotError::RiskBlocked {
                reason: format!(
                    "consecutive_loss_limit {} reached",
                    self.consecutive_loss_limit
                ),
            });
        }

        Ok(())
    }

    /// 미체결 비중이 한도 초과 → HARD Kill Switch 조건.
    pub fn check_kill_switch_hard(&self, stats: &SessionStats) -> bool {
        if stats.account_balance.is_zero() {
            return false;
        }
        let pct = stats.pending_order_value / stats.account_balance;
        pct > self.pending_order_limit_pct
    }
}
