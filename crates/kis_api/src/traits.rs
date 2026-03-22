use crate::{Exchange, Holiday, KisError, KisStream, RankingItem};
use async_trait::async_trait;

/// REST + WebSocket 클라이언트 트레이트 (의존성 역전 / 테스트 모킹용)
#[async_trait]
pub trait KisApi: Send + Sync {
    /// WebSocket 스트림 생성
    async fn stream(&self) -> Result<KisStream, KisError>;

    /// 해외주식 거래량 순위 조회
    async fn volume_ranking(
        &self,
        exchange: &Exchange,
        count: u32,
    ) -> Result<Vec<RankingItem>, KisError>;

    /// 해외 공휴일 조회 (country: "USA", "JPN" 등)
    async fn holidays(&self, country: &str) -> Result<Vec<Holiday>, KisError>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kis_api_extended_is_object_safe() {
        // Box<dyn KisApi>가 컴파일되는지 확인 (volume_ranking + holidays 추가 후)
        let _: Option<Box<dyn KisApi>> = None;
    }
}
