use crate::types::MarketRegime;

/// KIS API로부터 계산된 시장 지표 스냅샷.
/// NASD/S&P500 지수 기반으로 1시간마다 갱신.
pub struct RegimeInput {
    /// 나스닥 지수 5일 이동평균
    pub ma5: f64,
    /// 나스닥 지수 20일 이동평균
    pub ma20: f64,
    /// 당일 지수 등락률 (%)
    pub daily_change_pct: f64,
    /// 시장 거래량 / 20일 평균 거래량 비율
    pub volume_ratio: f64,
}

/// 스펙 Section 1 판정 로직.
/// Volatile 판정이 Quiet보다 우선 (등락률이 크면 거래량 무관).
pub fn classify_regime(input: &RegimeInput) -> MarketRegime {
    let abs_change = input.daily_change_pct.abs();

    // 1순위: 등락률 ±1.5% 이상 → Volatile
    if abs_change >= 1.5 {
        return MarketRegime::Volatile;
    }

    // 2순위: 등락률 ±0.3% 미만 + 거래량 저조 → Quiet
    if abs_change < 0.3 && input.volume_ratio < 0.8 {
        return MarketRegime::Quiet;
    }

    // 기본: Trending
    MarketRegime::Trending
}

/// 레짐 상태를 태스크 간 공유하기 위한 watch channel 타입 별칭.
pub type RegimeSender = tokio::sync::watch::Sender<MarketRegime>;
pub type RegimeReceiver = tokio::sync::watch::Receiver<MarketRegime>;

pub fn regime_channel(initial: MarketRegime) -> (RegimeSender, RegimeReceiver) {
    tokio::sync::watch::channel(initial)
}
