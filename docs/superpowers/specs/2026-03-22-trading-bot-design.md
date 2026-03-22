# trading-bot 설계 스펙

## 목표

`trading-bot`은 KIS OpenAPI 기반의 해외주식 자동매매 시스템이다.
단순히 "작동하는 봇"이 아니라 **전문 트레이더 관점에서 오래 살아남는 봇**을 목표로 한다.
신호 생성보다 실행 품질, 리스크 정의, 예외 처리, 상태 복구를 우선으로 설계한다.

---

## 레포 구조

| 레포 | 공개 여부 | 내용 |
|------|----------|------|
| `rs_kis` | public | `kis_api` + `domain` 라이브러리만 |
| `trading-bot` | **private** | 봇 알고리즘 (`kis_api`를 git dependency로 참조) |
| `trading-server` | **private** | Axum API + Telegram 통합 (`bot` + `kis_api` 참조) |

`trading-bot`의 `Cargo.toml`:
```toml
[dependencies]
kis_api = { git = "https://github.com/soki1229/rs_kis" }
```

---

## 파이프라인 아키텍처

```
Market Regime Filter
        ↓
Universe Discovery
        ↓
Entry Qualification
  ├── HARD BLOCK (이벤트 금지) → 종료
  └── Setup Score (0~100)
        ├── < 60 → 탈락
        ├── 60~79 → RuleEngine 단독 판정
        └── ≥ 80 → RuleEngine + LLM sanity check
                ↓
Risk Sizing (ATR 기반 수량 계산 + RiskGuard)
        ↓
Execution (Order State Machine + broker_order_id 매핑)
        ↓
Position Management (ATR 기반 익절 + 레짐별 Trailing)
        ↑
Session Risk Control (횡단 관심사, 전 단계 감시)
```

각 단계는 별도 tokio task로 실행되며 채널(mpsc/broadcast)로 통신한다.
`CancellationToken`으로 전체를 동기화하여 graceful shutdown 보장.

### 제어 규칙 우선순위

여러 조건이 동시에 발생하면 아래 순서로 강한 쪽이 우선 적용된다.

| 우선순위 | 조건 | 행동 |
|---------|------|------|
| 1 (최강) | `HARD` Kill Switch | 신규 주문 차단 + 전 포지션 시장가 청산 |
| 2 | `SOFT` Kill Switch | 신규 주문 차단, 포지션 유지 |
| 3 | 일일 손실 한도 초과 | 신규 진입 전면 차단, 포지션 관리만 |
| 4 | 연속 손실 한도 도달 | 봇 자동 중단 (수동 재개) |
| 5 | 프로파일 자동 전환 | Conservative 파라미터 적용 (진입 가능, 더 엄격한 조건) |
| 6 | 레짐 제한 (`Volatile`/`Quiet`) | 사이즈 축소 또는 진입 중단 |
| 7 (최약) | 일반 진입 조건 | Setup Score / Rule strength / LLM verdict 기준 적용 |

상위 조건이 발동 중일 때 하위 조건의 신호는 계산은 하되 실행은 하지 않는다.

### 봇 운영 상태 전이

```
              [RecoveryCheck]
              ↙            ↘
           pass             fail
            ↓                ↓
         [Active] ←──────[HardBlocked]
         /   |  \               ↑
        /    |   \        HARD KS trigger
       ↓     ↓    ↓       (10s recheck)
[EntryPaused] [SoftBlocked] [Suspended]
  daily_loss   SOFT KS       consecutive_loss
  / regime     trigger       limit reached
       ↓           ↓              ↓
  next day /    kill switch    manual reset
  regime change  file deleted
       ↓           ↓              ↓
            [Active]
```

| 상태 | 진입 조건 | 탈출 조건 |
|------|----------|----------|
| `Active` | 재시작 복구 성공, 수동 재개 | — |
| `EntryPaused` | 일일 손실 한도, Quiet 레짐 | 다음 거래일 / 레짐 변경 |
| `SoftBlocked` | SOFT Kill Switch 발동 | kill switch 파일 삭제 + 복구 성공 |
| `HardBlocked` | HARD Kill Switch 발동 (전 포지션 청산 완료) | 수동 개입 + kill switch 파일 삭제 + 복구 성공 |
| `Suspended` | 연속 손실 한도 도달 | `session_stats` 연속 손실 수동 초기화 + 복구 성공 |
| `RecoveryCheck` | 모든 재시작 직후 | 복구 절차 완료 |

---

## 1. Market Regime Filter

**목적**: 장세 구분 → 전략 활성/관망/중단 결정.

### 입력
- 나스닥/S&P500 지수 현재가 + 일봉 20일 (KIS API)
- 거래량 평균 대비 비율 (시장 전체)

### 판정 로직

| 레짐 | 조건 | 봇 행동 |
|------|------|---------|
| `Trending` | 지수 5일선 > 20일선, 거래량 정상 | 진입 허용 |
| `Volatile` | 당일 지수 등락률 ±1.5% 이상 | 포지션 사이즈 50% 축소, 신규 진입 제한 |
| `Quiet` | 지수 등락률 ±0.3% 미만, 거래량 저조 | 신규 진입 중단. Watchlist 유지 및 Setup Score 갱신 계속 실행 (다음 레짐 전환 시 즉시 분석 가능하도록) |

### 갱신 주기
- 장 시작 시 1회 + 이후 1시간마다 갱신

---

## 2. Universe Discovery

**목적**: 오늘 주목할 종목 풀 추출 (후보 선정, 진입 결정 아님).

### 소스
- KIS API `volume_surge` — 거래량 평균 대비 3배 이상
- KIS API `price_ranking` — 등락률 상위/하위
- Finnhub 뉴스 웹훅 — 뉴스 발생 종목 실시간 추가

