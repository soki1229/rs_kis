# Plan 2: kis_api Crate Redesign

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `crates/kis_api`를 스펙대로 완전히 재설계한다 — 전역 상태 제거, `KisConfig` builder, `KisError` 계층화, `KisApi`/`KisEventSource` 트레이트, `KisClient`(REST), `KisStream`(WebSocket) 구현.

**Architecture:** 모든 상태를 `Arc<Inner>` 패턴으로 관리해 `Clone` 가능하게 한다. REST와 WebSocket 라이프사이클을 분리한다. WebSocket 이벤트는 내부 `broadcast` 채널을 `EventReceiver`로 래핑해 노출한다. 재연결 및 구독 복구는 라이브러리 내부 책임. 기존 `src/` 코드는 파일 단위로 교체되며 이전 코드는 삭제된다.

**Tech Stack:** Rust 2021, `tokio`, `reqwest`, `tokio-tungstenite`, `serde_json`, `thiserror`, `async-trait`, `rust_decimal`, `chrono`, `shellexpand`, `futures`

**선행 조건:** Plan 1 완료 (workspace + domain 크레이트 존재)

**태스크 순서 원칙:** 각 태스크는 컴파일 가능한 상태에서 완료된다. `traits.rs`는 `EventReceiver`가 존재하는 Task 5 이후에 작성한다.

---

## File Map

```
crates/kis_api/src/
├── lib.rs              REWRITE — 공개 API (태스크 진행에 따라 점진적 업데이트)
├── config.rs           CREATE  — KisConfig + Builder
├── error.rs            REWRITE — KisError (Lagged, StreamClosed 포함)
├── event.rs            CREATE  — KisEvent, TransactionData 등
├── stream.rs           CREATE  — KisStream + EventReceiver (Task 5, traits.rs보다 먼저)
├── traits.rs           CREATE  — KisApi, KisEventSource 트레이트 (Task 6)
├── client.rs           REWRITE — KisClient (REST, Arc<Inner>)
├── auth/
│   └── mod.rs          REWRITE — TokenManager + current_kst() (FixedOffset 기반)
├── rest/
│   ├── mod.rs          CREATE
│   ├── http.rs         REWRITE — execute (내부)
│   └── overseas/
│       ├── mod.rs      CREATE
│       ├── deposit.rs  REWRITE — DepositInfo + check_deposit
│       └── price.rs    REWRITE — StockPrice + current_price
└── ws/
    ├── mod.rs          CREATE
    ├── connection.rs   CREATE  — WS 연결/재연결/구독 복구 루프
    └── message/
        ├── mod.rs      CREATE (또는 기존 파일 재사용)
        ├── request.rs  KEEP/수정
        └── response.rs KEEP/수정

# 삭제 대상 (Task 10에서 처리)
crates/kis_api/src/environment.rs
crates/kis_api/src/extentions/
crates/kis_api/src/logger.rs
crates/kis_api/src/core/
crates/kis_api/src/api/
crates/kis_api/src/types/
crates/kis_api/src/websockets.rs
crates/kis_api/src/websockets/   (디렉터리 형태인 경우)
```

---

## Task 0: `kis_api/Cargo.toml` 의존성 업데이트

Plan 2에서 사용할 신규 의존성을 먼저 추가. 이후 태스크가 컴파일 가능한 기반.

**Files:**
- Modify: `crates/kis_api/Cargo.toml`

- [ ] **Step 1: `Cargo.toml` 의존성 섹션 교체**

```toml
[package]
name = "kis_api"
version = "0.1.0"
edition = "2021"

[dependencies]
# HTTP
reqwest = { version = "0.12", features = ["json"] }
# WebSocket
tokio-tungstenite = "0.24"
# Async runtime
tokio = { version = "1", features = ["full"] }
futures = "0.3"
futures-util = "0.3"
# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
# Error handling
thiserror = "2"
# Async traits
async-trait = "0.1"
# Date/time (KST timezone support)
chrono = { version = "0.4", features = ["serde"] }
# Decimal arithmetic (no f64 for prices)
rust_decimal = { version = "1", features = ["serde-with-str"] }
rust_decimal_macros = "1"
# File path expansion (token cache)
shellexpand = "3"
# HTTP types
http = "1"
# Logging
log = "0.4"
# Domain shared types
domain = { path = "../domain" }
```

- [ ] **Step 2: `cargo check -p kis_api`**

```bash
cargo check -p kis_api
```

Expected: 의존성 다운로드 후 성공 (기존 코드와의 버전 충돌 없는지 확인)

- [ ] **Step 3: 커밋**

```bash
git add crates/kis_api/Cargo.toml
git commit -m "chore(kis_api): update dependencies for redesign"
```

---

## Task 1: `KisError` 재작성

**Files:**
- Rewrite: `crates/kis_api/src/error.rs`
- Modify: `crates/kis_api/src/lib.rs` (mod error만 유지, 나머지 주석 처리)

- [ ] **Step 1: 기존 `lib.rs`를 최소화 — 이후 태스크에서 점진적으로 복구**

```rust
// crates/kis_api/src/lib.rs
mod error;
pub use error::KisError;
```

- [ ] **Step 2: 실패하는 테스트 작성**

기존 `error.rs`를 아래 내용으로 교체:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KisError {
    #[error("auth error: {0}")]
    Auth(String),

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("KIS API error {code}: {message}")]
    Api { code: String, message: String },

    #[error("websocket error: {0}")]
    WebSocket(String),

    #[error("event stream lagged: {0} events missed")]
    Lagged(u64),

    #[error("event stream closed")]
    StreamClosed,

    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl KisError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Network(_) | Self::WebSocket(_))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retryable_errors() {
        assert!(KisError::WebSocket("timeout".into()).is_retryable());
        assert!(!KisError::Auth("invalid key".into()).is_retryable());
        assert!(!KisError::StreamClosed.is_retryable());
        assert!(!KisError::Lagged(5).is_retryable());
    }

    #[test]
    fn error_display_messages() {
        assert_eq!(
            KisError::Lagged(42).to_string(),
            "event stream lagged: 42 events missed"
        );
        assert_eq!(KisError::StreamClosed.to_string(), "event stream closed");
        assert_eq!(
            KisError::Api { code: "EGW00123".into(), message: "Unauthorized".into() }.to_string(),
            "KIS API error EGW00123: Unauthorized"
        );
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn kis_error_is_send_sync() {
        assert_send_sync::<KisError>();
    }
}
```

- [ ] **Step 3: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api error::tests 2>&1 | head -30
```

Expected: 컴파일 에러 (기존 `lib.rs`가 `error`를 이미 `mod error`로 선언한 경우 variant 불일치)

- [ ] **Step 4: `lib.rs` 최소화 후 테스트 재실행**

