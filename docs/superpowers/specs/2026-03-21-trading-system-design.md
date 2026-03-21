# Trading System — Architecture Design Spec

**Date:** 2026-03-21
**Status:** Approved
**Scope:** Full system architecture — Cargo workspace, `kis_api` crate design, inter-crate communication, Telegram integration

---

## 1. Goals

- 개인 사설 서버에서 24/7 자동 매매 봇 운영
- KIS(한국투자증권) REST/WebSocket API를 타입 안전하게 추상화하는 라이브러리 제공
- 웹 대시보드 + 텔레그램을 통한 사용자 상호작용 지원
- 멀티 크레이트 모노레포로 관심사 분리 및 독립적 테스트 가능

---

## 2. Repository Structure

단일 Cargo workspace 모노레포.

```
trading/
├── Cargo.toml                  # workspace 정의
├── crates/
│   ├── kis_api/                # KIS API 인터페이스 라이브러리
│   ├── domain/                 # 공유 도메인 타입 (순환 의존 방지)
│   ├── bot/                    # 매매 알고리즘 + AI
│   └── server/                 # 통합 허브 (Axum + teloxide)
├── web/
│   └── frontend/               # React 대시보드
└── docker-compose.yml
```

**Workspace `Cargo.toml`:**
```toml
[workspace]
members = ["crates/kis_api", "crates/domain", "crates/bot", "crates/server"]
resolver = "2"
```

### 의존 그래프

```
domain      ← 순수 타입, 외부 의존 없음
   ▲
kis_api     ← KIS 서버 통신 전용
   ▲            ▲
bot         ← domain + kis_api (트레이트만 사용)
   ▲
server      ← domain + kis_api (구체 타입으로 생성) + bot
               ├── Axum HTTP/WebSocket
               └── teloxide (Telegram)
```

> **Note:** `server`는 `KisClient` / `KisStream` 구체 타입을 직접 생성하는 유일한 크레이트.
> `bot`은 `KisApi` / `KisEventSource` 트레이트만 의존하여 mock 테스트가 가능.

---

## 3. `domain` Crate

순환 의존을 막기 위한 공유 타입 저장소. 비즈니스 로직 없음.

```rust
// 사용자 → 봇 방향
pub enum BotCommand {
    Start,
    Stop,
    Pause,
    ForceOrder(OrderRequest),
    SetRiskLimit(Decimal),          // Decimal: rust_decimal 크레이트
    QueryStatus,
    // SetStrategy는 StrategyParams 타입 정의 후 추가
}

// 봇 → 외부(웹/메신저) 방향
#[derive(Clone)]
pub enum BotEvent {
    OrderPlaced(OrderResult),
    OrderFilled(FillInfo),
    PositionChanged(Position),
    DailyPnL(PnLReport),
    Alert { level: AlertLevel, msg: String },
    StatusReport(BotStatus),
}

// 공유 값 타입
pub struct Position {
    pub symbol:     String,
    pub qty:        i64,
    pub avg_price:  Decimal,        // f64 사용 금지 — 누적 반올림 오류 방지
}

pub struct OrderRequest {
    pub symbol: String,
    pub side:   Side,
    pub qty:    u64,
    pub price:  Option<Decimal>,    // None = 시장가
}

pub struct PnLReport {
    pub realized:   Decimal,
    pub unrealized: Decimal,
    pub date:       NaiveDate,
}

pub enum AlertLevel { Info, Warning, Critical }
pub enum Side { Buy, Sell }

// 아래 타입은 구조 확정 후 추가 정의 예정
// pub struct OrderResult { ... }   // BotEvent::OrderPlaced, OrderFilled에 사용
// pub struct FillInfo   { ... }    // BotEvent::OrderFilled에 사용
// pub struct BotStatus  { ... }    // BotEvent::StatusReport에 사용
```

> **금액/가격 타입 원칙:** 모든 금액, 가격, 손익은 `rust_decimal::Decimal` 사용.
> `f64`는 표시용(로그 출력 등)에만 허용. 계산에 `f64` 사용 금지.

---

## 4. `kis_api` Crate

### 4.1 Module Structure