### 필터
- 시가총액 $1B 이상 (소형주 제외)
- 유동성 필터: 일평균 거래량 100만 주 이상
- 중복 제거 후 풀 최대 30종목 유지

### 갱신
- 이벤트 드리븐 (KIS API 폴링 15분 간격 + Finnhub 실시간)

### Watchlist TTL

Discovery 풀에 추가된 종목은 **2 거래일** 후 자동 만료된다.
만료된 종목은 재발굴 조건(거래량/등락/뉴스)을 다시 충족해야 재진입한다.
단, 실적 발표 전후 이벤트 추적 종목은 해당 이벤트가 끝날 때까지 TTL 연장.
만료 시 `signal_log`에 만료 사유(`2_day_elapsed` / `event_window_ended`)와 종목을 기록한다.

### Watchlist 역할

Discovery 풀은 레짐에 관계없이 항상 갱신된다. 다만 레짐별로 역할이 다르다:

| 레짐 | Watchlist 역할 |
|------|---------------|
| `Trending` | Entry Qualification 진입 대상 (진입 시도) |
| `Volatile` | Entry Qualification 진입 대상 (사이즈 축소 후 진입 시도) |
| `Quiet` | **관찰 전용** — Setup Score 갱신 + 뉴스 추적만. 진입 없음. 레짐 전환 시 우선 평가 대상 |

---

## 3. Entry Qualification + Setup Score

**목적**: 좋은 종목(후보)과 지금 들어갈 타이밍을 분리.
조건을 "전부 충족해야 하는 AND 체인"이 아닌 **필수 차단 + 점수 누적** 방식으로 설계해 과잉 필터링과 늦은 진입을 방지한다.

### 3-1. 이벤트 금지 필터 (HARD BLOCK — 필수)

아래 조건 중 하나라도 해당하면 해당 종목을 즉시 차단한다. Setup Score 계산 없이 종료.

| 조건 | 설명 |
|------|------|
| 실적 발표 ±1일 | 예측 불가 변동성 |
| FOMC / CPI 발표일 | 거시 이벤트 |
| 장 시작 후 `entry_blackout_open_mins`분 이내 | 초고변동 구간 |
| 장 마감 `entry_blackout_close_mins`분 이전 | 유동성 저하 구간 |
| 당일 등락률 > +10% 또는 < -3% | 과열/과매도 제외 |

이벤트 캘린더: Finnhub `earnings_calendar` API로 매일 장 시작 전 갱신.

### 3-2. Setup Score (가산 최대 100점, 감점으로 음수 가능)

HARD BLOCK을 통과한 종목에 대해 점수를 누적 산출한다.
각 조건은 독립적이며, 충족하지 못해도 다른 조건으로 보완 가능하다.

**가산 조건:**

| 조건 | 점수 |
|------|------|
| 5일 이동평균 > 20일 이동평균 | +20 |
| 거래량 비율 ≥ 2x (평균 대비) | +20 |
| 분봉 기준 최근 5분 거래량 > 이전 5분의 1.5배 | +15 |
| 호가 불균형: 매수잔량/매도잔량 ≥ 1.3 | +20 |
| 최근 10분 내 신고가 돌파 | +15 |
| 뉴스 촉매 존재 (Finnhub, 24h 이내) | +10 |

**감점 조건:**

| 조건 | 점수 |
|------|------|
| 당일 등락률 > +7% (과열 경고) | -15 |
| 장 마감 `entry_blackout_close_mins` × 2분 이내 (유동성 저하 진입) | -10 |
| Market Regime `Volatile` (레짐 불안정) | -10 |

최종 Setup Score = 가산 합계 - 감점 합계 (음수 가능, 실제 의사결정 임계값은 60/80 유지)

**의사결정 기준** (`[signal] setup_score_threshold_entry/llm` 설정값):

| Setup Score | 행동 |
|-------------|------|
| < `setup_score_threshold_entry` (기본 60) | 탈락 — Signal Engine 진입 없음 |
| 60 ~ `setup_score_threshold_llm` - 1 (기본 79) | RuleEngine만으로 판정 (LLM 없음) |
| ≥ `setup_score_threshold_llm` (기본 80) | RuleEngine + LLM sanity check |

**핵심 원칙**: LLM은 고품질 후보(Setup Score ≥ 80)에게만 호출한다. 나머지는 규칙만으로 판정해 비용을 통제하고 LLM 과의존을 방지한다.

**Setup Score와 RuleEngine strength 피처 분리 원칙:**
두 점수는 서로 다른 정보를 반영해야 한다. 동일 데이터를 양쪽에 중복 반영하면 신호가 과대평가된다.

| 점수 | 역할 | 반영 데이터 |
|------|------|------------|
| Setup Score | **정적 스냅샷** — 현재 종목 상태의 품질 | MA 정렬, 거래량 절대비율(level), 호가잔량, 신고가, 뉴스 |
| Rule strength | **동적 모멘텀** — 지금 이 순간 진입 타이밍 품질 | 분봉 방향 일관성, ATR 대비 가격 위치, 거래량 가속도(acceleration) |

거래량 관련: Setup Score는 "평균 대비 비율(level)"을, Rule strength는 "구간별 가속도 추이(rate of change)"를 반영. 동일 데이터의 다른 차원이므로 허용되지만, 구현 시 같은 raw 수치를 동일 계산식으로 재사용하지 않도록 주의한다.

---

## 4. Signal Engine

### 4-1. RuleEngine

Setup Score를 통과한 종목에 대해 기술적 점수를 재확인한다.
`RuleSignal { direction: Direction, strength: f64 }` 반환.

`strength`는 Setup Score와 독립적인 시계열 추세 강도 평가:
- 최근 분봉 방향성 일관성
- ATR 대비 현재가 위치 (전일 종가에서 얼마나 벗어났는가)
- 거래량 가속도 (최근 3구간 평균 대비)

