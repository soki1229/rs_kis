use kis_bot::control::kill_switch::KillSwitch;
use kis_bot::types::KillSwitchMode;
use tempfile::tempdir;

#[test]
fn no_file_means_no_kill_switch() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path);
    assert!(ks.current_mode().is_none());
}

#[test]
fn soft_kill_switch_persists_to_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path.clone());

    ks.activate(KillSwitchMode::Soft, "KIS API 3 errors", "HTTP 503 x3")
        .unwrap();

    let ks2 = KillSwitch::new(path);
    assert_eq!(ks2.current_mode(), Some(KillSwitchMode::Soft));
}

#[test]
fn hard_kill_switch_persists() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path.clone());

    ks.activate(KillSwitchMode::Hard, "balance mismatch", "")
        .unwrap();
    assert_eq!(ks.current_mode(), Some(KillSwitchMode::Hard));
}

#[test]
fn clear_removes_file() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path);

    ks.activate(KillSwitchMode::Soft, "test", "").unwrap();
    assert!(ks.current_mode().is_some());

    ks.clear().unwrap();
    assert!(ks.current_mode().is_none());
}

#[test]
fn file_content_includes_reason() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("ks").to_str().unwrap().to_string();
    let ks = KillSwitch::new(path.clone());

    ks.activate(KillSwitchMode::Soft, "KIS API error", "detail info")
        .unwrap();

    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("KIS API error"));
    assert!(content.contains("detail info"));
}
