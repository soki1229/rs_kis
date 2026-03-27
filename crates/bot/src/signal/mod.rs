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

impl SignalDecision {
    /// LLM 없는 경로 (Setup Score 60~79).
    pub fn evaluate_without_llm(
        setup_score: i32,
        rule_signal: Option<RuleSignal>,
        rule_threshold: f64,
    ) -> Self {
        if setup_score < 60 {
            return Self::Rejected { reason: format!("setup_score {} < 60", setup_score) };
        }
        match rule_signal {
            None => Self::Rejected { reason: "no rule signal".to_string() },
            Some(s) if s.strength < rule_threshold => Self::Rejected {
                reason: format!("strength {:.2} < threshold {:.2}", s.strength, rule_threshold),
            },
            Some(s) => Self::Enter {
                direction: s.direction,
                strength: s.strength,
                setup_score,
            },
        }
    }

    /// LLM 포함 경로 (Setup Score ≥ 80).
    pub fn evaluate_with_llm(
        setup_score: i32,
        rule_signal: Option<RuleSignal>,
        rule_threshold: f64,
        llm_verdict: LlmVerdict,
    ) -> Self {
        // LLM BLOCK은 rule signal 무관하게 항상 우선
        if llm_verdict == LlmVerdict::Block {
            return Self::Rejected { reason: "LLM verdict: BLOCK".to_string() };
        }

        match rule_signal {
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
        }
    }
}