```
crates/kis_api/src/
├── lib.rs              # pub use KisClient, KisStream, KisConfig, KisEvent, KisError
│                       #         KisApi, KisEventSource, EventReceiver
├── client.rs           # KisClient (REST)
├── stream.rs           # KisStream (WebSocket)
├── config.rs           # KisConfig + Builder
├── error.rs            # KisError
├── event.rs            # KisEvent enum
├── auth/
│   └── mod.rs          # TokenManager (내부 전용) + current_time() 단일 정의
├── rest/
│   ├── http.rs         # execute_api_call (내부)
│   └── overseas/
│       ├── deposit.rs  # DepositInfo + 호출
│       └── price.rs    # StockPrice + 호출
└── ws/
    ├── connection.rs   # 연결/재연결 루프 + 구독 복구 (내부)
    └── message/
        ├── request.rs
        └── response.rs
```

### 4.2 공개 트레이트

#### `KisApi` — REST 추상화

봇이 구체 타입이 아닌 트레이트에만 의존 → mock 테스트 가능.

```rust
#[async_trait]
pub trait KisApi: Send + Sync {
    async fn check_deposit(&self)                  -> Result<DepositInfo, KisError>;
    async fn current_price(&self, symbol: &str)    -> Result<StockPrice, KisError>;
    async fn place_order(&self, req: OrderRequest) -> Result<OrderResult, KisError>;
}
```

#### `KisEventSource` — WebSocket 추상화

봇이 구체 `KisStream` 없이도 테스트 가능.

```rust
pub trait KisEventSource: Send + Sync {
    fn receiver(&self) -> EventReceiver;
    // subscribe/unsubscribe는 server 레이어 책임 (bot은 이미 구독된 스트림을 받음)
}

impl KisEventSource for KisStream { ... }
// 테스트용: impl KisEventSource for MockStream { ... }
```

### 4.3 `KisClient` (REST)

```rust
#[derive(Clone)]          // Arc<Inner> 내부 — 여러 task에 자유롭게 공유
pub struct KisClient { inner: Arc<KisClientInner> }

impl KisClient {
    pub fn new(config: KisConfig) -> Self;                        // sync, IO 없음
    pub async fn stream(&self) -> Result<KisStream, KisError>;    // WS 핸들 생성
}

impl KisApi for KisClient { ... }
```

**설계 원칙:**
- 전역 상태 완전 제거 (`unsafe static mut`, `lazy_static` 없음)
- 토큰 자동 갱신은 `TokenManager` 내부에서 투명하게 처리
- `KisClient::new()`는 IO 없음 — WS 연결은 `stream()`에서만 시작
- `current_time()` KST 처리 함수는 `auth/mod.rs`에 단일 정의

### 4.4 `KisStream` (WebSocket)

```rust
#[derive(Clone)]
pub struct KisStream { inner: Arc<KisStreamInner> }

impl KisStream {
    pub async fn subscribe(&self, symbol: &str)   -> Result<(), KisError>;
    pub async fn unsubscribe(&self, symbol: &str) -> Result<(), KisError>;
    pub fn receiver(&self) -> EventReceiver;  // 독립 수신 채널
}

impl KisEventSource for KisStream { ... }
```

**내부 이벤트 전달 메커니즘:**

`broadcast::Sender<KisEvent>` (내부) + `EventReceiver` 래퍼 (공개).

```
WS 수신 루프
    │  broadcast::Sender<KisEvent>  (내부, KisStreamInner 소유)
    ├──▶  EventReceiver A  (bot)
    ├──▶  EventReceiver B  (web)
    └──▶  EventReceiver C  (추가 가능)
```

`EventReceiver`는 `broadcast::Receiver`를 직접 노출하지 않고 래핑.
`KisEvent: Clone`이므로 broadcast 가능 (이미 `#[derive(Clone)]` 적용).

```rust
pub struct EventReceiver {
    inner: broadcast::Receiver<KisEvent>,
}

impl EventReceiver {
    /// 이벤트 수신. Lagged(N) 에러 시 N개 이벤트가 유실됐음을 알림.
    pub async fn recv(&mut self) -> Result<KisEvent, KisError>;
    pub fn into_stream(self) -> impl Stream<Item = Result<KisEvent, KisError>>;
}
```

