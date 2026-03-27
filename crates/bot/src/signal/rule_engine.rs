use crate::types::{Direction, RuleSignal};

pub struct RuleEngineInput {
    /// 최근 N분봉 방향 (true=상승, false=하락)
    pub recent_candle_directions: Vec<bool>,
    /// (현재가 - 전일종가) / ATR_14 비율. 양수=상승 압력.
    pub price_vs_prev_close_atr_ratio: f64,
    /// 최근 3구간 거래량 (가속도 측정용)
    pub volume_acceleration: Vec<f64>,
}

pub struct RuleEngine;

impl RuleEngine {
    /// 스펙 Section 4-1 기반 RuleSignal 계산.
    /// strength = 세 요소 가중 평균 (각 0.0~1.0 정규화).
    /// strength < 0.40 이면 None 반환 (신호 없음).
    pub fn evaluate(&self, input: &RuleEngineInput) -> Option<RuleSignal> {
        let direction_score = self.direction_consistency(&input.recent_candle_directions);
        let price_score = self.price_position_score(input.price_vs_prev_close_atr_ratio);
        let volume_score = self.volume_acceleration_score(&input.volume_acceleration);

        // 가중 평균: 방향 40%, 가격 위치 30%, 거래량 가속 30%
        let raw_strength = direction_score * 0.40 + price_score * 0.30 + volume_score * 0.30;

        if raw_strength < 0.40 {
            return None;
        }

        // 방향 결정: 분봉 방향 다수결
        let up_count = input.recent_candle_directions.iter().filter(|&&d| d).count();
        let direction = if up_count * 2 >= input.recent_candle_directions.len() {
            Direction::Long
        } else {
            Direction::Short
        };

        Some(RuleSignal {
            direction,
            strength: raw_strength.clamp(0.0, 1.0),
        })
    }

    /// 연속성 점수 (0.0~1.0). 5분봉 중 같은 방향이 몇 개인지.
    fn direction_consistency(&self, directions: &[bool]) -> f64 {
        if directions.is_empty() {
            return 0.5;
        }
        let up = directions.iter().filter(|&&d| d).count();
        let majority = up.max(directions.len() - up);
        majority as f64 / directions.len() as f64
    }

    /// 가격 위치 점수 (0.0~1.0). ATR 비율 ±0.5 범위를 0~1로 매핑.
    fn price_position_score(&self, atr_ratio: f64) -> f64 {
        ((atr_ratio + 0.5) / 1.0).clamp(0.0, 1.0)
    }

    /// 거래량 가속도 점수 (0.0~1.0). 증가 추세이면 높음.
    fn volume_acceleration_score(&self, volumes: &[f64]) -> f64 {
        if volumes.len() < 2 {
            return 0.5;
        }
        let increasing = volumes.windows(2).filter(|w| w[1] > w[0]).count();
        increasing as f64 / (volumes.len() - 1) as f64
    }
}
