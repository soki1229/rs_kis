use async_trait::async_trait;
use crate::{KisError, KisStream};

/// REST API 클라이언트 마커 트레이트
/// 실제 메서드는 KisClient impl에서 직접 구현되며,
/// 이 트레이트는 의존성 역전(테스트 모킹)을 위해 존재합니다.
#[async_trait]
pub trait KisApi: Send + Sync {
    /// WebSocket 스트림 생성
    async fn stream(&self) -> Result<KisStream, KisError>;
}

/// WebSocket 이벤트 소스 트레이트
#[async_trait]
pub trait KisEventSource: Send + Sync {
    /// WebSocket 스트림 생성 (stream()의 별칭 - 이벤트 소스로서의 역할 강조)
    async fn event_stream(&self) -> Result<KisStream, KisError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_object_safe_kis_api() {
        // KisApi가 object-safe한지 컴파일 시 확인
        let _: Option<Box<dyn KisApi>> = None;
    }

    fn assert_object_safe_kis_event_source() {
        let _: Option<Box<dyn KisEventSource>> = None;
    }

    #[test]
    fn traits_are_object_safe() {
        assert_object_safe_kis_api();
        assert_object_safe_kis_event_source();
    }
}
