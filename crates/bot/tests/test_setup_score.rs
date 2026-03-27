use kis_bot::qualification::setup_score::{SetupScoreInput, calculate_setup_score};
use kis_bot::types::MarketRegime;

fn base_input() -> SetupScoreInput {
    SetupScoreInput {
        ma5_above_ma20: false,
        volume_ratio: 1.0,
        recent_5min_volume_ratio: 1.0,
        bid_ask_imbalance: 1.0,
        new_high_last_10min: false,
        has_news_catalyst: false,
        daily_change_pct: 2.0,
        mins_until_close: 300,
        entry_blackout_close_mins: 15,
        regime: MarketRegime::Trending,
    }
}

#[test]
fn zero_score_when_no_conditions_met() {
    let score = calculate_setup_score(&base_input());
    assert_eq!(score, 0);
}

#[test]
fn ma5_above_ma20_adds_20() {
    let mut input = base_input();
    input.ma5_above_ma20 = true;
    assert_eq!(calculate_setup_score(&input), 20);
}

#[test]
fn volume_ratio_2x_adds_20() {
    let mut input = base_input();
    input.volume_ratio = 2.0;
    assert_eq!(calculate_setup_score(&input), 20);
}

#[test]
fn all_positive_conditions_max_100() {
    let input = SetupScoreInput {
        ma5_above_ma20: true,
        volume_ratio: 2.5,
        recent_5min_volume_ratio: 1.6,
        bid_ask_imbalance: 1.4,
        new_high_last_10min: true,
        has_news_catalyst: true,
        daily_change_pct: 2.0,
        mins_until_close: 300,
        entry_blackout_close_mins: 15,
        regime: MarketRegime::Trending,
    };
    assert_eq!(calculate_setup_score(&input), 100);
}

#[test]
fn overheat_daily_gain_subtracts_15() {
    let input = SetupScoreInput {
        ma5_above_ma20: true,
        daily_change_pct: 7.5,
        ..base_input()
    };
    assert_eq!(calculate_setup_score(&input), 5);
}

#[test]
fn volatile_regime_subtracts_10() {
    let input = SetupScoreInput {
        ma5_above_ma20: true,
        regime: MarketRegime::Volatile,
        ..base_input()
    };
    assert_eq!(calculate_setup_score(&input), 10);
}

#[test]
fn score_can_go_negative() {
    let input = SetupScoreInput {
        daily_change_pct: 8.0,
        regime: MarketRegime::Volatile,
        mins_until_close: 20,
        entry_blackout_close_mins: 15,
        ..base_input()
    };
    assert_eq!(calculate_setup_score(&input), -35);
}
