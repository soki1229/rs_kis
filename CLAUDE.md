# rs_kis — Claude 작업 규칙

## 역할

**순수 KIS OpenAPI Rust 클라이언트 라이브러리.**
봇/전략 로직 없이 KIS 증권 API만 래핑한다.
`rs_kis_server`와 `rs_kis_pilot`에서 의존성으로 사용된다.

```
crates/kis_api/src/
  traits.rs          ← KisApi (US/해외), KisDomesticApi (KR/국내) trait
  client.rs          ← KisClient (US)
  domestic_client.rs ← KisDomesticClient (KR)
  stream.rs          ← KisStream (WebSocket, KR/US 공유)
  event.rs           ← KisEvent, TransactionData, QuoteData
  config.rs          ← KisConfig
  auth/              ← TokenManager, ApprovalKeyManager
  rest/
    overseas/        ← US REST API (order, quote, inquiry, analysis)
    domestic/        ← KR REST API (order, inquiry)
    rate_limit.rs    ← RateLimiter (15 req/s)
```

## 관련 레포

```
rs_kis/           ← 이 저장소 (KIS API 라이브러리)
rs_kis_server/    ← 봇 프레임워크 (../rs_kis_server)
rs_kis_pilot/     ← 전략 구현체 (../rs_kis_pilot)
```

---

## 변경 시 주의사항

### 하위 레포와의 연동

`rs_kis`를 수정하면:
1. `rs_kis`에서 커밋
2. `rs_kis_server/crates/server/Cargo.toml`과 `rs_kis_pilot/crates/pilot/Cargo.toml`의 git rev 업데이트
3. 각 레포에서 `cargo build`로 연동 확인

**로컬 개발 시:** git 참조를 path 참조로 임시 변경 가능:
```toml
# 개발 중 (임시)
kis_api = { path = "../../rs_kis/crates/kis_api" }

# 배포 (원상복구)
kis_api = { git = "...", rev = "..." }
```

### WebSocket (stream.rs) 수정 시

- `read_loop`는 `(DisconnectReason, bool)` 반환 — had_data로 백오프 리셋 여부 결정
- `KisStream::test_pair()`는 테스트용 직접 주입 인터페이스 — 시그니처 변경 금지
- 구독 메시지 포맷은 KIS 공식 문서 기준 유지 (`tr_id`, `tr_key`, `approval_key`)

### trait 수정 시 (KisApi / KisDomesticApi)

두 trait 모두 object-safe해야 한다.
`traits.rs` 테스트의 `*_is_object_safe` 테스트가 통과해야 한다:

```bash
cargo test --lib "is_object_safe"
```

### 파싱 필드 인덱스 (stream.rs)

KIS WebSocket 데이터는 `^` 구분자 파이프 포맷.
필드 인덱스 상수를 변경할 때는 반드시 KIS OpenAPI 개발가이드 원문과 대조한다:
- `HDFSCNT0`: 해외 실시간 체결
- `HDFSASP0/1`: 해외 실시간 호가
- `H0STCNT0`: 국내 실시간 체결
- `H0STASP0/1`: 국내 실시간 호가

---

## 빌드 및 테스트

```bash
# 빌드
cargo build

# 유닛 테스트 (통합 테스트는 실제 API 키 필요)
cargo test --lib

# 특정 테스트
cargo test --lib "parse_hdfscnt0"
```

---

## 컨텍스트 복구 체크리스트

1. `git log --oneline -10` — 최근 변경 파악
2. `cargo test --lib` — 현재 상태 확인
