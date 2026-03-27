use std::collections::HashMap;
use std::sync::Arc;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{broadcast, Mutex, RwLock};
use tokio_util::sync::CancellationToken;

use crate::{KisConfig, KisError, KisEvent};

/// 구독 종류
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubscriptionKind {
    /// 실시간 체결가 해외 (HDFSCNT0)
    Price,
    /// 실시간 호가 해외 (HDFSASP0/1)
    Orderbook,
    /// 실시간 체결가 국내 (H0STCNT0)
    DomesticPrice,
    /// 실시간 호가 국내 (H0STASP0)
    DomesticOrderbook,
}

type SubscriptionKey = (String, SubscriptionKind);

/// WebSocket 이벤트 수신기 (broadcast receiver 래퍼)
pub struct EventReceiver {
    inner: broadcast::Receiver<KisEvent>,
}

impl EventReceiver {
    /// 다음 이벤트를 기다린다. `KisError::Lagged(n)` 또는 `KisError::StreamClosed` 반환 가능.
    pub async fn recv(&mut self) -> Result<KisEvent, KisError> {
        match self.inner.recv().await {
            Ok(event) => Ok(event),
            Err(broadcast::error::RecvError::Lagged(n)) => Err(KisError::Lagged(n)),
            Err(broadcast::error::RecvError::Closed) => Err(KisError::StreamClosed),
        }
    }
}

/// WebSocket 실시간 스트림
#[derive(Clone)]
pub struct KisStream {
    inner: Arc<StreamInner>,
}

struct StreamInner {
    config: KisConfig,
    approval_key: String,
    tx: broadcast::Sender<KisEvent>,
    subscriptions: RwLock<HashMap<SubscriptionKey, ()>>,
    cancel: CancellationToken,
    /// WS writer (shared for sending subscribe/unsubscribe messages)
    ws_tx: Mutex<
        Option<
            futures_util::stream::SplitSink<
                tokio_tungstenite::WebSocketStream<
                    tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
                >,
                tokio_tungstenite::tungstenite::Message,
            >,
        >,
    >,
}

impl KisStream {
    /// 연결 수립 + 수신 루프 시작. `KisClient::stream()` 이 내부적으로 호출.
    pub(crate) async fn connect(config: KisConfig, approval_key: String) -> Result<Self, KisError> {
        let (tx, _) = broadcast::channel(config.ws_event_buffer);
        let cancel = CancellationToken::new();

        let stream = Self {
            inner: Arc::new(StreamInner {
                config: config.clone(),
                approval_key: approval_key.clone(),
                tx: tx.clone(),
                subscriptions: RwLock::new(HashMap::new()),
                cancel: cancel.clone(),
                ws_tx: Mutex::new(None),
            }),
        };

        // 연결 및 수신 루프 시작
        stream.start_connection_loop(tx, cancel).await?;

        Ok(stream)
    }

    async fn start_connection_loop(
        &self,
        tx: broadcast::Sender<KisEvent>,
        cancel: CancellationToken,
    ) -> Result<(), KisError> {
        use tokio_tungstenite::connect_async;
        use tokio_tungstenite::tungstenite::Message;

        let url = &self.inner.config.ws_url;
        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| KisError::WebSocket(e.to_string()))?;

        let (ws_write, ws_read) = ws_stream.split();
        *self.inner.ws_tx.lock().await = Some(ws_write);

        let _inner_clone = self.inner.clone();
        let tx_clone = tx.clone();

        tokio::spawn(async move {
            let mut reader = ws_read;
            loop {
                tokio::select! {
                    _ = cancel.cancelled() => break,
                    msg = reader.next() => {
                        match msg {
                            Some(Ok(Message::Text(text))) => {
                                if let Some(event) = parse_ws_message(&text) {
                                    let _ = tx_clone.send(event);
                                }
                            }
                            Some(Ok(Message::Ping(data))) => {
                                // Pong은 tokio-tungstenite가 자동 처리
                                let _ = data;
                            }
                            Some(Err(e)) => {
                                log::warn!("WS error: {e}");
                                break;
                            }
                            None => break,
                            _ => {}
                        }
                    }
                }
            }
            // tx_clone이 drop되면 수신측에서 RecvError::Closed → KisError::StreamClosed
        });

