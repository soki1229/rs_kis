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
        ↓
Signal Engine (Rule → LLM)
        ↓
Risk Sizing
        ↓
Execution (Order State Machine)
        ↓
Position Management
        ↑
Session Risk Control (횡단 관심사, 전 단계 감시)
```

각 단계는 별도 tokio task로 실행되며 채널(mpsc/broadcast)로 통신한다.
`CancellationToken`으로 전체를 동기화하여 graceful shutdown 보장.

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
| `Quiet` | 지수 등락률 ±0.3% 미만, 거래량 저조 | 신규 진입 중단 (보유 포지션 관리만) |

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

---

## 3. Entry Qualification

**목적**: 좋은 종목(후보)과 지금 들어갈 타이밍을 분리.
Discovery 풀 진입 후, 아래 조건을 모두 충족해야 Analysis로 넘어간다.

### 기술적 조건 (RuleEngine 전처리)
- 5일 이동평균 > 20일 이동평균 (골든크로스 방향)
- 당일 등락률 +1% ~ +8% 범위 (과열 제외)
- 분봉 기준 최근 5분 거래량이 그 이전 5분의 1.5배 이상

### 미시구조 조건
- 호가 불균형: 매수호가 잔량 / 매도호가 잔량 ≥ 1.3
- 최근 10분 신고가 돌파 또는 직전 저항선 돌파 확인

### 이벤트 금지 필터 (자동 패스)
아래 조건 중 하나라도 해당하면 해당 종목 당일 진입 금지:

| 조건 | 설명 |
|------|------|
| 실적 발표 ±1일 | 예측 불가 변동성 |
| FOMC / CPI 발표일 | 거시 이벤트 |
| 장 시작 후 `entry_blackout_open_mins`분 이내 | 초고변동 구간 |
| 장 마감 `entry_blackout_close_mins`분 이전 | 유동성 저하 구간 |

이벤트 캘린더: Finnhub `earnings_calendar` API로 매일 장 시작 전 갱신.

---

## 4. Signal Engine

### 4-1. RuleEngine (필수 통과 요건)

Entry Qualification을 통과한 종목에 대해 기술적 점수 산출.
`RuleSignal { direction: Direction, strength: f64 }` 반환.

- `strength < 0.6` → 진입 금지 (LLM 호출 없이 종료)
- `strength ≥ 0.6` → LlmEngine으로 전달

### 4-2. LlmEngine (보조 판정자)

**역할**: 가중 신호가 아닌 **진입 가능 / 관망 / 금지** 3단계 판정.
LLM이 뒤집을 수 있는 조건을 명시적으로 제한한다.

**입력 (단일 구조화 프롬프트)**:
```
종목: {symbol} | 현재가: {price} | 등락률: {change_pct}%
거래량 비율: {volume_ratio}x | 이동평균 상태: {ma_state}
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

**최종 의사결정**:

| RuleSignal | LLM Verdict | 결과 |
|-----------|-------------|------|
| strength ≥ 0.6 | ENTER | 진입 → Risk Sizing |
| strength ≥ 0.6 | WATCH | 관망 (30분 후 재평가) |
| strength ≥ 0.6 | BLOCK | 진입 금지 (이유 로그) |
| strength < 0.6 | — | LLM 호출 없이 진입 금지 |

**모델**: Claude Haiku (`claude-haiku-4-5-20251001`)
**비용 추정**: 활발한 날 5~15회 호출 → 월 $1~2

### 4-3. LLM 장애 처리
- LLM 타임아웃/에러 → `LlmUnavailable` 상태 전환
- `LlmUnavailable` 시: RuleEngine `strength ≥ 0.75`로 임계값 상향 + 포지션 사이즈 50% 축소
- 단순 rule-only 전환이 아니라 **전략 등급 하향** (더 보수적으로)
- 로그: "LLM unavailable. Threshold raised to 0.75, size halved."
- LLM 복구 감지 후 자동으로 원래 설정 복원

---

## 5. Risk Sizing

**목적**: 포지션 크기를 "계좌의 몇 %" 기준이 아닌 **허용 손실액 역산**으로 계산.

