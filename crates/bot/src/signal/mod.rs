pub mod llm_engine;
pub mod rule_engine;

use crate::types::{Direction, LlmVerdict, RuleSignal};

#[derive(Debug, Clone)]
pub enum SignalDecision {
    /// 진입 확정. direction과 strength를 Risk Sizing에 전달.
    Enter { direction: Direction, strength: f64, setup_score: i32 },
    /// 관망 (LLM WATCH 판정). 신호는 유효하지만 지금 진입 안 함.
    Watch { reason: String },
    /// 진입 거절.
    Rejected { reason: String },
}

/// 판정 논리 경로 — 어떤 분기를 통해 결론에 도달했는지 기록.
#[derive(Debug, Clone, PartialEq)]
pub enum DecisionPath {
    /// setup_score < 60 → 점수 미달로 즉시 거절
    LowScore,
    /// setup_score 60–79 → LLM 없이 룰엔진으로 판단
    RuleOnly,
    /// setup_score ≥ 80 → LLM 포함 판단
    RuleAndLlm,
}

/// 신호 평가 전체 이력 — 판정 근거 재현·감사·로깅 용도.
#[derive(Debug, Clone)]
pub struct SignalTrace {
    /// 어떤 평가 경로를 거쳤는지
    pub path: DecisionPath,
    /// 입력 setup score
    pub setup_score: i32,
    /// 룰 엔진 강도 (적용된 경우)
    pub rule_strength: Option<f64>,
    /// LLM 판정 (RuleAndLlm 경로에서만)
    pub llm_verdict: Option<LlmVerdict>,
    /// 최종 결론
    pub decision: SignalDecision,
}

impl SignalDecision {
    /// LLM 없는 경로 (Setup Score 60~79).
    pub fn evaluate_without_llm(
        setup_score: i32,
        rule_signal: Option<RuleSignal>,
        rule_threshold: f64,
    ) -> SignalTrace {
        if setup_score < 60 {
            return SignalTrace {
                path: DecisionPath::LowScore,
                setup_score,
                rule_strength: None,
                llm_verdict: None,
                decision: Self::Rejected {
                    reason: format!("setup_score {} < 60", setup_score),
                },
            };
        }
        let rule_strength = rule_signal.as_ref().map(|s| s.strength);
        let decision = match rule_signal {
            None => Self::Rejected { reason: "no rule signal".to_string() },
            Some(s) if s.strength < rule_threshold => Self::Rejected {
                reason: format!("strength {:.2} < threshold {:.2}", s.strength, rule_threshold),
            },
            Some(s) => Self::Enter {
                direction: s.direction,
                strength: s.strength,
                setup_score,
            },
        };
        SignalTrace {
            path: DecisionPath::RuleOnly,
            setup_score,
            rule_strength,
            llm_verdict: None,
            decision,
        }
    }

    /// LLM 포함 경로 (Setup Score ≥ 80).
    pub fn evaluate_with_llm(
        setup_score: i32,
        rule_signal: Option<RuleSignal>,
        rule_threshold: f64,
        llm_verdict: LlmVerdict,
    ) -> SignalTrace {
        // LLM BLOCK은 rule signal 무관하게 항상 우선
        if llm_verdict == LlmVerdict::Block {
            return SignalTrace {
                path: DecisionPath::RuleAndLlm,
                setup_score,
                rule_strength: rule_signal.as_ref().map(|s| s.strength),
                llm_verdict: Some(llm_verdict),
                decision: Self::Rejected { reason: "LLM verdict: BLOCK".to_string() },
            };
        }

        let rule_strength = rule_signal.as_ref().map(|s| s.strength);
        let decision = match rule_signal {
            None => Self::Rejected { reason: "no rule signal".to_string() },
            Some(s) if s.strength < rule_threshold => Self::Rejected {
                reason: format!("strength {:.2} < threshold {:.2}", s.strength, rule_threshold),
            },
            Some(s) => {
                if llm_verdict == LlmVerdict::Watch {
                    Self::Watch { reason: "LLM verdict: WATCH".to_string() }
                } else {
                    Self::Enter {
                        direction: s.direction,
                        strength: s.strength,
                        setup_score,
                    }
                }
            }
        };
        SignalTrace {
            path: DecisionPath::RuleAndLlm,
            setup_score,
            rule_strength,
            llm_verdict: Some(llm_verdict),
            decision,
        }
    }
}
