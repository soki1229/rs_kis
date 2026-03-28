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

/// 포트폴리오 수준 컨텍스트 — 개별 주문 사이징 시 포트폴리오 전체 상태를 반영.
pub struct PortfolioContext {
    /// 현재 열려 있는 포지션 수
    pub open_position_count: u32,
    /// 허용 최대 동시 포지션 수
    pub max_open_positions: u32,
    /// 현재 계좌 낙폭 비율 (0.0 = 없음, 0.10 = -10%)
    pub current_drawdown_pct: Decimal,
}

impl PortfolioContext {
    /// 포트폴리오 상태에 따른 사이즈 조정 배율 (0.0 ~ 1.0).
    /// - 포지션 한도 도달 → 0 (진입 불가)
    /// - 낙폭 ≥ 10%     → 0.5
    /// - 낙폭 ≥ 5%      → 0.75
    /// - 정상             → 1.0
    pub fn size_scale_factor(&self) -> Decimal {
        if self.open_position_count >= self.max_open_positions {
            return dec!(0);
        }
        if self.current_drawdown_pct >= dec!(0.10) {
            return dec!(0.5);
        }
        if self.current_drawdown_pct >= dec!(0.05) {
            return dec!(0.75);
        }
        dec!(1.0)
    }
}

/// 스펙 Section 5 최종 사이즈 계산식.
/// final_size = base_size × strength_factor × regime_factor × profile_factor × portfolio_scale
/// capped at account_balance × max_position_pct / entry_price
pub fn calculate_size(input: &RiskSizerInput, ctx: &PortfolioContext) -> Decimal {
    if input.atr == dec!(0) || input.atr_stop_multiplier == dec!(0) {
        return dec!(0);
    }

    let scale = ctx.size_scale_factor();
    if scale == dec!(0) {
        return dec!(0);
    }

    let stop_distance = input.atr * input.atr_stop_multiplier;
    let allowed_loss = input.account_balance * input.risk_per_trade;
    let base_size = allowed_loss / stop_distance;

    let sf = Decimal::from_f64_retain(strength_size_factor(input.strength)).unwrap_or(dec!(1.0));

    let final_size = base_size * sf * input.regime_factor * input.profile_factor * scale;

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