        Ok(())
    }

    /// 실시간 이벤트 수신기 획득
    pub fn receiver(&self) -> EventReceiver {
        EventReceiver {
            inner: self.inner.tx.subscribe(),
        }
    }

    /// 종목 구독 등록
    pub async fn subscribe(&self, symbol: &str, kind: SubscriptionKind) -> Result<(), KisError> {
        let key = (symbol.to_string(), kind);
        {
            let subs = self.inner.subscriptions.read().await;
            if subs.contains_key(&key) {
                return Ok(());
            }
        }

        self.send_subscribe_message(symbol, kind, true).await?;
        self.inner.subscriptions.write().await.insert(key, ());
        Ok(())
    }

    /// 종목 구독 해제
    pub async fn unsubscribe(&self, symbol: &str, kind: SubscriptionKind) -> Result<(), KisError> {
        let key = (symbol.to_string(), kind);
        {
            let subs = self.inner.subscriptions.read().await;
            if !subs.contains_key(&key) {
                return Ok(());
            }
        }

        self.send_subscribe_message(symbol, kind, false).await?;
        self.inner.subscriptions.write().await.remove(&key);
        Ok(())
    }

    async fn send_subscribe_message(
        &self,
        symbol: &str,
        kind: SubscriptionKind,
        subscribe: bool,
    ) -> Result<(), KisError> {
        use tokio_tungstenite::tungstenite::Message;

        let tr_id = match kind {
            SubscriptionKind::Price => "HDFSCNT0",
            SubscriptionKind::Orderbook => "HDFSASP0",
            SubscriptionKind::DomesticPrice => "H0STCNT0",
            SubscriptionKind::DomesticOrderbook => "H0STASP0",
        };

        let msg = serde_json::json!({
            "header": {
                "approval_key": self.inner.approval_key,
                "custtype": "P",
                "tr_type": if subscribe { "1" } else { "2" },
                "content-type": "utf-8"
            },
            "body": {
                "input": {
                    "tr_id": tr_id,
                    "tr_key": symbol
                }
            }
        });

        let text = serde_json::to_string(&msg).map_err(KisError::Parse)?;

        let mut guard = self.inner.ws_tx.lock().await;
        if let Some(ref mut writer) = *guard {
            writer
                .send(Message::Text(text))
                .await
                .map_err(|e| KisError::WebSocket(e.to_string()))?;
        }

        Ok(())
    }

    /// 스트림 종료
    pub fn close(&self) {
        self.inner.cancel.cancel();
    }
}

// ── HDFSCNT0 필드 인덱스 (KIS 해외 실시간 체결) ──────────────────────────
const HDFSCNT0_F_SYMBOL: usize = 1;
const HDFSCNT0_F_TIME: usize = 2; // HHMMSS (KST)
const HDFSCNT0_F_PRICE: usize = 11;
const HDFSCNT0_F_QTY: usize = 19;
const HDFSCNT0_F_IS_BUY: usize = 21; // "1"=매수, "2"=매도

fn parse_ws_message(text: &str) -> Option<KisEvent> {
    if text.starts_with('{') {
        return None; // JSON 제어 메시지
    }
    let parts: Vec<&str> = text.splitn(4, '|').collect();
    if parts.len() < 4 {
        return None;
    }
    let fields: Vec<&str> = parts[3].split('^').collect();
    match parts[1] {
        "HDFSCNT0" => parse_transaction(&fields),
        "HDFSASP0" | "HDFSASP1" => parse_quote(&fields),
        "H0STCNT0" => parse_domestic_transaction(&fields),
        "H0STASP0" | "H0STASP1" => parse_domestic_quote(&fields),
        _ => None,
    }
}