**이벤트 유실 정책:**
- broadcast 버퍼 크기: 기본 1024 (KisConfig에서 설정 가능)
- 수신자가 느려 버퍼를 초과하면 `KisError::Lagged(missed_count)` 반환
- "절대 유실 없음"은 보장하지 않음 — 수신자가 이벤트 처리를 따라가지 못할 경우 유실 가능
- 봇과 웹 모두 `Lagged` 에러를 명시적으로 처리해야 함 (경고 로그 + 재동기화)

**재연결 시 구독 복구:**
- `KisStreamInner`가 구독 목록(`HashSet<String>`)을 관리
- 재연결 후 자동으로 동일 구독 목록 재등록
- 재연결 중 이벤트 유실은 `Disconnected` / `Connected` 이벤트로 소비자에게 알림

### 4.5 `KisEvent`

```rust
#[derive(Debug, Clone)]
pub enum KisEvent {
    Transaction(TransactionData),   // 실시간 체결 (HDFSCNT0)
    Quote(QuoteData),               // 실시간 호가 (HDFSASP0/1)
    OrderConfirm(OrderConfirmData), // 체결 통보 (H0GSCNI0)
    Connected,
    Disconnected,
}

#[derive(Debug, Clone)]
pub struct TransactionData {
    pub symbol:    String,
    pub price:     Decimal,         // rust_decimal::Decimal
    pub volume:    u64,
    pub buy_qty:   u64,
    pub sell_qty:  u64,
    pub timestamp: DateTime<Utc>,
}
```

### 4.6 `KisConfig` — Builder 패턴

```rust
let config = KisConfig::builder()
    .app_key(env::var("APP_KEY")?)
    .app_secret(env::var("APP_SECRET")?)
    .account_num(env::var("ACCOUNT_NUM")?)
    .mock(true)                     // 생략 시 false (실서버) — URL 자동 선택
    .ws_event_buffer(1024)          // 생략 시 1024
    .token_cache_path("~/.trading/kis/access_token")  // 생략 시 기본 경로
    .build()?;
```

**mock 플래그 동작:**
- `mock(true)` → REST URL: `https://openapivts.koreainvestment.com:9443`, WS URL: `ws://ops.koreainvestment.com:31000`
- `mock(false)` → REST URL: `https://openapi.koreainvestment.com:9443`, WS URL: `ws://ops.koreainvestment.com:21000`
- URL을 직접 지정하고 싶으면 `.rest_url(...)` / `.ws_url(...)` 제공

**토큰 캐시:**
- `token_cache_path`로 파일 기반 영속성 설정 가능
- KIS 토큰 유효기간 24시간 — 프로세스 재시작 시 재발급 비용 및 일일 발급 한도 고려
- 캐시 경로 미설정 시 인메모리만 사용 (재시작 시 재발급)

### 4.7 `KisError`

```rust
#[derive(Error, Debug)]
pub enum KisError {
    #[error("auth error: {0}")]
    Auth(AuthError),                                        // 즉시 중단

    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),                        // 재시도 가능

    #[error("KIS API error {code}: {message}")]
    Api { code: String, message: String },                  // KIS 서버 에러 코드

    #[error("websocket error: {0}")]
    WebSocket(String),                                      // 재연결 트리거

    #[error("event stream lagged: {0} events missed")]
    Lagged(u64),                                            // 수신자 처리 지연

    #[error("event stream closed")]
    StreamClosed,                                           // Sender 드롭 — 정상 종료 신호

    #[error("parse error: {0}")]
    Parse(#[from] serde_json::Error),
}

impl KisError {
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Network(_) | Self::WebSocket(_))
    }
}
```

---

## 5. `bot` Crate

