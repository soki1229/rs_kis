use kis_bot::discovery::{Watchlist, AddReason};

#[test]
fn add_and_contains() {
    let mut wl = Watchlist::new(30);
    wl.add("NVDA".to_string(), AddReason::VolumeSurge);
    assert!(wl.contains("NVDA"));
}

#[test]
fn max_capacity_enforced() {
    let mut wl = Watchlist::new(2);
    wl.add("AAPL".to_string(), AddReason::PriceRanking);
    wl.add("NVDA".to_string(), AddReason::VolumeSurge);
    wl.add("TSLA".to_string(), AddReason::News);
    assert_eq!(wl.len(), 2);
}

#[test]
fn expired_entries_removed() {
    let mut wl = Watchlist::new(30);
    let old_time = chrono::Utc::now() - chrono::Duration::hours(49);
    wl.add_with_time("AAPL".to_string(), AddReason::VolumeSurge, old_time);
    wl.remove_expired();
    assert!(!wl.contains("AAPL"));
}

#[test]
fn active_entries_not_removed() {
    let mut wl = Watchlist::new(30);
    wl.add("NVDA".to_string(), AddReason::VolumeSurge);
    wl.remove_expired();
    assert!(wl.contains("NVDA"));
}

#[test]
fn duplicate_add_refreshes_ttl() {
    let mut wl = Watchlist::new(30);
    let old_time = chrono::Utc::now() - chrono::Duration::hours(49);
    wl.add_with_time("AAPL".to_string(), AddReason::VolumeSurge, old_time);
    wl.add("AAPL".to_string(), AddReason::News);
    wl.remove_expired();
    assert!(wl.contains("AAPL"), "re-added entry should survive expiry check");
}