**진입 조건**:

| Setup Score | RuleSignal strength | LLM | 결과 |
|-------------|--------------------|----|------|
| ≥ 80 | ≥ 0.55 | ENTER 또는 WATCH | 진입 / 관망 |
| ≥ 80 | ≥ 0.55 | BLOCK | 금지 |
| 60~79 | ≥ 0.65 | — (호출 안 함) | 진입 |
| 60~79 | < 0.65 | — | 금지 |
| < 60 | — | — | 금지 (여기까지 오지 않음) |

### 4-2. LlmEngine (최종 sanity check — 고품질 후보 한정)

**역할**: Setup Score ≥ 80인 종목에 대해서만 호출. 가중 신호가 아닌 **ENTER / WATCH / BLOCK** 판정.
LLM이 ENTER를 뒤집어 BLOCK할 수 있지만, BLOCK을 뒤집어 ENTER할 수는 없다.

**입력 (단일 구조화 프롬프트)**:
```
종목: {symbol} | 현재가: {price} | 등락률: {change_pct}%
거래량 비율: {volume_ratio}x | MA 상태: {ma_state} | Setup Score: {score}/100
최근 뉴스 (24h, 최대 5건):
  1. {headline} ({source}, {published_at})
  ...

세 관점에서 각각 판단하라:
- Bull: 매수 근거 (1~2문장)
- Bear: 위험 요인 (1~2문장)
- Neutral: 중립 평가 (1문장)

최종 판정: ENTER / WATCH / BLOCK
BLOCK 사유 (BLOCK인 경우만): {reason}
```

**응답 (JSON)**:
```json
{
  "verdict": "ENTER",
  "bull": "...",
  "bear": "...",
  "neutral": "...",
  "block_reason": null
}
```

**모델**: Claude Haiku (`claude-haiku-4-5-20251001`)
**비용 추정**: LLM은 Score ≥ 80 후보에만 → 하루 2~5회 → 월 $0.5~1.5

### 4-3. LLM 장애 처리
- LLM 타임아웃/에러 → `LlmUnavailable` 상태 전환
- `LlmUnavailable` 시: `setup_score_threshold_llm` → 999 (LLM 경로 전면 비활성) + 포지션 사이즈 50% 축소
- 60~79 경로(RuleEngine 단독)는 정상 동작 유지 (`fallback_rule_strength = 0.70`으로 임계값 상향)
- 로그: `"LLM unavailable. Score-80 path disabled, rule-only threshold raised to 0.70, size halved."`
- LLM 복구 감지 후 자동 복원

---

## 5. Risk Sizing

**목적**: 포지션 크기를 "계좌의 몇 %" 기준이 아닌 **허용 손실액 역산**으로 계산.

### ATR 기반 손절폭 + RuleEngine strength 연동 사이즈

```
기본 수량       = (계좌 잔고 × risk_per_trade) / (ATR_14 × atr_stop_multiplier)
최종 수량       = 기본 수량 × strength_size_factor(strength)
손절가          = 진입가 - (ATR_14 × atr_stop_multiplier)
```

`strength_size_factor`: RuleEngine strength 값에 따라 포지션 크기를 선형 보정.
같은 진입이라도 신호 강도가 높을수록 더 많이 베팅하고, 경계선 신호는 더 적게.

| RuleSignal strength | 사이즈 배율 |
|--------------------|------------|
| 0.55 ~ 0.64 | 0.75× |
| 0.65 ~ 0.74 | 1.00× (기준) |
| 0.75 ~ 0.84 | 1.15× |
| 0.85 ~ 1.00 | 1.25× |

최대 배율은 RiskGuard의 `max_position_pct` 한도 내에서만 적용된다.

### 최종 사이즈 계산식 (모든 보정 통합)

```
base_size    = (account_balance × risk_per_trade) / (ATR_14 × atr_stop_multiplier)
final_size   = base_size × strength_factor × regime_factor × profile_factor
capped_size  = min(final_size, account_balance × max_position_pct / entry_price)
```

| 보정 인자 | 기준 |
|----------|------|
| `strength_factor` | 0.75× ~ 1.25× (RuleEngine strength 구간별, 위 테이블 참조) |
| `regime_factor` | Trending = 1.0, Volatile = 0.5, Quiet = N/A (진입 없음) |
| `profile_factor` | default = 1.0, conservative = 0.6, aggressive = 1.6 |

보정이 중첩될 때 (예: Volatile 레짐 + Conservative 프로파일) 최소값이 아닌 **곱셈** 적용.
예: `1.0 × 0.5 × 0.6 = 0.30×` → 기본 사이즈의 30%.

- `risk_per_trade`: `[risk] risk_per_trade` 설정값 (Section 12)
- `atr_stop_multiplier`: `[risk] atr_stop_multiplier` 설정값 (Section 12)

예시 (`atr_stop_multiplier = 1.5`, `risk_per_trade = 0.005`, `strength = 0.80`):
- 계좌 $10,000 → 기본 허용 손실 $50
- NVDA 진입가 $892, ATR_14 = $18 → 손절가 = $865
- 기본 수량 = $50 / $27 = 1.85주 → 최종 = 1.85 × 1.15 = 2.13주

### 한도 검사 (RiskGuard)

| 한도 | 설정 키 | 위반 시 행동 |
|------|---------|------------|
| 종목당 최대 계좌 비중 | `max_position_pct` | 수량 상한 조정 |
| 동시 보유 종목 수 | `max_open_positions` | 진입 차단 (로그 기록, 큐잉 없음) |
| 동일 섹터 총 익스포저 | `max_sector_exposure` | 진입 차단 (로그 기록) |
| 당일 손실 한도 | `daily_loss_limit` | 신규 진입 전면 차단 |
| 연속 손실 횟수 | `consecutive_loss_limit` | 3회 **도달 시** 봇 자동 중단 (수동 재개) |

