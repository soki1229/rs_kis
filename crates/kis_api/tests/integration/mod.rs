pub mod order;

/// 통합 테스트 스킵 매크로
/// `KIS_INTEGRATION_TEST=1` 환경변수가 없으면 테스트를 건너뜁니다.
#[macro_export]
macro_rules! skip_unless_integration {
    () => {
        if std::env::var("KIS_INTEGRATION_TEST").unwrap_or_default() != "1" {
            return;
        }
    };
}

/// VTS 환경에서 KisClient 생성
/// `.env` 파일에서 환경변수를 로드합니다 (이미 설정된 경우 사용).
pub fn make_vts_client() -> kis_api::KisClient {
    // .env 파일이 있으면 로드 (선택적)
    let _ = dotenvy::dotenv();
    let config = kis_api::KisConfig::from_env_vts()
        .expect("VTS credentials not set — check .env file");
    kis_api::KisClient::new(config)
}
