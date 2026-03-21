# rs_kis

[![fmt](https://github.com/soki1229/rs_kis/actions/workflows/fmt.yml/badge.svg)](https://github.com/soki1229/rs_kis/actions/workflows/fmt.yml)
[![lint](https://github.com/soki1229/rs_kis/actions/workflows/lint.yml/badge.svg)](https://github.com/soki1229/rs_kis/actions/workflows/lint.yml)
[![test](https://github.com/soki1229/rs_kis/actions/workflows/test.yml/badge.svg)](https://github.com/soki1229/rs_kis/actions/workflows/test.yml)

KIS(한국투자증권) OpenAPI Rust 비공식 라이브러리. Cargo workspace 기반으로 구성되어 있으며, `crates/kis_api`가 핵심 라이브러리 크레이트다.

**주요 기능:**
- 해외주식 주문/취소 (나스닥, NYSE, 아멕스, 도쿄, 홍콩, 상해, 심천, 하노이, 호치민)
- 계좌 잔고, 미체결, 체결내역, 기간손익
- 현재가, 호가, 차트 (일봉/분봉)
- 종목 검색, 뉴스, 배당, 국가별 휴장일
- 시세분석: 등락률/거래량/시총 순위, 체결강도, 신고/신저가
- WebSocket 실시간 체결가 · 호가

> **Note:** 이 크레이트는 crates.io에 게재되지 않습니다. `path` 또는 `git` 의존성으로만 사용 가능합니다.
> ```toml
> [dependencies]
> kis_api = { path = "crates/kis_api" }
> ```

---

## Setup

`.env.example`을 복사해 `.env`로 저장하고 값을 채운다:

```bash
cp .env.example .env
```

최소 필수 항목 3개:

```
APP_KEY=...
APP_SECRET=...
ACCOUNT_NUM=XXXXXXXX-XX
```

나머지 변수(`REST_URL`, `WS_URL`, `TOKEN_CACHE_PATH` 등)와 VTS 모의투자 설정은 `.env.example` 참고.

---

## Usage

### 주문 / 계좌

```rust
use kis_api::{
    KisClient, KisConfig, Exchange, OrderSide, OrderType, PlaceOrderRequest,
};
use rust_decimal_macros::dec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = KisConfig::from_env()?;
    let client = KisClient::new(config);

    // 나스닥 AAPL 1주 시장가 매수
    let order = client.place_order(PlaceOrderRequest {
        symbol: "AAPL".to_string(),
        exchange: Exchange::NASD,
        side: OrderSide::Buy,
        order_type: OrderType::Market,
        qty: dec!(1),
        price: None,
    }).await?;
    println!("주문번호: {}", order.order_org_no);

    // 계좌 잔고 조회
    let balance = client.balance().await?;
    println!("평가손익: {}", balance.summary.total_pnl);

    Ok(())
}
```

### 시세

```rust
use kis_api::{KisClient, KisConfig, Exchange, DailyChartRequest, ChartPeriod};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = KisConfig::from_env()?;
    let client = KisClient::new(config);

    // 현재가
    let price = client.price("AAPL", &Exchange::NASD).await?;
    println!("현재가: {} ({}%)", price.last, price.rate);

    // 일봉 20개
    let bars = client.daily_chart(DailyChartRequest {
        symbol: "AAPL".to_string(),
        exchange: Exchange::NASD,
        period: ChartPeriod::Daily,
        adj_price: true,
    }).await?;
    for bar in bars.iter().take(3) {
        println!("{}: O={} H={} L={} C={}", bar.date, bar.open, bar.high, bar.low, bar.close);
    }

    Ok(())
}
```

### 시세분석

```rust
use kis_api::{KisClient, KisConfig, Exchange, RankingRequest, RankingSort};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = KisConfig::from_env()?;
    let client = KisClient::new(config);

    // 나스닥 등락률 상위 10개
    // sort: ChangeRate(등락률) / Volume(거래량) / MarketCap(시총)
    let ranking = client.price_ranking(RankingRequest {
        exchange: Exchange::NASD,
        sort: RankingSort::ChangeRate,
        count: 10,
    }).await?;
    for item in &ranking {
        println!("{} {}: {}%", item.exchange, item.symbol, item.rate);
    }

    Ok(())
}
```

### WebSocket 실시간

```rust
use kis_api::{KisClient, KisConfig, KisApi, KisEvent, KisError, SubscriptionKind};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = KisConfig::from_env()?;
    let client = KisClient::new(config);

    // stream()은 KisApi trait method — use kis_api::KisApi 필요
    let stream = client.stream().await?;
    stream.subscribe("AAPL", SubscriptionKind::Price).await?;

    let mut rx = stream.receiver();
    loop {
        match rx.recv().await {
            Ok(KisEvent::Transaction(d)) => println!("체결: {} @ {}", d.symbol, d.price),
            Ok(KisEvent::Quote(d))       => println!("호가: {} ask={}", d.symbol, d.ask_price),
            Err(KisError::StreamClosed)  => break,
            Err(KisError::Lagged(n))     => eprintln!("{}개 이벤트 유실", n),
            _ => {}
        }
    }
    Ok(())
}
```

→ 전체 API 목록 및 상세 설명: [docs/usage.md](docs/usage.md)

---

## Disclaimer

이 라이브러리는 한국투자증권(KIS)의 공식 프로젝트가 아닙니다. 개인이 제작한 비공식 클라이언트이며, 실제 거래에 사용 시 발생하는 모든 손실에 대한 책임은 사용자 본인에게 있습니다.
