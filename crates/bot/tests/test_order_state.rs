use kis_bot::execution::order_state::OrderStateMachine;
use kis_bot::types::OrderState;
use rust_decimal_macros::dec;

#[test]
fn pending_to_submitted() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    assert_eq!(sm.state(), &OrderState::Submitted);
}

#[test]
fn submitted_to_fully_filled() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::FullyFilled).unwrap();
    assert_eq!(sm.state(), &OrderState::FullyFilled);
}

#[test]
fn submitted_to_partially_filled_to_fully_filled() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::PartiallyFilled {
        filled_qty: dec!(1),
    })
    .unwrap();
    sm.transition(OrderState::FullyFilled).unwrap();
    assert_eq!(sm.state(), &OrderState::FullyFilled);
}

#[test]
fn partially_filled_to_cancelled_partial() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::PartiallyFilled {
        filled_qty: dec!(1),
    })
    .unwrap();
    sm.transition(OrderState::CancelledPartial {
        filled_qty: dec!(1),
    })
    .unwrap();
    assert!(matches!(sm.state(), OrderState::CancelledPartial { .. }));
}

#[test]
fn invalid_transition_returns_error() {
    let mut sm = OrderStateMachine::new();
    let result = sm.transition(OrderState::FullyFilled);
    assert!(result.is_err());
}

#[test]
fn terminal_state_cannot_transition() {
    let mut sm = OrderStateMachine::new();
    sm.transition(OrderState::Submitted).unwrap();
    sm.transition(OrderState::FullyFilled).unwrap();
    let result = sm.transition(OrderState::Cancelled);
    assert!(result.is_err());
}
