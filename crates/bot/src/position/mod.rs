use rust_decimal::Decimal;
use crate::types::MarketRegime;

pub struct PositionState {
    pub entry_price: Decimal,
    pub stop_price: Decimal,
    pub atr_at_entry: Decimal,
    pub profit_target_1: Decimal,
    pub profit_target_2: Decimal,
    pub trailing_stop_price: Option<Decimal>,
    pub partial_exit_done: bool,
    pub regime: MarketRegime,
    pub profit_target_1_atr: Decimal,
    pub profit_target_2_atr: Decimal,
    pub trailing_atr_trending: Decimal,
    pub trailing_atr_volatile: Decimal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExitDecision {
    /// 손절가 도달 → 시장가 전량 청산
    StopLoss,
    /// 1차 목표가 도달 → 50% 부분 익절. pct = 0.5
    PartialExit { pct: Decimal },
    /// 2차 목표가 도달 (1차 익절 후) → 잔여 전량 청산
    FullExit,
    /// Trailing stop 도달 → 잔여 전량 청산
    TrailingStop,
    /// 아무 조건도 해당 없음 → 보유 유지
    Hold,
}

/// 스펙 Section 7 기반 익절/손절 판정. 순수 함수.
/// 우선순위: StopLoss > FullExit > TrailingStop > PartialExit > Hold
pub fn evaluate_exit(pos: &PositionState, current_price: Decimal) -> ExitDecision {
    // 손절가 도달
    if current_price <= pos.stop_price {
        return ExitDecision::StopLoss;
    }

    if pos.partial_exit_done {
        // 2차 목표가 도달
        if current_price >= pos.profit_target_2 {
            return ExitDecision::FullExit;
        }
        // Trailing stop 도달 (1차 익절 후에만 적용)
        if let Some(ts) = pos.trailing_stop_price {
            if current_price <= ts {
                return ExitDecision::TrailingStop;
            }
        }
    } else {
        // 1차 목표가 도달
        if current_price >= pos.profit_target_1 {
            return ExitDecision::PartialExit { pct: Decimal::new(5, 1) }; // 0.5
        }
    }

    ExitDecision::Hold
}

/// 레짐별 trailing stop 가격 계산. 스펙 Section 7 기반.
pub fn calculate_trailing_stop(
    high_price: Decimal,
    atr: Decimal,
    regime: &MarketRegime,
    trending_multiplier: Decimal,
    volatile_multiplier: Decimal,
) -> Decimal {
    let multiplier = match regime {
        MarketRegime::Trending => trending_multiplier,
        MarketRegime::Volatile => volatile_multiplier,
        MarketRegime::Quiet => return high_price,
    };
    high_price - atr * multiplier
}