`max_open_positions` 위반은 사이즈 조정이 아닌 **진입 차단**만 적용된다 (개수 한도이므로 수량 조정 불가).

---

## 6. Execution (Order State Machine)

### 주문 방식
시장가/지정가 대신 **Marketable Limit Order** 사용:
- 매수: `현재 매도호가 + $0.01` (즉시 체결 가능하지만 슬리피지 제한)
- 미체결 60초 경과 시: 호가 재조회 후 재호가 (최대 3회)
- 3회 재호가 후 미체결 → 주문 취소 + 진입 포기

### 주문 상태머신

```
PENDING_SUBMIT
    → SUBMITTED
        → PARTIALLY_FILLED
            → FULLY_FILLED
            → CANCELLED_PARTIAL   ← 재호가 3회 소진 또는 세션 종료 시
        → FULLY_FILLED
        → CANCELLED               ← 재호가 3회 소진 (체결량 0)
        → FAILED
```

**`CANCELLED_PARTIAL` 처리**: 이미 체결된 수량은 포지션으로 유지한다.
미체결 잔여 수량만 포기하고 `positions` 테이블에 실제 체결 수량으로 기록.

WebSocket 체결 이벤트를 1차로 처리하되, 30초 간격으로 KIS REST `unfilled_orders()` 폴링으로 상태 검증 (WebSocket 지연/누락 대비).

### 중복 주문 방지
주문 제출 전 `pending_order_map`에 `(symbol, side)` 키 등록.
동일 키가 존재하면 주문 차단 (재시작 후 복구 시에도 확인).

### 브로커 주문번호 매핑
모든 주문 제출 후 KIS가 반환하는 `broker_order_id`를 내부 `order_id`와 함께 `orders` 테이블에 저장.
재시작 복구 시 `unfilled_orders()` 응답의 `broker_order_id`로 내부 상태를 조회한다.
`broker_order_id` 없이는 주문 상태 추적이 불가능하므로 제출 직후 저장이 필수다.

---

## 7. Position Management

### ATR 기반 익절 목표

고정 % 익절 대신 **ATR 배수**를 사용해 종목 변동성에 비례한 현실적인 목표를 설정한다.

```
1차 목표가 = 진입가 + (ATR_14 × profit_target_1_atr)   # 기본: 2x
2차 목표가 = 진입가 + (ATR_14 × profit_target_2_atr)   # 기본: 4x
```

| 조건 | 행동 |
|------|------|
| 현재가 ≥ 1차 목표가 | 수량의 50% 부분 익절 |
| 현재가 ≥ 2차 목표가 | 잔여 수량 전량 익절 |
| 손절가 도달 | **시장가** 전량 청산 (가격보다 탈출이 우선) |
| 보유 4시간 경과 (시간 손절) | 포지션 재평가 → BLOCK이면 청산 |
| 장 마감 `eod_liquidate_mins`분 전 | 전체 포지션 청산 (익일 갭 리스크 제거) |

- `profit_target_1_atr`, `profit_target_2_atr`: `[position]` 설정값 (Section 12)
- ATR_14: 진입 시점 기준 (이후 갱신 없음)

### Trailing Stop (레짐별)

1차 목표가 도달 후 잔여 수량에 적용. 레짐에 따라 폭을 다르게 설정한다:

| 레짐 | Trailing Stop 기준 | 설명 |
|------|-------------------|------|
| `Trending` | 최고가 - (ATR_14 × 2.0) | 넓게 — 추세를 충분히 타도록 |
| `Volatile` | 최고가 - (ATR_14 × 1.0) | 좁게 — 빠른 이익 보호 |
| `Quiet` | 시간 손절만 적용 (Trailing 없음) | 의미있는 움직임 없으므로 생략 |

trailing stop 계산에 사용하는 ATR_14는 **레짐 변경이 감지된 직후 다음 1분봉 종가 기준**으로 재산정한다.
진행 중인 캔들이 아닌 완성된 캔들 종가를 사용해 노이즈를 방지한다.
재산정된 ATR_14는 해당 포지션의 기존 trailing stop을 즉시 갱신한다.

**Trailing stop 갱신 원자성**: 레짐 변경 시 ATR 재산정 → trailing stop 신규 산출 → 기존 stop 주문 취소 → 신규 stop 주문 제출의 4단계는 **단일 트랜잭션처럼** 처리한다. 구체적으로, 기존 stop 주문 취소 성공을 확인한 뒤에만 신규 stop 주문을 제출한다. 취소 실패 시 ATR 재산정을 롤백하고 기존 stop을 유지한다. 이 과정에서 포지션이 stop 없는 상태로 노출되는 구간(mid-flight window)을 최소화한다.

**원자 단위 경계 (구현 지침)**: 하나의 `update_trailing_stop(position_id)` 함수가 4단계를 순서대로 실행하고 중간 실패 시 롤백한다. DB `positions` 테이블의 `trailing_stop_price` 컬럼은 신규 stop 주문 제출 성공 확인 후에만 갱신한다. 취소와 제출 사이에 포지션이 체결될 경우(`FULLY_FILLED` 이미 처리됨), 신규 stop 제출을 건너뛴다.

---

## 8. Session Risk Control

전 단계를 감시하는 횡단 관심사. 별도 task로 실행.

| 한도 | 조건 | 행동 |
|------|------|------|
| 일일 손실 `daily_loss_limit` | 초과 시 | 신규 매수 전면 차단, 보유 포지션만 관리 |
| 연속 손실 `consecutive_loss_limit`회 | **도달 시** | 봇 자동 중단 + 알림 (수동 재개 필요) |
| Market Regime `Volatile` | 진입 시 | 포지션 사이즈 50% 제한 + **신규 진입 제한** |
| Kill Switch 조건 | 아래 참조 | 전체 신규 주문 차단 + 알림 |

