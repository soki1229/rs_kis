pub mod guard;

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

pub struct RiskSizerInput {
    pub account_balance: Decimal,
    pub risk_per_trade: Decimal,
    pub atr: Decimal,
    pub atr_stop_multiplier: Decimal,
    pub entry_price: Decimal,
    pub strength: f64,
    /// Trending=1.0, Volatile=0.5, Quiet=N/A
    pub regime_factor: Decimal,
    /// Default=1.0, Conservative=0.6, Aggressive=1.6
    pub profile_factor: Decimal,
    pub max_position_pct: Decimal,
}

/// 스펙 Section 5 최종 사이즈 계산식.
/// final_size = base_size × strength_factor × regime_factor × profile_factor
/// capped at account_balance × max_position_pct / entry_price
pub fn calculate_size(input: &RiskSizerInput) -> Decimal {
    if input.atr == dec!(0) || input.atr_stop_multiplier == dec!(0) {
        return dec!(0);
    }

    let stop_distance = input.atr * input.atr_stop_multiplier;
    let allowed_loss = input.account_balance * input.risk_per_trade;
    let base_size = allowed_loss / stop_distance;

    let sf = Decimal::from_f64_retain(strength_size_factor(input.strength))
        .unwrap_or(dec!(1.0));

    let final_size = base_size * sf * input.regime_factor * input.profile_factor;

    // 포지션 비중 상한
    let max_size = input.account_balance * input.max_position_pct / input.entry_price;

    final_size.min(max_size).max(dec!(0))
}

/// 스펙 Section 5 strength 구간별 사이즈 배율 테이블.
pub fn strength_size_factor(strength: f64) -> f64 {
    match strength {
        s if s >= 0.85 => 1.25,
        s if s >= 0.75 => 1.15,
        s if s >= 0.65 => 1.00,
        _ => 0.75,
    }
}
