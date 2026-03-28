use chrono::{DateTime, Utc};
use std::collections::HashMap;

const TTL_HOURS: i64 = 48; // 2 거래일

#[derive(Debug, Clone)]
pub enum AddReason {
    VolumeSurge,
    PriceRanking,
    News,
}

#[derive(Debug, Clone)]
pub struct WatchlistEntry {
    pub symbol: String,
    pub reason: AddReason,
    pub added_at: DateTime<Utc>,
    /// 실적 이벤트 추적 종목은 Some(이벤트 종료 시각) → TTL 연장
    pub event_end: Option<DateTime<Utc>>,
}

impl WatchlistEntry {
    pub fn is_expired(&self, now: DateTime<Utc>) -> bool {
        if let Some(end) = self.event_end {
            return now > end;
        }
        now - self.added_at > chrono::Duration::hours(TTL_HOURS)
    }
}

pub struct Watchlist {
    entries: HashMap<String, WatchlistEntry>,
    max_size: usize,
}

impl Watchlist {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: HashMap::new(),
            max_size,
        }
    }

    pub fn add(&mut self, symbol: String, reason: AddReason) {
        self.add_with_time(symbol, reason, Utc::now());
    }

    pub fn add_with_time(&mut self, symbol: String, reason: AddReason, time: DateTime<Utc>) {
        // 이미 있으면 TTL 갱신
        if self.entries.contains_key(&symbol) {
            if let Some(e) = self.entries.get_mut(&symbol) {
                e.added_at = time;
            }
            return;
        }
        // 최대 크기 초과 시 거절
        if self.entries.len() >= self.max_size {
            return;
        }
        self.entries.insert(
            symbol.clone(),
            WatchlistEntry {
                symbol,
                reason,
                added_at: time,
                event_end: None,
            },
        );
    }

    pub fn extend_event_window(&mut self, symbol: &str, end: DateTime<Utc>) {
        if let Some(e) = self.entries.get_mut(symbol) {
            e.event_end = Some(end);
        }
    }

    pub fn remove_expired(&mut self) {
        let now = Utc::now();
        self.entries.retain(|_, e| !e.is_expired(now));
    }

    pub fn contains(&self, symbol: &str) -> bool {
        self.entries.contains_key(symbol)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn symbols(&self) -> Vec<&str> {
        self.entries.keys().map(String::as_str).collect()
    }
}