> **연속 손실 기준**: `consecutive_loss_limit = 3`이면 3회째 손실이 확정되는 시점에 즉시 중단. "초과(4회)"가 아닌 "도달(3회)"이 트리거다.

### Kill Switch 발동 조건
다음 중 하나라도 발생하면 신규 주문 즉시 차단, 수동 확인 요구:

- KIS API 에러 3회 연속 (5분 이내)
- 잔고/포지션이 내부 상태와 불일치 (±5% 이상)
- 미체결 주문 5개 이상 **또는** 미체결 주문 총액이 계좌 잔고의 30% 초과 (개수 + 비중 병행)
- WebSocket 재연결 실패 3회 연속

Kill Switch는 발동 강도에 따라 두 가지 모드로 분리한다:

| 모드 | 조건 | 행동 |
|------|------|------|
| `SOFT` | KIS API 에러 연속, WebSocket 재연결 실패 | 신규 주문 차단. 기존 포지션 유지. |
| `HARD` | 잔고 불일치, 미체결 비중 30% 초과 | 신규 주문 차단 + 기존 포지션 **시장가 전량 청산** |

`HARD` 모드는 계좌 상태를 신뢰할 수 없을 때만 발동한다. 정상 장세에서는 `SOFT`를 우선 적용한다.

**HARD 청산 전 1회 재확인 절차**: 잔고/포지션 불일치가 감지되면 즉시 청산하지 않고 10초 대기 후 재조회한다.
- 불일치가 해소됐으면 → `SOFT` Kill Switch (API 동기화 지연이었음)
- 불일치가 유지되면 → `HARD` Kill Switch 발동 (실제 이상 확인됨)

이 절차는 KIS API 응답 지연으로 인한 오탐을 방지한다. 미체결 비중 30% 초과는 재확인 없이 즉시 `HARD` 발동한다 (포지션 붕괴 위험이 명확하므로).

Kill Switch 상태는 `[state] kill_switch_path` 경로에 파일 생성으로 영속화 (Section 12 참조).
파일 내용에는 발동 원인과 모드를 JSON으로 기록한다:
```json
{
  "mode": "SOFT",
  "reason": "KIS API 3 consecutive errors within 5 min",
  "triggered_at": "2026-03-22T14:23:45+09:00",
  "details": "HTTP 503 × 3: /uapi/overseas-stock/v1/trading/order"
}
```
재시작 시 파일 존재하면 수동으로 삭제하기 전까지 주문 차단 유지.
원인 없는 Kill Switch는 현장에서 디버깅 지옥이 되므로 파일 내용은 필수다.

---

## 9. 상태 영속화 & 복구

### 영속화 항목
SQLite 파일 (`~/.local/share/trading_bot/state.db`)에 저장:

| 테이블 | 내용 |
|--------|------|
| `orders` | 주문 ID, **브로커 주문번호(broker_order_id)**, 상태, 심볼, 수량, 가격, 타임스탬프 |
| `positions` | 현재 포지션, 매수가, 손절가, ATR (진입 시점 ATR_14 고정값 — 손절가 산출 기준, 이후 갱신 없음) |
| `session_stats` | 당일 손익, 연속 손실 횟수 |
| `signal_log` | 모든 신호 판정 결과 (Rule + LLM 포함) |
| `strategy_stats` | Setup Score 구간별 승률, LLM verdict별 R, 레짐별 R, 일별 드로다운 (Section 15 지표) |

### 재시작 복구 절차 (브로커 기준 정규화)

재시작 시 내부 DB 상태가 아닌 **브로커(KIS) 상태를 정본**으로 취급해 정규화한다.

1. `balance()` + `unfilled_orders()` + `order_history(today)` 조회
2. DB `orders` 테이블의 `broker_order_id`와 브로커 응답을 매핑
3. 브로커에 존재하지 않는 `SUBMITTED` 상태 주문 → `CANCELLED`로 강제 전이
4. 브로커에서 체결 완료됐으나 DB가 `SUBMITTED`인 주문 → `FULLY_FILLED`로 갱신 (WS 이벤트 누락 복구)
5. `positions` 테이블과 브로커 `balance()` 비교 → ±5% 초과 불일치 시 Kill Switch 발동
6. 일치 시 정상 기동

**RecoveryCheck 실패 원인 분류** (Kill Switch 발동 시 reason 필드에 기록):

| 코드 | 설명 |
|------|------|
| `BALANCE_MISMATCH` | `positions` 내부 총액과 브로커 `balance()` 차이 ±5% 초과 |
| `ORPHANED_ORDER` | DB에 `SUBMITTED` 상태이나 브로커 `unfilled_orders()`에 존재하지 않음 |
| `UNRECONCILED_FILL` | DB는 `SUBMITTED`이나 브로커에서 이미 체결 완료 (WS 이벤트 누락) |
| `BROKER_ORDER_MISSING` | DB `orders`에 `broker_order_id` 없이 `SUBMITTED` 상태인 주문 존재 |

`UNRECONCILED_FILL`은 HARD Kill Switch 없이 DB 상태만 `FULLY_FILLED`로 갱신 후 계속 진행한다 (정상 누락 복구).

**RecoveryCheck 재시도 정책**: `BALANCE_MISMATCH` / `ORPHANED_ORDER` / `BROKER_ORDER_MISSING` 감지 시 즉시 Kill Switch로 진행하지 않고, **5초 간격으로 최대 3회 브로커 API 재조회** 후에도 유지되면 Kill Switch 발동.
브로커 일시 응답 지연으로 인한 오탐을 방지한다. `UNRECONCILED_FILL`은 재시도 없이 즉시 처리.

