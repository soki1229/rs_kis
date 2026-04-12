# rs_kis (Zero Boilerplate & Context-Rich)

한국투자증권 KIS OpenAPI를 위한 **데이터 기반 차세대 Rust SDK**입니다. 이 크레이트는 수동 코딩을 완전히 배제하고, KIS API 포털의 최신 명세와 자연어 가이드를 100% Rust 코드로 이식하는 것을 목표로 합니다.

## 🌟 핵심 가치 (Core Values)

### 1. Zero Boilerplate (100% Automation)
- **명세서가 곧 코드**: 250개 이상의 KIS API 엔드포인트와 요청/응답 모델을 수동 작업 없이 자동으로 생성합니다.
- **Self-Updating**: GitHub Actions가 매일 새벽 KIS 문서를 감지하고, 변경 시 자동으로 코드를 갱신하여 PR을 생성합니다.

### 2. Context-Rich Documentation
- **자연어 가이드 이식**: KIS 문서의 복잡한 한글 설명(예: 주문구분 코드값 등)을 Rust `///` 주석으로 완벽하게 보존합니다. IDE에서 코딩하며 즉시 도메인 지식을 확인할 수 있습니다.

### 3. Namespace-based Design
- **계층적 접근**: `client.stock().trading().order_cash()`와 같이 API 경로에 기반한 계층 구조를 제공하여 API 발견 가능성(Discoverability)을 극대화했습니다.

### 4. Enterprise-grade Reliability
- **Token Caching**: KIS의 1분당 1회 토큰 발급 제한을 파일 기반 캐싱 엔진으로 완벽하게 해결했습니다.
- **Thread-Safety**: `Arc`와 `RwLock` 기반 설계로 고성능 멀티스레드 트레이딩 환경에 최적화되어 있습니다.
- **Environment Aware**: `KisEnv` 설정을 통해 실전/모의투자 환경에 맞는 TR ID와 URL을 지능적으로 자동 선택합니다.

## 🏗️ 프로젝트 구조
- `src/client.rs`: 핵심 전송 엔진 및 토큰 캐싱 로직
- `src/generated/`: 자동화 파이프라인의 산출물
    - `models.rs`: 500개 이상의 정규화된 데이터 구조체
    - `stock.rs`: 국내 주식 API (Namespace 그룹화)
    - `overseas.rs`: 해외 주식 API
    - `tests.rs`: 전수 검증용 Smoke Test 세트
- `scripts/`: Playwright 기반 명세 추출기 및 Rust 코드 생성기