```bash
cargo test -p kis_api error::tests
```

Expected: 3개 PASS

- [ ] **Step 5: 커밋**

```bash
git add crates/kis_api/src/error.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): rewrite KisError with Lagged, StreamClosed, is_retryable"
```

---

## Task 2: `KisConfig` — Builder 패턴

**Files:**
- Create: `crates/kis_api/src/config.rs`

- [ ] **Step 1: `config.rs`에 테스트만 먼저 작성**

```rust
// crates/kis_api/src/config.rs — 테스트 먼저

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_config() -> KisConfig {
        KisConfig::builder()
            .app_key("test_key")
            .app_secret("test_secret")
            .account_num("12345678-01")
            .mock(true)
            .build()
            .unwrap()
    }

    #[test]
    fn builder_sets_mock_urls_automatically() {
        let cfg = mock_config();
        assert!(cfg.rest_url.contains("openapivts"));
        assert!(cfg.ws_url.contains("31000"));
    }

    #[test]
    fn builder_sets_production_urls_when_not_mock() {
        let cfg = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .build()  // mock 기본값 = false
            .unwrap();
        assert!(cfg.rest_url.contains("openapi.koreainvestment.com"));
        assert!(cfg.ws_url.contains("21000"));
    }

    #[test]
    fn builder_allows_url_override() {
        let cfg = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .rest_url("https://custom.example.com")
            .ws_url("ws://custom.example.com:9999")
            .build()
            .unwrap();
        assert_eq!(cfg.rest_url, "https://custom.example.com");
        assert_eq!(cfg.ws_url, "ws://custom.example.com:9999");
    }

    #[test]
    fn builder_fails_without_required_fields() {
        assert!(KisConfig::builder().build().is_err());
    }

    #[test]
    fn default_event_buffer_is_1024() {
        let cfg = mock_config();
        assert_eq!(cfg.ws_event_buffer, 1024);
    }

    #[test]
    fn token_cache_path_optional() {
        let cfg = mock_config();
        assert!(cfg.token_cache_path.is_none());

        let cfg2 = KisConfig::builder()
            .app_key("k").app_secret("s").account_num("a")
            .token_cache_path("~/.trading/token")
            .build()
            .unwrap();
        assert_eq!(cfg2.token_cache_path.as_deref(), Some("~/.trading/token"));
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api config::tests 2>&1 | head -20
```

Expected: FAIL (타입 미정의)

- [ ] **Step 3: 구현 작성 — 테스트 위에 추가**

파일 상단에 구현 추가 (테스트 모듈 앞):

```rust
#[derive(Debug, Clone)]
pub struct KisConfig {
    pub(crate) app_key: String,
    pub(crate) app_secret: String,
    pub(crate) account_num: String,
    pub(crate) mock: bool,
    pub(crate) rest_url: String,
    pub(crate) ws_url: String,
    pub(crate) ws_event_buffer: usize,
    pub(crate) token_cache_path: Option<String>,
}

impl KisConfig {
    pub fn builder() -> KisConfigBuilder {
        KisConfigBuilder::default()
    }
}

#[derive(Default)]
pub struct KisConfigBuilder {
    app_key: Option<String>,
    app_secret: Option<String>,
    account_num: Option<String>,
    mock: bool,
    rest_url: Option<String>,
    ws_url: Option<String>,
    ws_event_buffer: Option<usize>,
    token_cache_path: Option<String>,
}

impl KisConfigBuilder {
    pub fn app_key(mut self, v: impl Into<String>) -> Self   { self.app_key = Some(v.into()); self }
    pub fn app_secret(mut self, v: impl Into<String>) -> Self { self.app_secret = Some(v.into()); self }
    pub fn account_num(mut self, v: impl Into<String>) -> Self { self.account_num = Some(v.into()); self }
    pub fn mock(mut self, v: bool) -> Self                   { self.mock = v; self }
    pub fn rest_url(mut self, v: impl Into<String>) -> Self  { self.rest_url = Some(v.into()); self }
    pub fn ws_url(mut self, v: impl Into<String>) -> Self    { self.ws_url = Some(v.into()); self }
    pub fn ws_event_buffer(mut self, v: usize) -> Self       { self.ws_event_buffer = Some(v); self }
    pub fn token_cache_path(mut self, v: impl Into<String>) -> Self { self.token_cache_path = Some(v.into()); self }

    pub fn build(self) -> Result<KisConfig, String> {
        let (default_rest, default_ws) = if self.mock {
            ("https://openapivts.koreainvestment.com:9443", "ws://ops.koreainvestment.com:31000")
        } else {
            ("https://openapi.koreainvestment.com:9443",   "ws://ops.koreainvestment.com:21000")
        };
        Ok(KisConfig {
            app_key:          self.app_key.ok_or("app_key required")?,
            app_secret:       self.app_secret.ok_or("app_secret required")?,
            account_num:      self.account_num.ok_or("account_num required")?,
            mock:             self.mock,
            rest_url:         self.rest_url.unwrap_or_else(|| default_rest.into()),
            ws_url:           self.ws_url.unwrap_or_else(|| default_ws.into()),
            ws_event_buffer:  self.ws_event_buffer.unwrap_or(1024),
            token_cache_path: self.token_cache_path,
        })
    }
}
```

- [ ] **Step 4: `lib.rs`에 `mod config; pub use config::KisConfig;` 추가**

- [ ] **Step 5: 테스트 실행**

```bash
cargo test -p kis_api config::tests
```