**KIS API 공통 retry 정책** (RecoveryCheck 외 모든 REST 호출에 동일 적용):
- 재시도 조건: HTTP 5xx, 네트워크 타임아웃
- 재시도 횟수: 최대 3회
- 대기 시간: exponential backoff with jitter — `base_delay × 2^attempt + random(0, base_delay)` (base_delay = 1초)
- 재시도 불가: HTTP 4xx (클라이언트 오류), 인증 실패
- 3회 모두 실패 시 `KisError::Network` 반환 → 호출 측에서 Kill Switch 또는 로그 처리

**retry 실패 → Kill Switch 경계 매핑**:

| 실패 호출 | 결과 |
|----------|------|
| `place_order` / `cancel_order` | `SOFT` Kill Switch (신규 진입 차단, 보유 포지션 유지) |
| `unfilled_orders` / `balance` (복구 절차 중) | `HARD` Kill Switch 후보 → RecoveryCheck `BROKER_ORDER_MISSING`으로 분류 |
| `price` / `orderbook` 등 시세 조회 | 로그 + WARN 알림, Kill Switch 불발동 |
| 토큰 갱신 (`token_issue`) | 재시도 후에도 실패 시 `HARD` Kill Switch (인증 없이 운영 불가) |

### 재시작 후 자동 거래 재개 조건

| 재시작 원인 | 자동 재개 여부 |
|------------|--------------|
| 정상 종료 후 재시작 (kill switch 파일 없음) | 복구 절차 성공 시 **자동 재개** |
| `SOFT` Kill Switch 해제 후 재시작 | 복구 절차 성공 시 **자동 재개** |
| `HARD` Kill Switch 후 재시작 | kill switch 파일 수동 삭제 후에만 재개 (**수동 확인 필수**) |
| 연속 손실 한도 도달로 중단 후 재시작 | `session_stats` 연속 손실 횟수 수동 초기화 후 재개 (**수동 확인 필수**) |

HARD Kill Switch 또는 연속 손실 중단의 경우, 자동 재개를 막는 이유는 운영자가 "왜 멈췄는지"를 반드시 확인하도록 강제하기 위해서다.

**재개 가능 여부 한 줄 판단 기준**: kill switch 파일이 없고, `session_stats` 연속 손실이 한도 미만이며, 복구 절차가 성공하면 자동 재개된다. 셋 중 하나라도 아니면 수동 개입 필요.

---

## 10. Graceful Shutdown

`Ctrl+C` / `SIGTERM` 수신:

1. `CancellationToken.cancel()` → 모든 task에 전파
2. Discovery/Analysis/Signal task 즉시 중단
3. Execution task: 미체결 주문 취소 API 호출 (타임아웃 5초)
4. 보유 포지션 상태 DB 저장
5. KisStream 구독 해제 및 연결 종료
6. 모든 task join 완료 후 종료
7. 최종 로그: `"Bot stopped. P&L today: +$XX.XX | Positions: N"`

---

## 11. 뉴스 소스: Finnhub

- **플랜**: Free tier (월 30만 API 호출)
- **사용 엔드포인트**:
  - `GET /news?category=general` — 시장 전반 뉴스
  - `GET /company-news?symbol=NVDA` — 종목별 뉴스 (24h)
  - `GET /calendar/earnings` — 실적 발표 캘린더
- **갱신**: 뉴스는 이벤트 드리븐 (웹훅 또는 15분 폴링)

---

## 12. 환경 설정

### 운영 프로파일

파라미터 전체를 바꾸지 않고 장세와 계좌 상황에 따라 프로파일을 전환한다.
`[profile]` 섹션에서 선택하며, 해당 프로파일의 값이 아래 기본값을 덮어쓴다.

| 파라미터 | 기본형 (Default) | 보수형 (Conservative) | 공격형 (Aggressive) |
|---------|----------------|----------------------|-------------------|
| `risk_per_trade` | 0.005 | 0.003 | 0.008 |
| `setup_score_threshold_entry` | 60 | 70 | 55 |
| `setup_score_threshold_llm` | 80 | 85 | 75 |
| `atr_stop_multiplier` | 1.5 | 2.0 | 1.2 |
| `profit_target_1_atr` | 2.0 | 2.5 | 1.5 |
| `profit_target_2_atr` | 4.0 | 5.0 | 3.0 |
| `daily_loss_limit` | 0.015 | 0.010 | 0.020 |
| `consecutive_loss_limit` | 3 | 2 | 4 |

- **기본형**: 신규 실거래 시작, 검증 단계
- **보수형**: 계좌 손실 복구 중이거나 고변동성 장세
- **공격형**: 전략 성과가 검증된 이후, 드라이런 지표 양호 시만 사용. **수동 전환만 허용.**

```toml
[profile]
active = "default"   # "default" | "conservative" | "aggressive"
```

### 프로파일 자동 전환 규칙

`Aggressive`는 절대 자동 전환되지 않는다 (수동 전환만). `Default ↔ Conservative` 만 자동 전환.

| 조건 | 현재 → 전환 |
|------|------------|
| 7일 누적 R < -2R | Default → **Conservative** |
| MDD 5% 도달 | Default → **Conservative** |
| 레짐 연속 손실 3회 (Default 상태) | Default → **Conservative** |
| 7일 누적 R > +1R AND 연속 손실 0 AND Conservative 상태 7일 경과 | Conservative → **Default** (복구 신호) |
| MDD 10% 도달 또는 연속 손실 한도 도달 | 어떤 프로파일이든 → **거래 일시 중단 + 수동 확인** |

자동 전환 발생 시 `strategy_stats`에 기록하고 로그로 알림을 출력한다.