### ATR 기반 손절폭

```
손절가          = 진입가 - (ATR_14 × atr_stop_multiplier)
포지션 수량     = (계좌 잔고 × risk_per_trade) / (ATR_14 × atr_stop_multiplier)
```

- `risk_per_trade`: `[risk] risk_per_trade` 설정값 (Section 12)
- `atr_stop_multiplier`: `[risk] atr_stop_multiplier` 설정값 (Section 12)

예시 (`atr_stop_multiplier = 1.5`, `risk_per_trade = 0.005`):
- 계좌 $10,000 → 허용 손실 $50
- NVDA 진입가 $892, ATR_14 = $18 → 손절가 = $892 - $27 = $865
- 수량 = $50 / $27 = 1.85주

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

---

## 7. Position Management

### 진입 후 관리

| 조건 | 행동 |
|------|------|
| 수익 +3% 도달 | 수량의 50% 부분 익절 |
| 수익 +5% 도달 | 잔여 수량 전량 익절 |
| 손절가 도달 | 시장가 전량 청산 (지체 없이) |
| 보유 4시간 경과 (시간 손절) | 포지션 재평가 → BLOCK이면 청산 |
| 장 마감 `eod_liquidate_mins`분 전 | 전체 포지션 청산 (익일 갭 리스크 제거) |

### Trailing Stop
`수익 +3%` 부분 익절 이후 잔여 수량에 trailing stop 적용:
- 최고가 대비 -2% 이탈 시 잔여 전량 청산

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
- 미체결 주문이 5개 이상 동시 존재
- WebSocket 재연결 실패 3회 연속

Kill Switch 상태는 `[state] kill_switch_path` 경로에 파일 생성으로 영속화 (Section 12 참조).
재시작 시 파일 존재하면 수동으로 삭제하기 전까지 주문 차단 유지.

---

## 9. 상태 영속화 & 복구

### 영속화 항목
SQLite 파일 (`~/.local/share/trading_bot/state.db`)에 저장:

| 테이블 | 내용 |
|--------|------|
| `orders` | 주문 ID, 상태, 심볼, 수량, 가격, 타임스탬프 |
| `positions` | 현재 포지션, 매수가, 손절가, ATR (진입 시점 ATR_14 고정값 — 손절가 산출 기준, 이후 갱신 없음) |
| `session_stats` | 당일 손익, 연속 손실 횟수 |
| `signal_log` | 모든 신호 판정 결과 (Rule + LLM 포함) |

### 재시작 복구 절차
1. DB에서 이전 포지션 로드
2. KIS API `balance()` + `unfilled_orders()` 조회
3. 두 상태를 비교 → 불일치 시 Kill Switch 발동
4. 일치 시 정상 기동

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

```toml
[risk]
risk_per_trade = 0.005        # 거래당 허용 손실 (계좌의 0.5%)
max_open_positions = 5
max_position_pct = 0.10
max_sector_exposure = 0.30
daily_loss_limit = 0.015      # 1.5%
consecutive_loss_limit = 3
atr_stop_multiplier = 1.5

[llm]
model = "claude-haiku-4-5-20251001"
timeout_secs = 10
fallback_rule_threshold = 0.75  # LLM 미가용 시 상향

[trading_hours]
entry_blackout_open_mins = 15   # 장 시작 후 금지 구간
entry_blackout_close_mins = 15  # 장 마감 전 금지 구간
eod_liquidate_mins = 15         # 장 마감 N분 전 전량 청산

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

### Dry Run 모드
`DRY_RUN=1` 설정 시:
- 모든 주문을 실제 제출하지 않고 로그만 출력
- 신호/리스크 로직은 그대로 실행
- 실전 전 검증 필수 단계

---

## 범위 외

| 항목 | 이유 |
|------|------|
| TWAP/VWAP 알고리즘 주문 | 초기 버전 범위 외 (별도 플랜) |
| 국내주식 지원 | 해외주식 전용 |
| 백테스트 엔진 | `trading-server` 또는 별도 레포 담당 |
| Telegram 알림 | `trading-server` 크레이트 담당 |