- `KisApi` + `KisEventSource` 트레이트에만 의존 → 구체 구현과 무관
- `BotCommand` 수신 채널로 사용자 명령 처리
- `BotEvent` 송신 채널로 외부에 상태 전파
- `StockAnalyzer` 등 분석 로직은 `kis_api`에서 이동하여 여기서 관리
- CPU 집약적 분석은 `tokio::task::spawn_blocking` 사용 (Tokio 런타임 블록 방지)
  - 데이터 병렬 처리가 필요하면 `spawn_blocking` 클로저 안에서 `rayon` 사용 가능
  - `rayon::spawn`은 async와 결과를 연결하는 수단이 없으므로 async 컨텍스트에서 직접 사용 금지

```rust
pub async fn run(
    api:        Arc<dyn KisApi>,
    events:     Arc<dyn KisEventSource>,   // 구체 KisStream 아님
    cmd_rx:     mpsc::Receiver<BotCommand>,
    event_tx:   broadcast::Sender<BotEvent>,
    shutdown:   CancellationToken,          // 종료 신호
) { ... }
```

**테스트 예시:**
```rust
let mock_api    = Arc::new(MockKisApi { price: dec!(130.0) });
let mock_stream = Arc::new(MockEventSource::new(vec![
    KisEvent::Transaction(TransactionData { price: dec!(131.5), .. }),
]));
let result = decide_order(&mock_api, &mock_stream, &signal).await;
assert_eq!(result, Action::Buy);
```

---

## 6. `server` Crate

통합 허브. Axum + teloxide를 동일한 커맨드/이벤트 채널로 연결.
`KisClient` / `KisStream` 구체 타입을 직접 생성하는 유일한 크레이트.

```rust
let shutdown = CancellationToken::new();

// 구체 타입 생성 (server만 알고 있음)
let client = KisClient::new(config);
let stream = client.stream().await?;
stream.subscribe("NVDA").await?;

// 채널 생성
let (cmd_tx, cmd_rx)  = mpsc::channel::<BotCommand>(64);
let (event_tx, _)     = broadcast::channel::<BotEvent>(256);

// 봇 실행
tokio::spawn(bot::run(
    Arc::new(client.clone()),
    Arc::new(stream.clone()),
    cmd_rx,
    event_tx.clone(),
    shutdown.clone(),
));

// 웹 대시보드 (Axum)
tokio::spawn(web::serve(cmd_tx.clone(), event_tx.subscribe(), shutdown.clone()));

// 텔레그램
tokio::spawn(telegram::run(cmd_tx.clone(), event_tx.subscribe(), shutdown.clone()));

// SIGTERM / Ctrl+C → 종료 신호
tokio::signal::ctrl_c().await?;
shutdown.cancel();
```

---

## 7. Telegram 통합

`teloxide` 크레이트 사용.

### 명령어 → `BotCommand`

| 명령어 | BotCommand |
|--------|------------|
| `/start` | `Start` |
| `/stop` | `Stop` |
| `/pause` | `Pause` |
| `/status` | `QueryStatus` |
| `/risk 0.05` | `SetRiskLimit(dec!(0.05))` |
| `/order BUY NVDA 10` | `ForceOrder(...)` |

### `BotEvent` → 텔레그램 메시지 (자동 푸시)

| BotEvent | 메시지 형식 |
|----------|------------|
| `OrderFilled` | `✅ NVDA 50주 체결 @ $134.20` |
| `DailyPnL` | `📊 오늘 손익: +$320.50 (+2.1%)` |
| `Alert(Warning)` | `⚠️ 리스크 한도 근접` |
| `Alert(Critical)` | `🚨 연결 끊김 — 재연결 시도 중` |

### `Notifier` 트레이트 (확장 고려)

```rust
// 내부 뮤터블 상태는 구조체 내부에서 Mutex/RwLock으로 처리
// 트레이트 메서드는 &self 사용 → Arc<dyn Notifier> 가능
#[async_trait]
pub trait Notifier: Send + Sync {
    async fn send(&self, event: &BotEvent) -> Result<(), NotifyError>;
}

// 명령 수신은 별도 핸들 (Clone 가능)
#[async_trait]
pub trait CommandSource: Send + Sync {
    async fn next_command(&self) -> Option<BotCommand>;
}

pub struct TelegramNotifier { ... }  // Notifier + CommandSource 구현
// pub struct DiscordNotifier { ... }  // 추후 확장
```

---

## 8. 종료 처리 (Graceful Shutdown)