**복귀 쿨다운 (플래핑 방지)**: Conservative 전환 후 Default 복귀는 아래 조건이 **모두** 충족될 때만 허용한다.
- Conservative 상태 최소 **3 거래일** 경과
- 7일 누적 R > +1R
- 연속 손실 횟수 = 0

쿨다운 미충족 시 복귀 조건이 맞아도 Default 전환을 보류하고 다음 확인 주기(1거래일)에 재평가한다.
재평가는 장 시작 직전에 자동 실행되며, 매 거래일 반복한다. 조건 충족 시 즉시 Default 전환.

```toml
[risk]
risk_per_trade = 0.005        # 거래당 허용 손실 (계좌의 0.5%)
max_open_positions = 5
max_position_pct = 0.10
max_sector_exposure = 0.30
daily_loss_limit = 0.015      # 1.5%
consecutive_loss_limit = 3
atr_stop_multiplier = 1.5

[signal]
setup_score_threshold_entry = 60   # 이 점수 미만은 탈락 (LLM/Rule 모두 없음)
setup_score_threshold_llm = 80     # 이 점수 이상만 LLM sanity check 호출
rule_strength_threshold = 0.65     # Score 60~79 경로의 RuleEngine 진입 임계값
fallback_rule_strength = 0.70      # LLM 미가용 시 rule_strength_threshold 상향값

[llm]
model = "claude-haiku-4-5-20251001"
timeout_secs = 10

[position]
profit_target_1_atr = 2.0     # 1차 익절 목표: 진입가 + ATR × 2
profit_target_2_atr = 4.0     # 2차 익절 목표: 진입가 + ATR × 4
trailing_atr_trending = 2.0   # Trending 레짐 trailing stop 폭
trailing_atr_volatile = 1.0   # Volatile 레짐 trailing stop 폭

[trading_hours]
entry_blackout_open_mins = 15   # 장 시작 후 금지 구간
entry_blackout_close_mins = 15  # 장 마감 전 금지 구간
eod_liquidate_mins = 15         # 장 마감 N분 전 전량 청산

[monitoring]
regime_consecutive_loss_limit = 5    # 동일 레짐 연속 손실 → 해당 레짐 일시 중단
regime_suspend_days = 7              # 레짐 일시 중단 기간 (일)
llm_underperformance_weeks = 3       # LLM ENTER 승률 부진 감지 기간
cumulative_r_alert_threshold = -5.0  # 30일 누적 R 알림 기준
mdd_alert_pct = 0.10                 # 최대 드로다운 알림 기준 (계좌 잔고의 10%)

[finnhub]
api_key = "${FINNHUB_API_KEY}"

[state]
db_path = "~/.local/share/trading_bot/state.db"
kill_switch_path = "~/.local/share/trading_bot/.kill_switch_active"
```

---

## 13. 로깅 & 감사

모든 의사결정을 로그에 기록한다. 사후 분석 및 버그 추적 필수 요건.

| 이벤트 | 로그 내용 |
|--------|----------|
| Market Regime 변경 | 레짐 종류, 조건 값 |
| Discovery 종목 추가 | 종목, 추가 이유 (거래량/등락/뉴스) |
| Entry Qualification 결과 | 통과/실패, 실패 사유 |
| RuleSignal | direction, strength |
| LLM 호출 | 입력 요약, 응답, 소요 시간, 토큰 수 |
| 주문 제출/체결/취소 | 주문 ID, 가격, 수량, 상태 |
| Risk 차단 | 차단 사유, 당시 상태 수치 |
| Kill Switch | 발동 조건 상세 |

---

## 14. 테스트 전략

### 단위 테스트
- `RuleEngine`: fixture 데이터로 신호 판정 검증
- `RiskSizer`: ATR/허용손실/수량 계산 수치 검증
- `OrderStateMachine`: 각 상태 전이 검증
- `MarketRegimeFilter`: 레짐 판정 경계값 검증

### 통합 테스트
- `KIS_INTEGRATION_TEST=1` 환경변수 설정 시 실행
- VTS(모의투자) 환경 필수 사용
- 실제 주문/체결 플로우 end-to-end 검증

다음 5개 시나리오는 운영 사고를 가장 많이 줄이므로 필수로 포함한다:

| 시나리오 | 검증 내용 |
|---------|----------|
| **재시작 복구** | DB 상태와 KIS API 잔고/미체결 일치 여부 → 일치 시 정상 기동, 불일치 시 Kill Switch |
| **중복 체결 이벤트** | 동일 체결 WebSocket 이벤트 2회 수신 시 포지션 수량 중복 누적 없음 (멱등성) |
| **WebSocket 끊김 후 REST 복구** | WS 연결 실패 후 `unfilled_orders()` 폴링으로 주문 상태 정확히 복원 |
| **부분 체결 후 세션 종료** | `CANCELLED_PARTIAL` 상태 처리 — 체결 수량은 포지션 유지, 미체결 수량만 포기 |
| **블랙아웃 경계** | 장 시작 `entry_blackout_open_mins`분 직전·직후 신호 발생 시 차단/허용 정확성 |

### Dry Run 모드
`DRY_RUN=1` 설정 시:
- 모든 주문을 실제 제출하지 않고 로그만 출력
- 신호/리스크 로직은 그대로 실행
- 실전 전 검증 필수 단계

---

## 15. 전략 검증 지표 & 드리프트 모니터링

**목적**: "왜 돈을 버는지/잃는지 모르는 꾸준한 소손실"을 막는다.
구조가 아무리 좋아도 전략의 실제 기대값과 드리프트를 지속 감시하지 않으면 시스템이 조용히 망가진다.

### 자동 집계 지표 (signal_log + positions → 집계)

모든 거래 종료 시 `session_stats`에 아래 지표를 누적 기록한다.