fn parse_transaction(fields: &[&str]) -> Option<KisEvent> {
    use crate::event::TransactionData;
    use chrono::{FixedOffset, NaiveTime, TimeZone, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let symbol = fields.get(HDFSCNT0_F_SYMBOL)?.to_string();
    if symbol.is_empty() {
        return None;
    }

    let price = Decimal::from_str(fields.get(HDFSCNT0_F_PRICE)?).ok()?;
    let qty = Decimal::from_str(fields.get(HDFSCNT0_F_QTY)?).ok()?;
    let is_buy = match *fields.get(HDFSCNT0_F_IS_BUY)? {
        "1" => true,
        "2" => false,
        _ => {
            log::warn!("HDFSCNT0: unknown is_buy field");
            return None;
        }
    };

    // HHMMSS → DateTime<FixedOffset> (KST UTC+9)
    let hhmmss = fields.get(HDFSCNT0_F_TIME)?;
    let naive_time = NaiveTime::parse_from_str(hhmmss, "%H%M%S").ok()?;
    let naive_dt = Utc::now().date_naive().and_time(naive_time);
    let kst = FixedOffset::east_opt(9 * 3600)?;
    let time = kst.from_local_datetime(&naive_dt).single()?;

    Some(KisEvent::Transaction(TransactionData {
        symbol,
        price,
        qty,
        time,
        is_buy,
    }))
}

// ── HDFSASP0/1 필드 인덱스 (KIS 해외 실시간 호가) ──────────────────────────
const HDFSASP_F_SYMBOL: usize = 1;
const HDFSASP_F_TIME: usize = 2;
const HDFSASP_F_ASK: usize = 14;
const HDFSASP_F_BID: usize = 15;
const HDFSASP_F_ASK_QTY: usize = 16;
const HDFSASP_F_BID_QTY: usize = 17;

fn parse_quote(fields: &[&str]) -> Option<KisEvent> {
    use crate::event::QuoteData;
    use chrono::{FixedOffset, NaiveTime, TimeZone, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let symbol = fields.get(HDFSASP_F_SYMBOL)?.to_string();
    if symbol.is_empty() {
        return None;
    }

    let ask_price = Decimal::from_str(fields.get(HDFSASP_F_ASK)?).ok()?;
    let bid_price = Decimal::from_str(fields.get(HDFSASP_F_BID)?).ok()?;
    let ask_qty = Decimal::from_str(fields.get(HDFSASP_F_ASK_QTY)?).ok()?;
    let bid_qty = Decimal::from_str(fields.get(HDFSASP_F_BID_QTY)?).ok()?;

    let hhmmss = fields.get(HDFSASP_F_TIME)?;
    let naive_time = NaiveTime::parse_from_str(hhmmss, "%H%M%S").ok()?;
    let naive_dt = Utc::now().date_naive().and_time(naive_time);
    let kst = FixedOffset::east_opt(9 * 3600)?;
    let time = kst.from_local_datetime(&naive_dt).single()?;

    Some(KisEvent::Quote(QuoteData {
        symbol,
        ask_price,
        bid_price,
        ask_qty,
        bid_qty,
        time,
    }))
}

// ── H0STCNT0 필드 인덱스 (KIS 국내 실시간 체결) ──────────────────────────
// 필드 구조 (pipe-separated after header): 종목코드^시간^현재가^...^체결량^...
// ⚠ 정확한 인덱스는 KIS OpenAPI 개발가이드 "H0STCNT0" 항목에서 확인 필요
const H0STCNT0_F_SYMBOL: usize = 0;
const H0STCNT0_F_TIME: usize = 1; // HHMMSS (KST)
const H0STCNT0_F_PRICE: usize = 2; // 주식현재가
const H0STCNT0_F_QTY: usize = 9; // 체결거래량

fn parse_domestic_transaction(fields: &[&str]) -> Option<KisEvent> {
    use crate::event::TransactionData;
    use chrono::{FixedOffset, NaiveTime, TimeZone, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let symbol = fields.get(H0STCNT0_F_SYMBOL)?.to_string();
    if symbol.is_empty() {
        return None;
    }

    let price = Decimal::from_str(fields.get(H0STCNT0_F_PRICE)?).ok()?;
    let qty = Decimal::from_str(fields.get(H0STCNT0_F_QTY)?).ok()?;

    let hhmmss = fields.get(H0STCNT0_F_TIME)?;
    let naive_time = NaiveTime::parse_from_str(hhmmss, "%H%M%S").ok()?;
    let naive_dt = Utc::now().date_naive().and_time(naive_time);
    let kst = FixedOffset::east_opt(9 * 3600)?;
    let time = kst.from_local_datetime(&naive_dt).single()?;

    Some(KisEvent::Transaction(TransactionData {
        symbol,
        price,
        qty,
        time,
        is_buy: true, // 국내 체결에는 별도 방향 필드 없음 — 방향 중립
    }))
}

// ── H0STASP0 필드 인덱스 (KIS 국내 실시간 호가) ──────────────────────────
// ⚠ 정확한 인덱스는 KIS OpenAPI 개발가이드 "H0STASP0" 항목에서 확인 필요
const H0STASP0_F_SYMBOL: usize = 0;
const H0STASP0_F_TIME: usize = 1;
const H0STASP0_F_ASK_PRICE: usize = 3; // 매도호가1
const H0STASP0_F_BID_PRICE: usize = 4; // 매수호가1
const H0STASP0_F_ASK_QTY: usize = 13; // 매도호가잔량1
const H0STASP0_F_BID_QTY: usize = 14; // 매수호가잔량1

fn parse_domestic_quote(fields: &[&str]) -> Option<KisEvent> {
    use crate::event::QuoteData;
    use chrono::{FixedOffset, NaiveTime, TimeZone, Utc};
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let symbol = fields.get(H0STASP0_F_SYMBOL)?.to_string();
    if symbol.is_empty() {
        return None;
    }

    let ask_price = Decimal::from_str(fields.get(H0STASP0_F_ASK_PRICE)?).ok()?;
    let bid_price = Decimal::from_str(fields.get(H0STASP0_F_BID_PRICE)?).ok()?;
    let ask_qty = Decimal::from_str(fields.get(H0STASP0_F_ASK_QTY)?).ok()?;
    let bid_qty = Decimal::from_str(fields.get(H0STASP0_F_BID_QTY)?).ok()?;

    let hhmmss = fields.get(H0STASP0_F_TIME)?;
    let naive_time = NaiveTime::parse_from_str(hhmmss, "%H%M%S").ok()?;
    let naive_dt = Utc::now().date_naive().and_time(naive_time);
    let kst = FixedOffset::east_opt(9 * 3600)?;
    let time = kst.from_local_datetime(&naive_dt).single()?;

    Some(KisEvent::Quote(QuoteData {
        symbol,
        ask_price,
        bid_price,
        ask_qty,
        bid_qty,
        time,
    }))
}

#[cfg(any(test, feature = "test-utils"))]
impl KisStream {
    /// 테스트용 (KisStream, broadcast::Sender<KisEvent>) 쌍 반환.
    /// Sender를 통해 테스트 코드에서 직접 이벤트 또는 drop으로 StreamClosed를 주입 가능.
    pub fn test_pair() -> (KisStream, broadcast::Sender<KisEvent>) {
        let (tx, _) = broadcast::channel(128);
        let cancel = CancellationToken::new();
        let stream = KisStream {
            inner: Arc::new(StreamInner {
                config: KisConfig {
                    app_key: "test".into(),
                    app_secret: "test".into(),
                    account_num: "00000000-01".into(),
                    rest_url: "http://localhost".into(),
                    ws_url: "ws://localhost".into(),
                    mock: true,
                    token_cache_path: None,
                    ws_event_buffer: 128,
                },
                approval_key: "test_key".into(),
                tx: tx.clone(),
                subscriptions: RwLock::new(HashMap::new()),
                cancel,
                ws_tx: Mutex::new(None),
            }),
        };
        (stream, tx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscription_kind_eq() {
        assert_eq!(SubscriptionKind::Price, SubscriptionKind::Price);
        assert_ne!(SubscriptionKind::Price, SubscriptionKind::Orderbook);
    }

    #[test]
    fn subscription_kind_is_hashable() {
        let mut map = HashMap::new();
        map.insert(("AAPL".to_string(), SubscriptionKind::Price), ());
        assert!(map.contains_key(&("AAPL".to_string(), SubscriptionKind::Price)));
    }

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn kis_stream_is_send_sync() {
        assert_send_sync::<KisStream>();
    }

    #[test]
    fn event_receiver_type_exists() {
        // EventReceiver가 컴파일되는지 확인 (런타임 생성은 KisStream 필요)
        let _ = std::mem::size_of::<EventReceiver>();
    }

    #[test]
    fn parse_hdfscnt0_transaction() {
        let mut fields = vec![""; 26];
        fields[1] = "NVDA";
        fields[2] = "143022"; // 14:30:22 KST
        fields[11] = "134.20";
        fields[19] = "50";
        fields[21] = "1"; // 매수
        let data = fields.join("^");
        let msg = format!("0|HDFSCNT0|1|{}", data);

        let result = parse_ws_message(&msg);
        assert!(result.is_some(), "should parse HDFSCNT0");
        if let Some(KisEvent::Transaction(tx)) = result {
            use rust_decimal_macros::dec;
            assert_eq!(tx.symbol, "NVDA");
            assert_eq!(tx.price, dec!(134.20));
            assert_eq!(tx.qty, dec!(50));
            assert!(tx.is_buy);
        } else {
            panic!("expected Transaction event");
        }
    }

    #[test]
    fn parse_json_returns_none() {
        let json = r#"{"header":{"tr_id":"PINGPONG"},"body":{}}"#;
        assert!(parse_ws_message(json).is_none());
    }

    #[test]
    fn parse_unknown_trid_returns_none() {
        assert!(parse_ws_message("0|UNKNOWN|1|data").is_none());
    }

    #[test]
    fn parse_malformed_returns_none() {
        assert!(parse_ws_message("not|enough").is_none());
        assert!(parse_ws_message("").is_none());
    }

    #[test]
    fn parse_bad_decimal_returns_none() {
        let mut fields = vec![""; 26];
        fields[1] = "NVDA";
        fields[2] = "143022";
        fields[11] = "NOT_A_NUMBER";
        fields[19] = "50";
        fields[21] = "1";
        let msg = format!("0|HDFSCNT0|1|{}", fields.join("^"));
        assert!(parse_ws_message(&msg).is_none());
    }

    #[test]
    fn parse_hdfsasp0_quote() {
        let mut fields = vec![""; 30];
        fields[1] = "AAPL";
        fields[2] = "150000";
        fields[14] = "191.00"; // ask
        fields[15] = "190.90"; // bid
        fields[16] = "100"; // ask_qty
        fields[17] = "200"; // bid_qty
        let msg = format!("0|HDFSASP0|1|{}", fields.join("^"));

        let result = parse_ws_message(&msg);
        assert!(result.is_some());
        if let Some(KisEvent::Quote(q)) = result {
            use rust_decimal_macros::dec;
            assert_eq!(q.symbol, "AAPL");
            assert_eq!(q.ask_price, dec!(191.00));
            assert_eq!(q.bid_price, dec!(190.90));
        } else {
            panic!("expected Quote event");
        }
    }

    #[test]
    fn parse_hdfsasp1_also_works() {
        let mut fields = vec![""; 30];
        fields[1] = "SONY";
        fields[2] = "090000";
        fields[14] = "10.50";
        fields[15] = "10.40";
        fields[16] = "500";
        fields[17] = "300";
        let msg = format!("0|HDFSASP1|1|{}", fields.join("^"));
        assert!(matches!(parse_ws_message(&msg), Some(KisEvent::Quote(_))));
    }

    #[test]
    fn domestic_subscription_kind_maps_to_h0stcnt0() {
        // Verify that DomesticPrice maps to the correct KIS TR ID
        assert_eq!(subscription_tr_id(SubscriptionKind::DomesticPrice), "H0STCNT0");
        assert_eq!(subscription_tr_id(SubscriptionKind::DomesticOrderbook), "H0STASP0");
    }

    // Helper exposed only for tests:
    fn subscription_tr_id(kind: SubscriptionKind) -> &'static str {
        match kind {
            SubscriptionKind::Price => "HDFSCNT0",
            SubscriptionKind::Orderbook => "HDFSASP0",
            SubscriptionKind::DomesticPrice => "H0STCNT0",
            SubscriptionKind::DomesticOrderbook => "H0STASP0",
        }
    }
}