Expected: 6개 PASS

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/config.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): add KisConfig builder with mock/prod URL switching"
```

---

## Task 3: `KisEvent` 타입 정의

**Files:**
- Create: `crates/kis_api/src/event.rs`

- [ ] **Step 1: 테스트만 먼저 작성**

```rust
// crates/kis_api/src/event.rs — 테스트 먼저
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn kis_event_is_clone_and_send_sync() {
        fn check<T: Clone + Send + Sync>() {}
        check::<KisEvent>();
    }

    #[test]
    fn transaction_data_uses_decimal_price() {
        let t = TransactionData {
            symbol: "NVDA".into(),
            price: dec!(134.20),
            volume: 50, buy_qty: 100, sell_qty: 80,
            timestamp: chrono::Utc::now(),
        };
        assert_eq!(t.price, dec!(134.20));
    }

    #[test]
    fn connected_disconnected_are_unit_variants() {
        let _ = (KisEvent::Connected.clone(), KisEvent::Disconnected.clone());
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api event::tests 2>&1 | head -10
```

Expected: FAIL (타입 미정의)

- [ ] **Step 3: 구현 작성**

파일 상단에 추가:

```rust
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KisEvent {
    Transaction(TransactionData),
    Quote(QuoteData),
    OrderConfirm(OrderConfirmData),
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    pub symbol: String,
    pub price: Decimal,
    pub volume: u64,
    pub buy_qty: u64,
    pub sell_qty: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteData {
    pub symbol: String,
    pub bid: Decimal,
    pub ask: Decimal,
    pub bid_qty: u64,
    pub ask_qty: u64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderConfirmData {
    pub order_id: String,
    pub symbol: String,
    pub filled_qty: u64,
    pub filled_price: Decimal,
    pub timestamp: DateTime<Utc>,
}
```

- [ ] **Step 4: `lib.rs`에 `mod event;` 및 pub use 추가**

- [ ] **Step 5: 테스트 실행**

```bash
cargo test -p kis_api event::tests
```

Expected: 3개 PASS

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/event.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): add KisEvent types with Decimal prices"
```

---

## Task 4: `TokenManager` — 인증 관리 (TDD)

기존 `token.rs` + `oauth_certification.rs` 통합. `current_kst()`를 `FixedOffset` 기반으로 올바르게 구현.

**Files:**
- Rewrite: `crates/kis_api/src/auth/mod.rs`

> **핵심 수정:** 기존 코드의 `Utc::now() + Duration::hours(9)` 패턴은 `DateTime<Utc>` 타입에 9시간을 더해 의미상 잘못된 시간을 생성한다. `FixedOffset::east_opt(9 * 3600)`으로 올바른 KST 시간대를 사용한다.

- [ ] **Step 1: 테스트만 먼저 작성**

```rust
// crates/kis_api/src/auth/mod.rs — 테스트 먼저
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn current_kst_offset_is_9_hours() {
        let kst = current_kst();
        // FixedOffset::east(9*3600) = UTC+9
        assert_eq!(kst.offset().local_minus_utc(), 9 * 3600);
    }

    #[test]
    fn empty_token_is_expired() {
        assert!(TokenData::empty().is_expired());
    }

    #[test]
    fn future_token_is_not_expired() {
        let future = current_kst() + Duration::hours(23);
        let token = TokenData {
            access_token: "tok".into(),
            access_token_token_expired: future.format("%Y-%m-%d %H:%M:%S").to_string(),
            token_type: "Bearer".into(),
            expires_in: 86400,
        };
        assert!(!token.is_expired());
    }

    #[test]
    fn past_token_is_expired() {
        let past = current_kst() - Duration::hours(1);
        let token = TokenData {
            access_token: "tok".into(),
            access_token_token_expired: past.format("%Y-%m-%d %H:%M:%S").to_string(),
            token_type: "Bearer".into(),
            expires_in: 1,
        };
        assert!(token.is_expired());
    }

    #[tokio::test]
    async fn manager_refreshes_expired_token() {
        let manager = TokenManager::new(None);
        let future = current_kst() + Duration::hours(23);
        let fresh = TokenData {
            access_token: "fresh".into(),
            access_token_token_expired: future.format("%Y-%m-%d %H:%M:%S").to_string(),
            token_type: "Bearer".into(),
            expires_in: 86400,
        };
        let result = manager.get_valid_token(async { Ok(fresh) }).await.unwrap();
        assert_eq!(result, "fresh");
    }

    #[tokio::test]
    async fn manager_skips_refresh_when_valid() {
        let manager = TokenManager::new(None);
        let future = current_kst() + Duration::hours(23);
        {
            let mut t = manager.token.lock().await;
            *t = TokenData {
                access_token: "valid".into(),
                access_token_token_expired: future.format("%Y-%m-%d %H:%M:%S").to_string(),
                token_type: "Bearer".into(),
                expires_in: 86400,
            };
        }
        let result = manager
            .get_valid_token(async { panic!("should not refresh") })
            .await
            .unwrap();
        assert_eq!(result, "valid");
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api auth::tests 2>&1 | head -20
```

Expected: FAIL (타입 미정의)

- [ ] **Step 3: 구현 작성**

파일 상단에 추가:

```rust
use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::error::KisError;

/// 현재 KST 시간 (UTC+9). `FixedOffset`으로 올바른 타임존 처리.
pub fn current_kst() -> DateTime<FixedOffset> {
    let kst = FixedOffset::east_opt(9 * 3600).expect("valid offset");
    chrono::Utc::now().with_timezone(&kst)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub access_token: String,
    pub access_token_token_expired: String,  // KIS API 필드명 그대로
    pub token_type: String,
    pub expires_in: i64,
}

impl TokenData {
    pub fn empty() -> Self {
        Self {
            access_token: String::new(),
            access_token_token_expired: current_kst()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            token_type: String::new(),
            expires_in: 0,
        }
    }

    pub fn is_expired(&self) -> bool {
        if self.expires_in == 0 { return true; }
        let naive = match NaiveDateTime::parse_from_str(
            &self.access_token_token_expired, "%Y-%m-%d %H:%M:%S",
        ) {
            Ok(dt) => dt,
            Err(_)  => return true,
        };
        // 만료 시각을 KST로 해석해 현재 KST와 비교
        let kst = FixedOffset::east_opt(9 * 3600).expect("valid offset");
        let expiry = kst.from_local_datetime(&naive)
            .single()
            .unwrap_or_else(|| kst.from_local_datetime(&naive).earliest().unwrap());
        current_kst() >= expiry
    }
}

pub struct TokenManager {
    pub(crate) token: Arc<Mutex<TokenData>>,
    cache_path: Option<String>,
}

impl TokenManager {
    pub fn new(cache_path: Option<String>) -> Self {
        let initial = cache_path
            .as_deref()
            .and_then(Self::load_from_file)
            .unwrap_or_else(TokenData::empty);
        Self {
            token: Arc::new(Mutex::new(initial)),
            cache_path,
        }
    }

    pub async fn get_valid_token(
        &self,
        refresh: impl std::future::Future<Output = Result<TokenData, KisError>>,
    ) -> Result<String, KisError> {
        let mut token = self.token.lock().await;
        if token.is_expired() {
            *token = refresh.await?;
            if let Some(path) = &self.cache_path {
                let _ = self.save_to_file(&token, path);
            }
        }
        Ok(token.access_token.clone())
    }

    fn save_to_file(&self, token: &TokenData, path: &str) -> Result<(), KisError> {
        let expanded = shellexpand::tilde(path).to_string();
        if let Some(parent) = std::path::Path::new(&expanded).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&expanded, serde_json::to_string(token)?)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Option<TokenData> {
        let expanded = shellexpand::tilde(path).to_string();
        let content = std::fs::read_to_string(expanded).ok()?;
        serde_json::from_str(&content).ok()
    }
}
```

- [ ] **Step 4: `lib.rs`에 `mod auth;` 추가 (pub use 불필요 — internal)**

- [ ] **Step 5: 테스트 실행**

```bash
cargo test -p kis_api auth::tests
```

Expected: 6개 PASS

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/auth/mod.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): add TokenManager with FixedOffset KST and file cache"
```

---

## Task 5: `KisStream` + `EventReceiver` (WebSocket 핸들)

**`traits.rs`보다 먼저 작성해야 한다** — `traits.rs`가 `EventReceiver` 타입을 참조하기 때문.

**Files:**
- Create: `crates/kis_api/src/stream.rs`

- [ ] **Step 1: 테스트만 먼저 작성**

```rust
// crates/kis_api/src/stream.rs — 테스트 먼저
#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::KisEvent;

    fn make_stream() -> KisStream {
        KisStream::new("ws://test".into(), 128)
    }

    #[tokio::test]
    async fn receiver_gets_sent_events() {
        let stream = make_stream();
        let mut rx = stream.receiver();
        let tx = stream.sender();
        tx.send(KisEvent::Connected).unwrap();
        assert!(matches!(rx.recv().await.unwrap(), KisEvent::Connected));
    }

    #[tokio::test]
    async fn multiple_receivers_each_get_event() {
        let stream = make_stream();
        let mut rx1 = stream.receiver();
        let mut rx2 = stream.receiver();
        let tx = stream.sender();
        tx.send(KisEvent::Connected).unwrap();
        assert!(matches!(rx1.recv().await.unwrap(), KisEvent::Connected));
        assert!(matches!(rx2.recv().await.unwrap(), KisEvent::Connected));
    }

    #[tokio::test]
    async fn stream_closed_error_on_sender_drop() {
        let stream = make_stream();
        let mut rx = stream.receiver();
        drop(stream);
        assert!(matches!(rx.recv().await, Err(KisError::StreamClosed)));
    }

    #[tokio::test]
    async fn subscribe_and_unsubscribe() {
        let stream = make_stream();
        stream.subscribe("NVDA").await.unwrap();
        stream.subscribe("AAPL").await.unwrap();
        assert!(stream.subscriptions().await.contains("NVDA"));
        stream.unsubscribe("NVDA").await.unwrap();
        assert!(!stream.subscriptions().await.contains("NVDA"));
        assert!(stream.subscriptions().await.contains("AAPL"));
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn kis_stream_is_send_sync() {
        assert_send_sync::<KisStream>();
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api stream::tests 2>&1 | head -10
```

Expected: FAIL (타입 미정의)

- [ ] **Step 3: 구현 작성**

파일 상단에 추가:

```rust
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use crate::error::KisError;
use crate::event::KisEvent;

pub struct EventReceiver {
    inner: broadcast::Receiver<KisEvent>,
}

impl EventReceiver {
    pub(crate) fn new(rx: broadcast::Receiver<KisEvent>) -> Self {
        Self { inner: rx }
    }

    pub async fn recv(&mut self) -> Result<KisEvent, KisError> {
        match self.inner.recv().await {
            Ok(ev)                                      => Ok(ev),
            Err(broadcast::error::RecvError::Lagged(n)) => Err(KisError::Lagged(n)),
            Err(broadcast::error::RecvError::Closed)    => Err(KisError::StreamClosed),
        }
    }

    pub fn into_stream(self) -> impl futures::Stream<Item = Result<KisEvent, KisError>> {
        futures::stream::unfold(self, |mut rx| async move {
            Some((rx.recv().await, rx))
        })
    }
}

pub(crate) struct KisStreamInner {
    pub(crate) tx: broadcast::Sender<KisEvent>,
    pub(crate) subscriptions: Mutex<HashSet<String>>,
    pub(crate) ws_url: String,
}

#[derive(Clone)]
pub struct KisStream {
    pub(crate) inner: Arc<KisStreamInner>,
}

impl KisStream {
    pub(crate) fn new(ws_url: String, buffer_size: usize) -> Self {
        let (tx, _) = broadcast::channel(buffer_size);
        Self {
            inner: Arc::new(KisStreamInner {
                tx,
                subscriptions: Mutex::new(HashSet::new()),
                ws_url,
            }),
        }
    }

    pub(crate) fn sender(&self) -> broadcast::Sender<KisEvent> {
        self.inner.tx.clone()
    }

    pub fn receiver(&self) -> EventReceiver {
        EventReceiver::new(self.inner.tx.subscribe())
    }

    pub async fn subscribe(&self, symbol: &str) -> Result<(), KisError> {
        self.inner.subscriptions.lock().await.insert(symbol.to_string());
        Ok(())
    }

    pub async fn unsubscribe(&self, symbol: &str) -> Result<(), KisError> {
        self.inner.subscriptions.lock().await.remove(symbol);
        Ok(())
    }

    pub async fn subscriptions(&self) -> HashSet<String> {
        self.inner.subscriptions.lock().await.clone()
    }
}
```

- [ ] **Step 4: `lib.rs`에 `mod stream; pub use stream::{KisStream, EventReceiver};` 추가**

- [ ] **Step 5: 테스트 실행**

```bash
cargo test -p kis_api stream::tests
```

Expected: 5개 PASS

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/stream.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): add KisStream and EventReceiver with broadcast fan-out"
```

---

## Task 6: `KisApi` + `KisEventSource` 트레이트

**`stream.rs`가 존재하므로 이제 `EventReceiver` 참조 가능.**

**Files:**
- Create: `crates/kis_api/src/traits.rs`

- [ ] **Step 1: `traits.rs` 작성**

```rust
use async_trait::async_trait;
use rust_decimal::Decimal;
use domain::types::OrderRequest;
use crate::error::KisError;
use crate::stream::EventReceiver;

// REST 응답 타입
#[derive(Debug, Clone)]
pub struct DepositInfo {
    pub available: Decimal,
    pub currency: String,
}

#[derive(Debug, Clone)]
pub struct StockPrice {
    pub symbol: String,
    pub price: Decimal,
    pub volume: u64,
}

#[derive(Debug, Clone)]
pub struct OrderResult {
    pub order_id: String,
}

// REST 추상화 — bot은 이것에만 의존 (mock 테스트 가능)
#[async_trait]
pub trait KisApi: Send + Sync {
    async fn check_deposit(&self)                  -> Result<DepositInfo, KisError>;
    async fn current_price(&self, symbol: &str)    -> Result<StockPrice, KisError>;
    async fn place_order(&self, req: OrderRequest) -> Result<OrderResult, KisError>;
}

/// WebSocket 추상화 — bot은 이것에만 의존.
/// `receiver()`는 소비자당 한 번만 호출하고 결과를 보관해야 한다.
/// 동일한 `KisStream`에서 여러 번 호출하면 각자 독립적인 수신자를 얻는다.
pub trait KisEventSource: Send + Sync {
    fn receiver(&self) -> EventReceiver;
}

impl KisEventSource for KisStream {
    fn receiver(&self) -> EventReceiver {
        KisStream::receiver(self)
    }
}

use crate::stream::KisStream;

#[cfg(test)]
mod tests {
    use super::*;

    struct MockApi;

    #[async_trait]
    impl KisApi for MockApi {
        async fn check_deposit(&self) -> Result<DepositInfo, KisError> {
            Ok(DepositInfo { available: rust_decimal_macros::dec!(1000.0), currency: "USD".into() })
        }
        async fn current_price(&self, symbol: &str) -> Result<StockPrice, KisError> {
            Ok(StockPrice { symbol: symbol.into(), price: rust_decimal_macros::dec!(130.0), volume: 0 })
        }
        async fn place_order(&self, _req: OrderRequest) -> Result<OrderResult, KisError> {
            Ok(OrderResult { order_id: "mock-001".into() })
        }
    }

    #[tokio::test]
    async fn mock_api_implements_trait() {
        let api: Box<dyn KisApi> = Box::new(MockApi);
        let price = api.current_price("NVDA").await.unwrap();
        assert_eq!(price.symbol, "NVDA");
    }

    #[test]
    fn kis_stream_implements_event_source() {
        fn check<T: KisEventSource>(_: &T) {}
        let stream = KisStream::new("ws://test".into(), 16);
        check(&stream);
    }
}
```

- [ ] **Step 2: `lib.rs`에 `mod traits;` 및 pub use 추가**

```rust
pub use traits::{DepositInfo, KisApi, KisEventSource, OrderResult, StockPrice};
```

- [ ] **Step 3: 테스트 실행**

```bash
cargo test -p kis_api traits::tests
```

Expected: 2개 PASS

- [ ] **Step 4: 커밋**

```bash
git add crates/kis_api/src/traits.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): add KisApi and KisEventSource traits with mock example"
```

---

## Task 7: REST HTTP 내부 클라이언트

**Files:**
- Create: `crates/kis_api/src/rest/mod.rs`
- Create: `crates/kis_api/src/rest/http.rs`

- [ ] **Step 1: 테스트만 먼저 작성**

`crates/kis_api/src/rest/http.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn url_join_path_with_leading_slash() {
        use reqwest::Url;
        let base = "https://openapi.koreainvestment.com:9443";
        let url = Url::parse(base).unwrap().join("/oauth2/tokenP").unwrap();
        assert_eq!(url.as_str(), "https://openapi.koreainvestment.com:9443/oauth2/tokenP");
    }

    #[test]
    fn url_join_does_not_duplicate_slash() {
        use reqwest::Url;
        let base = "https://example.com:9443/";
        let url = Url::parse(base).unwrap().join("/api/v1").unwrap();
        assert!(url.as_str().starts_with("https://example.com:9443/api/v1"));
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api rest::http::tests 2>&1 | head -10
```

Expected: FAIL (모듈 없음)

- [ ] **Step 3: 구현 작성**

파일 상단에 추가:

```rust
use std::time::Duration;
use reqwest::{Client, Method, Url, header::HeaderMap};
use serde_json::Value;
use crate::error::KisError;

pub(crate) async fn execute(
    client:   &Client,
    base_url: &str,
    path:     &str,
    method:   Method,
    headers:  Option<HeaderMap>,
    body:     Option<Value>,
    query:    Option<Vec<(String, String)>>,
) -> Result<reqwest::Response, KisError> {
    let url = Url::parse(base_url)
        .map_err(|e| KisError::WebSocket(e.to_string()))?
        .join(path)
        .map_err(|e| KisError::WebSocket(e.to_string()))?;

    let mut req = client.request(method, url).timeout(Duration::from_secs(30));
    if let Some(h) = headers { req = req.headers(h); }
    if let Some(b) = body    { req = req.json(&b); }
    if let Some(q) = query   { req = req.query(&q); }

    let resp = req.send().await?;
    if resp.status().is_success() {
        Ok(resp)
    } else {
        let code = resp.status().as_u16().to_string();
        let msg  = resp.text().await.unwrap_or_default();
        Err(KisError::Api { code, message: msg })
    }
}
```

`crates/kis_api/src/rest/mod.rs`:

```rust
pub(crate) mod http;
pub mod overseas;
```

- [ ] **Step 4: `lib.rs`에 `mod rest;` 추가**

- [ ] **Step 5: 테스트 실행**

```bash
cargo test -p kis_api rest::http::tests
```

Expected: 2개 PASS

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/rest/
git commit -m "feat(kis_api): add internal REST HTTP executor"
```

---

## Task 8: 해외 REST API 엔드포인트

**Files:**
- Create: `crates/kis_api/src/rest/overseas/mod.rs`
- Create: `crates/kis_api/src/rest/overseas/deposit.rs`
- Create: `crates/kis_api/src/rest/overseas/price.rs`

- [ ] **Step 1: 테스트만 먼저 작성**

`crates/kis_api/src/rest/overseas/deposit.rs`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn account_num_split_at_dash() {
        let (num, typ) = "12345678-01".split_once('-').unwrap();
        assert_eq!(num, "12345678");
        assert_eq!(typ, "01");
    }

    #[test]
    fn account_num_without_dash_returns_none() {
        assert!("12345678".split_once('-').is_none());
    }
}
```

- [ ] **Step 2: 테스트 실행 — 통과 확인** (단순 문자열 테스트라 바로 통과)

```bash
cargo test -p kis_api rest::overseas::deposit::tests
```

Expected: 2개 PASS

- [ ] **Step 3: `deposit.rs` 구현 작성**

파일 상단에 추가:

```rust
use reqwest::{Client, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use rust_decimal::Decimal;
use serde::de::Error as DeError;
use std::str::FromStr;
use crate::{config::KisConfig, error::KisError, rest::http, traits::DepositInfo};

pub(crate) async fn check_deposit(
    client: &Client, config: &KisConfig, token: &str, account_num: &str,
) -> Result<DepositInfo, KisError> {
    let mut h = HeaderMap::new();
    h.insert(HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| KisError::Auth(e.to_string()))?);
    h.insert(HeaderName::from_static("appkey"),
        HeaderValue::from_str(&config.app_key)
            .map_err(|e| KisError::Auth(e.to_string()))?);
    h.insert(HeaderName::from_static("appsecret"),
        HeaderValue::from_str(&config.app_secret)
            .map_err(|e| KisError::Auth(e.to_string()))?);
    h.insert(HeaderName::from_static("tr_id"),
        HeaderValue::from_static(if config.mock { "VTTS3012R" } else { "TTTS3012R" }));
    h.insert(HeaderName::from_static("tr_cont"), HeaderValue::from_static(""));

    let (a_num, a_type) = account_num.split_once('-')
        .ok_or_else(|| KisError::Auth("account_num must be in 'XXXXXXXX-YY' format".into()))?;

    let query = vec![
        ("CANO".into(), a_num.into()), ("ACNT_PRDT_CD".into(), a_type.into()),
        ("OVRS_EXCG_CD".into(), "NAS".into()), ("TR_CRCY_CD".into(), "USD".into()),
        ("CTX_AREA_FK200".into(), "".into()), ("CTX_AREA_NK200".into(), "".into()),
    ];

    let resp = http::execute(client, &config.rest_url,
        "/uapi/overseas-stock/v1/trading/inquire-balance",
        Method::GET, Some(h), None, Some(query)).await?;

    let json: serde_json::Value = resp.json().await?;
    let raw = json["output"]["frcr_dncl_amt_2"].as_str().unwrap_or("0");
    let available = Decimal::from_str(raw)
        .map_err(|e| KisError::Parse(serde_json::Error::custom(e.to_string())))?;

    Ok(DepositInfo { available, currency: "USD".into() })
}
```

> **Note:** `serde_json::Error::custom()` 사용 시 `use serde::de::Error as DeError;` 필요.
> 이미 파일 상단에 포함되어 있다.

- [ ] **Step 4: `price.rs` 구현 작성**

`crates/kis_api/src/rest/overseas/price.rs`:

```rust
use reqwest::{Client, Method, header::{HeaderMap, HeaderName, HeaderValue}};
use rust_decimal::Decimal;
use serde::de::Error as DeError;
use std::str::FromStr;
use crate::{config::KisConfig, error::KisError, rest::http, traits::StockPrice};

pub(crate) async fn current_price(
    client: &Client, config: &KisConfig, token: &str, symbol: &str,
) -> Result<StockPrice, KisError> {
    let mut h = HeaderMap::new();
    h.insert(HeaderName::from_static("authorization"),
        HeaderValue::from_str(&format!("Bearer {}", token))
            .map_err(|e| KisError::Auth(e.to_string()))?);
    h.insert(HeaderName::from_static("appkey"),
        HeaderValue::from_str(&config.app_key)
            .map_err(|e| KisError::Auth(e.to_string()))?);
    h.insert(HeaderName::from_static("appsecret"),
        HeaderValue::from_str(&config.app_secret)
            .map_err(|e| KisError::Auth(e.to_string()))?);
    h.insert(HeaderName::from_static("tr_id"),
        HeaderValue::from_static(if config.mock { "VTTS1002U" } else { "HHDFS00000300" }));

    let query = vec![
        ("AUTH".into(), "".into()), ("EXCD".into(), "NAS".into()),
        ("SYMB".into(), symbol.to_string()),
    ];

    let resp = http::execute(client, &config.rest_url,
        "/uapi/overseas-stock/v1/quotations/price",
        Method::GET, Some(h), None, Some(query)).await?;

    let json: serde_json::Value = resp.json().await?;
    let raw = json["output"]["last"].as_str().unwrap_or("0");
    let price = Decimal::from_str(raw)
        .map_err(|e| KisError::Parse(serde_json::Error::custom(e.to_string())))?;

    Ok(StockPrice { symbol: symbol.into(), price, volume: 0 })
}
```

`crates/kis_api/src/rest/overseas/mod.rs`:

```rust
pub(crate) mod deposit;
pub(crate) mod price;
```

- [ ] **Step 5: 전체 테스트 실행**

```bash
cargo test -p kis_api rest
```

Expected: PASS

- [ ] **Step 6: 커밋**

```bash
git add crates/kis_api/src/rest/overseas/
git commit -m "feat(kis_api): implement check_deposit and current_price REST endpoints"
```

---

## Task 9: WebSocket 연결 루프 (`connection.rs`)

**Files:**
- Create: `crates/kis_api/src/ws/mod.rs`
- Create: `crates/kis_api/src/ws/message/mod.rs`
- Create: `crates/kis_api/src/ws/connection.rs`

> **핵심 수정 (C2):** 기존 설계의 `subscriptions` 클로저에서 `block_on` 호출은 Tokio 런타임 안에서 panic을 발생시킨다. 대신 `Arc<Mutex<HashSet<String>>>`을 직접 전달한다.

- [ ] **Step 1: 테스트만 먼저 작성**

`crates/kis_api/src/ws/connection.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_transaction_message() {
        let mut fields = vec![""; 26];
        fields[1] = "NVDA";
        fields[11] = "134.20";
        let data = fields.join("^");
        let msg = format!("0|HDFSCNT0|1|{}", data);
        let result = parse_message(&msg);
        assert!(matches!(result, Some(crate::event::KisEvent::Transaction(_))));
    }

    #[test]
    fn parse_json_returns_none() {
        let json = r#"{"header":{"tr_id":"PINGPONG"},"body":{}}"#;
        assert!(parse_message(json).is_none());
    }

    #[test]
    fn parse_unknown_trid_returns_none() {
        assert!(parse_message("0|UNKNOWN|1|data").is_none());
    }

    #[test]
    fn parse_malformed_returns_none() {
        assert!(parse_message("not|enough").is_none());
    }
}
```

- [ ] **Step 2: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api ws::connection::tests 2>&1 | head -10
```

Expected: FAIL (모듈 없음)

- [ ] **Step 3: `connection.rs` 구현 작성**

파일 상단에 추가:

```rust
use std::collections::HashSet;
use std::sync::Arc;
use futures::{SinkExt, StreamExt};
use log::{error, info, warn};
use tokio::sync::{broadcast, Mutex};
use tokio::time::{sleep, Duration};
use tokio_tungstenite::{connect_async, tungstenite::Message as WsMessage};
use crate::event::{KisEvent, QuoteData, TransactionData};

/// WS 연결 루프. `subscriptions`는 `Arc<Mutex<HashSet>>` — 재연결 시 현재 목록을 비동기로 읽음.
/// `block_on` 사용 금지 — Tokio 런타임 안에서 panic 발생.
pub async fn run_connection_loop(
    ws_url: String,
    approval_key: String,
    subscriptions: Arc<Mutex<HashSet<String>>>,
    tx: broadcast::Sender<KisEvent>,
) {
    loop {
        let _ = tx.send(KisEvent::Disconnected);
        info!("WS connecting: {}", ws_url);

        match connect_async(&ws_url).await {
            Ok((ws, _)) => {
                let _ = tx.send(KisEvent::Connected);
                info!("WS connected");
                let (mut write, mut read) = ws.split();

                // 재연결 후 구독 목록 복구 (await OK — async 컨텍스트)
                let subs = subscriptions.lock().await.clone();
                for symbol in &subs {
                    let msg = make_subscribe_json(&approval_key, symbol, true);
                    let _ = write.send(WsMessage::Text(msg)).await;
                }

                while let Some(msg) = read.next().await {
                    match msg {
                        Ok(WsMessage::Text(text)) => {
                            if let Some(ev) = parse_message(&text) {
                                let _ = tx.send(ev);
                            }
                        }
                        Ok(WsMessage::Close(_)) => { info!("WS closed"); break; }
                        Err(e) => { error!("WS recv error: {}", e); break; }
                        _ => {}
                    }
                }
            }
            Err(e) => error!("WS connect failed: {}", e),
        }

        warn!("WS disconnected — retry in 5s");
        sleep(Duration::from_secs(5)).await;
    }
}

fn make_subscribe_json(approval_key: &str, symbol: &str, subscribe: bool) -> String {
    serde_json::json!({
        "header": {
            "approval_key": approval_key,
            "custtype": "P",
            "tr_type": if subscribe { "1" } else { "2" },
            "content-type": "utf-8"
        },
        "body": {
            "input": { "tr_id": "HDFSCNT0", "tr_key": format!("DNAS{}", symbol) }
        }
    }).to_string()
}

/// 파이프 구분 텍스트 메시지를 `KisEvent`로 파싱. JSON은 None 반환 (ping-pong 등).
fn parse_message(text: &str) -> Option<KisEvent> {
    // JSON이면 무시 (ping-pong 응답 등)
    if serde_json::from_str::<serde_json::Value>(text).is_ok() {
        return None;
    }

    let parts: Vec<&str> = text.splitn(4, '|').collect();
    if parts.len() < 4 { return None; }

    let tr_id = parts[1];
    let data  = parts[3];

    match tr_id {
        "HDFSCNT0"              => Some(parse_transaction(data)),
        "HDFSASP0" | "HDFSASP1" => Some(parse_quote(data)),
        _                       => None,
    }
}

fn parse_transaction(data: &str) -> KisEvent {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    let f: Vec<&str> = data.split('^').collect();
    KisEvent::Transaction(TransactionData {
        symbol:    f.get(1).unwrap_or(&"").to_string(),
        price:     Decimal::from_str(f.get(11).unwrap_or(&"0")).unwrap_or_default(),
        volume:    f.get(19).unwrap_or(&"0").parse().unwrap_or(0),
        buy_qty:   f.get(17).unwrap_or(&"0").parse().unwrap_or(0),
        sell_qty:  f.get(18).unwrap_or(&"0").parse().unwrap_or(0),
        timestamp: chrono::Utc::now(),
    })
}

fn parse_quote(data: &str) -> KisEvent {
    use rust_decimal::Decimal;
    use std::str::FromStr;
    let f: Vec<&str> = data.split('^').collect();
    KisEvent::Quote(QuoteData {
        symbol:    f.get(1).unwrap_or(&"").to_string(),
        bid:       Decimal::from_str(f.get(15).unwrap_or(&"0")).unwrap_or_default(),
        ask:       Decimal::from_str(f.get(14).unwrap_or(&"0")).unwrap_or_default(),
        bid_qty:   f.get(17).unwrap_or(&"0").parse().unwrap_or(0),
        ask_qty:   f.get(16).unwrap_or(&"0").parse().unwrap_or(0),
        timestamp: chrono::Utc::now(),
    })
}
```

- [ ] **Step 4: `ws/mod.rs` 및 `ws/message/mod.rs` 생성**

`crates/kis_api/src/ws/mod.rs`:
```rust
pub(crate) mod connection;
pub(crate) mod message;
```

`crates/kis_api/src/ws/message/mod.rs`:
```rust
// 기존 request.rs / response.rs 파일이 있으면 유지.
// 없으면 빈 파일로 생성 후 이후 필요 시 채움.
pub mod request;
pub mod response;
```

기존 `src/websockets/message/request.rs`, `response.rs`가 있으면 복사:
```bash
cp crates/kis_api/src/websockets/message/request.rs crates/kis_api/src/ws/message/request.rs 2>/dev/null || echo "// placeholder" > crates/kis_api/src/ws/message/request.rs
cp crates/kis_api/src/websockets/message/response.rs crates/kis_api/src/ws/message/response.rs 2>/dev/null || echo "// placeholder" > crates/kis_api/src/ws/message/response.rs
```

- [ ] **Step 5: `lib.rs`에 `mod ws;` 추가**

- [ ] **Step 6: 테스트 실행**

```bash
cargo test -p kis_api ws::connection::tests
```

Expected: 4개 PASS

- [ ] **Step 7: 커밋**

```bash
git add crates/kis_api/src/ws/
git commit -m "feat(kis_api): add WS connection loop with Arc<Mutex> subscriptions (no block_on)"
```

---

## Task 10: `KisClient` 완성

**Files:**
- Rewrite: `crates/kis_api/src/client.rs`
- Update: `crates/kis_api/src/auth/mod.rs` (OAuth 호출 추가)

- [ ] **Step 1: `auth/mod.rs`에 OAuth 발급 함수 추가**

기존 `auth/mod.rs` 하단에 추가:

```rust
// OAuth 관련 imports (파일 상단에도 추가 필요)
// use reqwest::Client; use crate::config::KisConfig; use crate::rest::http; use reqwest::Method;

pub(crate) async fn fetch_api_token(
    client: &reqwest::Client,
    config: &crate::config::KisConfig,
) -> Result<TokenData, KisError> {
    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &config.app_key,
        "appsecret": &config.app_secret,
    });
    let resp = crate::rest::http::execute(
        client, &config.rest_url, "/oauth2/tokenP",
        reqwest::Method::POST, None, Some(body), None,
    ).await?;
    Ok(resp.json::<TokenData>().await?)
}

pub(crate) async fn fetch_websocket_key(
    client: &reqwest::Client,
    config: &crate::config::KisConfig,
) -> Result<String, KisError> {
    let body = serde_json::json!({
        "grant_type": "client_credentials",
        "appkey": &config.app_key,
        "secretkey": &config.app_secret,
    });
    let resp = crate::rest::http::execute(
        client, &config.rest_url, "/oauth2/Approval",
        reqwest::Method::POST, None, Some(body), None,
    ).await?;
    let json: serde_json::Value = resp.json().await?;
    Ok(json["approval_key"].as_str().unwrap_or("").to_string())
}
```

- [ ] **Step 2: `client.rs` 테스트 먼저 작성**

`crates/kis_api/src/client.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> KisConfig {
        KisConfig::builder()
            .app_key("k").app_secret("s").account_num("12345678-01")
            .mock(true).build().unwrap()
    }

    #[test]
    fn kis_client_is_clone() {
        let _ = KisClient::new(test_config()).clone();
    }

    #[test]
    fn kis_client_is_send_sync() {
        fn check<T: Send + Sync>() {}
        check::<KisClient>();
    }

    #[test]
    fn implements_kis_api_trait() {
        fn check<T: KisApi>() {}
        check::<KisClient>();
    }
}
```

- [ ] **Step 3: 테스트 실행 — 실패 확인**

```bash
cargo test -p kis_api client::tests 2>&1 | head -10
```

Expected: FAIL (타입 미정의)

- [ ] **Step 4: `client.rs` 구현 작성**

파일 상단에 추가:

```rust
use std::sync::Arc;
use reqwest::Client;
use async_trait::async_trait;
use domain::types::OrderRequest;

use crate::auth::{TokenManager, fetch_api_token, fetch_websocket_key};
use crate::config::KisConfig;
use crate::error::KisError;
use crate::stream::KisStream;
use crate::traits::{DepositInfo, KisApi, OrderResult, StockPrice};

struct KisClientInner {
    config:          KisConfig,
    http:            Client,
    token_manager:   TokenManager,
}

#[derive(Clone)]
pub struct KisClient {
    inner: Arc<KisClientInner>,
}

impl KisClient {
    pub fn new(config: KisConfig) -> Self {
        let manager = TokenManager::new(config.token_cache_path.clone());
        Self {
            inner: Arc::new(KisClientInner { config, http: Client::new(), token_manager: manager }),
        }
    }

    async fn valid_token(&self) -> Result<String, KisError> {
        let inner = &self.inner;
        inner.token_manager
            .get_valid_token(fetch_api_token(&inner.http, &inner.config))
            .await
    }

    /// WebSocket 핸들 생성 — 연결 시작 및 재연결 루프 spawn.
    pub async fn stream(&self) -> Result<KisStream, KisError> {
        let approval_key = fetch_websocket_key(&self.inner.http, &self.inner.config).await?;
        let config = self.inner.config.clone();
        let stream = KisStream::new(config.ws_url.clone(), config.ws_event_buffer);

        // Arc<Mutex<HashSet>>를 직접 전달 — block_on 사용 금지
        let subscriptions = Arc::clone(&stream.inner.subscriptions);
        let tx = stream.sender();

        tokio::spawn(async move {
            crate::ws::connection::run_connection_loop(
                config.ws_url,
                approval_key,
                subscriptions,
                tx,
            ).await;
        });

        Ok(stream)
    }
}

#[async_trait]
impl KisApi for KisClient {
    async fn check_deposit(&self) -> Result<DepositInfo, KisError> {
        let token = self.valid_token().await?;
        crate::rest::overseas::deposit::check_deposit(
            &self.inner.http, &self.inner.config,
            &token, &self.inner.config.account_num,
        ).await
    }

    async fn current_price(&self, symbol: &str) -> Result<StockPrice, KisError> {
        let token = self.valid_token().await?;
        crate::rest::overseas::price::current_price(
            &self.inner.http, &self.inner.config, &token, symbol,
        ).await
    }

    async fn place_order(&self, _req: OrderRequest) -> Result<OrderResult, KisError> {
        // TODO: Plan 2 추가 구현
        Err(KisError::Api { code: "NOT_IMPLEMENTED".into(), message: "place_order TBD".into() })
    }
}
```

- [ ] **Step 5: `KisStreamInner.subscriptions` 필드를 `pub(crate)` Arc로 접근 가능한지 확인**

`stream.rs`에서 `KisStreamInner`와 `subscriptions` 필드가 `pub(crate)`인지 확인. Task 5에서 이미 설정됨.

- [ ] **Step 6: `lib.rs` 최종 정리**

```rust
// crates/kis_api/src/lib.rs
mod auth;
mod client;
mod config;
mod error;
mod event;
mod rest;
mod stream;
mod traits;
mod ws;

pub use client::KisClient;
pub use config::KisConfig;
pub use error::KisError;
pub use event::{KisEvent, OrderConfirmData, QuoteData, TransactionData};
pub use stream::EventReceiver;
pub use stream::KisStream;
pub use traits::{DepositInfo, KisApi, KisEventSource, OrderResult, StockPrice};
```

- [ ] **Step 7: 테스트 실행**

```bash
cargo test -p kis_api client::tests
```

Expected: 3개 PASS

- [ ] **Step 8: 커밋**

```bash
git add crates/kis_api/src/client.rs crates/kis_api/src/auth/mod.rs crates/kis_api/src/lib.rs
git commit -m "feat(kis_api): complete KisClient with TokenManager and KisApi impl"
```

---

## Task 11: 구 파일 삭제 및 최종 정리

**Files:**
- Delete: 아래 목록 참조

- [ ] **Step 1: 불필요 파일/디렉터리 삭제**

```bash
# 구조 교체 완료된 파일 삭제
rm -f  crates/kis_api/src/environment.rs
rm -f  crates/kis_api/src/logger.rs
rm -f  crates/kis_api/src/websockets.rs    # 루트 파일 형태
rm -rf crates/kis_api/src/websockets/      # 디렉터리 형태
rm -rf crates/kis_api/src/extentions/
rm -rf crates/kis_api/src/core/
rm -rf crates/kis_api/src/api/
rm -rf crates/kis_api/src/types/
```

- [ ] **Step 2: `cargo build -p kis_api`**

```bash
cargo build -p kis_api
```

Expected: 성공. 삭제한 파일을 `lib.rs`가 아직 참조하면 에러 발생 — `lib.rs`에서 해당 `mod` 선언 제거.

- [ ] **Step 3: workspace 전체 테스트**

```bash
cargo test
```

Expected:
- `domain`: 전체 PASS
- `kis_api`: 전체 PASS
- `bot`, `server`: PASS (skeleton)

- [ ] **Step 4: `cargo clippy`**

```bash
cargo clippy -- -D warnings
```

경고 발생 시 수정 후 재실행.

- [ ] **Step 5: 최종 커밋**

```bash
git add -A
git commit -m "chore(kis_api): remove legacy files, fix clippy warnings"
```

---

## 완료 기준

- [ ] `cargo build` workspace 전체 성공
- [ ] `cargo test` 전체 PASS
- [ ] `unsafe`, 전역 상태, `lazy_static` 없음
- [ ] `f64` 금액 계산 없음 (모두 `Decimal`)
- [ ] `current_kst()`가 `FixedOffset::east_opt(9*3600)` 기반
- [ ] `KisClient`가 `KisApi` 트레이트 구현
- [ ] `KisStream`이 `KisEventSource` 트레이트 구현
- [ ] `EventReceiver::recv()`가 `Lagged`, `StreamClosed` 에러 반환
- [ ] `run_connection_loop()`에서 `block_on` 없음 — `Arc<Mutex<HashSet>>` 직접 전달
- [ ] WS 재연결 시 구독 자동 복구
- [ ] `ws/message/mod.rs` 존재
- [ ] `types/`, `websockets.rs` 삭제 완료