| 지표 | 설명 | 목적 |
|------|------|------|
| **Setup Score 구간별 승률** | 60~69 / 70~79 / 80+ 구간별 이긴 거래 비율 | 점수 임계값 보정 근거 |
| **LLM verdict별 손익** | ENTER/WATCH/BLOCK별 평균 R (손절 기준 R 배수) | LLM 유효성 검증 |
| **레짐별 평균 R** | Trending/Volatile/Quiet별 평균 R | 레짐 필터 효과 측정 |
| **Rule-only vs LLM-assisted 승률** | Score 60~79 경로 vs 80+ 경로 비교 | LLM 임계값 80 타당성 검토 |
| **일별 최대 드로다운** | 당일 최대 손실 / 최대 이익 | 손실 패턴 이상 감지 |
| **Setup Score 분포** | 구간별(< 60 / 60~79 / 80+) 일별 신호 수 | 임계값이 너무 높거나 낮은지 판단 |
| **Rule strength 분포** | 0.55~0.64 / 0.65~0.74 / 0.75~0.84 / 0.85+ 구간별 비율 | 신호 강도 쏠림 감지 |

이 지표들은 DB `strategy_stats` 테이블에 저장하고, 주기적으로 로그로 출력한다.

### 경보 vs 강제 전환 분리

모니터링 조건은 **알림만 발송하는 것**과 **자동으로 행동을 바꾸는 것**을 명확히 구분한다.

| 구분 | 조건 | 임계값 | 행동 |
|------|------|--------|------|
| **강제 전환** | 7일 누적 R | < -2R | Default → Conservative 자동 전환 |
| **강제 전환** | MDD | 계좌의 -5% | Default → Conservative 자동 전환 |
| **강제 전환** | 레짐 연속 손실 | 동일 레짐 5회 | 해당 레짐 진입 7일 중단 |
| **알림 전용** | 30일 누적 R | < -5R | 수동 재검토 알림 (자동 중단 없음) |
| **알림 전용** | MDD | 계좌의 -10% | 수동 재검토 알림 |
| **알림 전용** | LLM ENTER 승률 | rule-only보다 10%p 이상 낮게 3주 지속 | `setup_score_threshold_llm` 상향 제안 알림 |
| **알림 전용** | Setup Score 80+ 승률 | < 40% (2주 기준) | 전략 파라미터 재검토 알림 |

강제 전환은 즉시 파라미터를 바꾸고 `strategy_stats`에 기록한다.
알림 전용은 로그 출력만 하며 시스템 행동은 변경하지 않는다.

**알림 메시지 템플릿** (로그 + 향후 Telegram 연동 기준):

| 심각도 | 조건 | 메시지 템플릿 |
|--------|------|-------------|
| `WARN` | MDD -10% | `[WARN] MDD reached -10%. Review strategy before next session.` |
| `WARN` | 30d R < -5R | `[WARN] 30-day cumulative R below -5R. Strategy parameter review recommended.` |
| `INFO` | LLM 3주 부진 | `[INFO] LLM ENTER win rate underperforming rule-only by 10%p for 3 weeks. Consider raising setup_score_threshold_llm.` |
| `INFO` | Score 80+ 승률 저조 | `[INFO] Score 80+ win rate below 40% for 2 weeks. Check signal thresholds or feature design.` |
| `WARN` | 강제 Conservative 전환 | `[WARN] Profile switched to Conservative. Reason: {reason}. Cooldown: 3 trading days.` |
| `CRITICAL` | Kill Switch 발동 | `[CRITICAL] Kill Switch triggered ({mode}). Reason: {reason}. Manual intervention required.` |
| `WARN` | 레짐 진입 중단 | `[WARN] {regime} regime entry suspended for 7 days. Reason: 5 consecutive losses.` |

심각도 기준: `INFO` = 정보 전달, 즉각 행동 불필요 / `WARN` = 모니터링 강화 권고 / `CRITICAL` = 즉각 확인 필요.

**알림 채널별 정책**:

| 심각도 | 채널 |
|--------|------|
| `CRITICAL` | Telegram 즉시 발송 + 로그 기록 |
| `WARN` | 로그 기록 + Telegram 발송 |
| `INFO` | 로그 기록만 (Telegram 미발송) |

Telegram 연동은 `trading-server` 크레이트 담당이므로, `trading-bot`은 심각도별 이벤트를 채널로 발행하고 `trading-server`가 구독하여 발송한다.

**`trading-bot` ↔ `trading-server` 알림 인터페이스**:
```rust
// trading-bot이 발행하는 이벤트 타입 (trading-server가 구독)
pub enum AlertEvent {
    Info  { message: String },
    Warn  { message: String },
    Critical { message: String },
}
```
`trading-bot`은 발송 성공 여부에 무관하게 이벤트를 채널에 넣고 계속 실행한다. Telegram 429 / 네트워크 오류 처리 및 재시도 큐잉은 `trading-server` 책임이다.

### Dry Run 분석 필수 절차

실거래 전 VTS 환경에서 최소 2주 Dry Run 실행:
- 위 4개 지표가 모두 방향성을 보여야 실거래 전환 허용
- LLM verdict별 손익이 무작위(ENTER/WATCH/BLOCK 간 차이 없음)면 LLM 프롬프트 재설계 필요
- Setup Score 80+ 경로 거래 수가 너무 적으면 (`setup_score_threshold_llm` 높음) 임계값 하향 검토

---

## 16. 범위 외 (이 스펙에서 다루지 않음)

| 항목 | 이유 |
|------|------|
| TWAP/VWAP 알고리즘 주문 | 초기 버전 범위 외 (별도 플랜) |
| 국내주식 지원 | 해외주식 전용 |
| 백테스트 엔진 | `trading-server` 또는 별도 레포 담당 |
| Telegram 알림 구현 | `trading-server` 크레이트 담당 (`trading-bot`은 이벤트 발행만) |
