use crate::types::OrderState;

#[derive(Debug, thiserror::Error)]
#[error("Invalid transition from {from:?} to {to:?}")]
pub struct TransitionError {
    pub from: String,
    pub to: String,
}

pub struct OrderStateMachine {
    state: OrderState,
}

impl OrderStateMachine {
    pub fn new() -> Self {
        Self { state: OrderState::PendingSubmit }
    }

    pub fn state(&self) -> &OrderState {
        &self.state
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.state,
            OrderState::FullyFilled
            | OrderState::CancelledPartial { .. }
            | OrderState::Cancelled
            | OrderState::Failed { .. }
        )
    }

    /// 유효하지 않은 전이는 Err 반환. 성공 시 내부 상태 갱신.
    pub fn transition(&mut self, next: OrderState) -> Result<(), TransitionError> {
        if self.is_terminal() {
            return Err(TransitionError {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            });
        }

        let valid = matches!(
            (&self.state, &next),
            (OrderState::PendingSubmit, OrderState::Submitted)
            | (OrderState::PendingSubmit, OrderState::Failed { .. })
            | (OrderState::Submitted, OrderState::PartiallyFilled { .. })
            | (OrderState::Submitted, OrderState::FullyFilled)
            | (OrderState::Submitted, OrderState::Cancelled)
            | (OrderState::Submitted, OrderState::Failed { .. })
            | (OrderState::PartiallyFilled { .. }, OrderState::FullyFilled)
            | (OrderState::PartiallyFilled { .. }, OrderState::CancelledPartial { .. })
        );

        if !valid {
            return Err(TransitionError {
                from: format!("{:?}", self.state),
                to: format!("{:?}", next),
            });
        }

        self.state = next;
        Ok(())
    }
}

impl Default for OrderStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
