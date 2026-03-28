use crate::types::MarketRegime;

pub struct SetupScoreInput {
    // ── 가산 조건 ──────────────────────────────────────────
    /// 5일 MA > 20일 MA
    pub ma5_above_ma20: bool,
    /// 거래량 / 20일 평균 거래량 비율 (≥ 2.0x → +20)
    pub volume_ratio: f64,
    /// 최근 5분 거래량 / 이전 5분 거래량 (≥ 1.5x → +15)
    pub recent_5min_volume_ratio: f64,
    /// 매수잔량 / 매도잔량 (≥ 1.3 → +20)
    pub bid_ask_imbalance: f64,
    /// 최근 10분 내 신고가 돌파 여부
    pub new_high_last_10min: bool,
    /// 24h 이내 뉴스 촉매 존재
    pub has_news_catalyst: bool,

    // ── 감점 조건 ──────────────────────────────────────────
    /// 당일 등락률 (> +7% → -15)
    pub daily_change_pct: f64,
    /// 장 마감까지 남은 분 (< blackout*2 → -10)
    pub mins_until_close: i64,
    pub entry_blackout_close_mins: i64,
    /// Volatile 레짐 → -10
    pub regime: MarketRegime,
}

/// 스펙 Section 3-2 기반 Setup Score 계산.
/// 반환값: i32 (음수 가능)
pub fn calculate_setup_score(input: &SetupScoreInput) -> i32 {
    let mut score: i32 = 0;

    // 가산
    if input.ma5_above_ma20 {
        score += 20;
    }
    if input.volume_ratio >= 2.0 {
        score += 20;
    }
    if input.recent_5min_volume_ratio >= 1.5 {
        score += 15;
    }
    if input.bid_ask_imbalance >= 1.3 {
        score += 20;
    }
    if input.new_high_last_10min {
        score += 15;
    }
    if input.has_news_catalyst {
        score += 10;
    }

    // 감점
    if input.daily_change_pct > 7.0 {
        score -= 15;
    }
    if input.mins_until_close < input.entry_blackout_close_mins * 2 {
        score -= 10;
    }
    if input.regime == MarketRegime::Volatile {
        score -= 10;
    }

    score
}