24/7 운영 중 재시작(OS 업데이트, Docker 재배포) 시 열린 포지션 보존 및 상태 안전 저장.

```rust
// tokio-util::sync::CancellationToken 사용
// 모든 장기 실행 task에 shutdown 토큰 전달

async fn bot_run(..., shutdown: CancellationToken) {
    loop {
        tokio::select! {
            _ = shutdown.cancelled() => {
                // 1. 진행 중인 주문 완료 대기 (타임아웃 포함)
                // 2. 현재 포지션 파일/DB에 저장
                // 3. 텔레그램으로 종료 알림 전송
                break;
            }
            event = events.recv() => { ... }
        }
    }
}
```

**종료 순서:**
1. SIGTERM / Ctrl+C 수신 → `shutdown.cancel()`
2. 봇: 진행 중 주문 완료 대기 (최대 30초)
3. 봇: 포지션 상태 영속화
4. 텔레그램: 종료 알림 발송
5. Axum / Telegram task 종료
6. 프로세스 exit

---

## 9. 배포 구조

```
개인 서버
└── docker-compose
    ├── trading-server   (Rust binary: server crate)
    │   ├── KIS WebSocket 연결 유지
    │   ├── 봇 tokio task 실행
    │   ├── Axum HTTP/WS 서빙
    │   └── Telegram 봇 폴링
    └── trading-frontend (Nginx + React 정적 파일)
```

`bot`은 별도 프로세스가 아닌 `server` 내부의 tokio task — 단일 바이너리로 24/7 운영.

---

## 10. 현재 `rs_kis` → 신규 구조 마이그레이션

| 기존 파일 | 이동 위치 |
|-----------|----------|
| `src/api/oauth_certification.rs` | `crates/kis_api/src/auth/mod.rs` |
| `src/api/overseas/order_deposit.rs` | `crates/kis_api/src/rest/overseas/deposit.rs` |
| `src/api/overseas/stock_price.rs` | `crates/kis_api/src/rest/overseas/price.rs` |
| `src/api/overseas/subscribe_notification.rs` | `crates/kis_api/src/ws/message/` (내용 검토 후 통합) |
| `src/core/http.rs` | `crates/kis_api/src/rest/http.rs` |
| `src/websockets.rs` | `crates/kis_api/src/ws/connection.rs` |
| `src/types/token.rs` | `crates/kis_api/src/auth/mod.rs` (내부) |
| `src/extentions/analyzer.rs` | `crates/bot/src/analyzer.rs` |
| `src/environment.rs` | **제거** → `KisConfig` builder로 대체 |

**제거 대상:**
- `unsafe static mut INSTANCE` (`environment.rs`)
- `lazy_static! GLOBAL_TX` (`websockets.rs`)
- `GLOBAL_ARRAY` 하드코딩 (`websockets.rs`)
- `current_time()` 중복 정의 (`types/token.rs` + `oauth_certification.rs`) → `auth/mod.rs`에 단일 정의
- `rayon` 의존성 제거 또는 `bot` 크레이트로 이동 (CPU 집약 분석에만 사용)

---

## 11. 핵심 설계 원칙 요약

1. **전역 상태 없음** — 모든 상태는 `KisClient` / `KisStream` 내부 `Arc<Inner>`에
2. **트레이트 추상화** — 봇은 `KisApi` + `KisEventSource` trait에만 의존, mock 테스트 가능
3. **관심사 분리** — KIS 통신 / 알고리즘 / 알림 / 웹이 각자 다른 크레이트
4. **best-effort 이벤트 전달** — broadcast 버퍼 초과 시 `KisError::Lagged`로 명시적 통보, 소비자가 처리 책임
5. **재연결 투명성** — WS 재연결 + 구독 복구는 `kis_api` 내부 책임
6. **금액은 Decimal** — 모든 가격/금액은 `rust_decimal::Decimal`, `f64` 계산 금지
7. **단일 배포 단위** — 하나의 바이너리(`server`)가 봇 + 웹 + 텔레그램 통합 실행
8. **안전한 종료** — `CancellationToken`으로 모든 task 조율, 포지션 상태 영속화
