#![cfg(any(test, feature = "test-utils"))]

use crate::{rest::domestic::types::*, CandleBar, Holiday, KisDomesticApi, KisError, KisStream};

/// 테스트용 `KisDomesticApi` mock — 설정 가능한 캔들 응답 반환
///
/// `place_order_result`에 `None`을 설정하면 `KisError::Auth("mock error")` 를 반환한다.
pub struct MockDomesticKisApi {
    /// `Some(resp)` → Ok(resp), `None` → Err(KisError::Auth("mock error"))
    pub place_order_result: std::sync::Arc<std::sync::Mutex<Option<DomesticPlaceOrderResponse>>>,
    pub unfilled_orders_result: std::sync::Arc<std::sync::Mutex<Vec<DomesticUnfilledOrder>>>,
    /// `order_history` 응답. 빈 리스트면 "취소 또는 내역 없음"으로 처리됨.
    pub order_history_result: std::sync::Arc<std::sync::Mutex<Vec<DomesticOrderHistoryItem>>>,
}

impl MockDomesticKisApi {
    pub fn new() -> Self {
        Self {
            place_order_result: std::sync::Arc::new(std::sync::Mutex::new(Some(
                DomesticPlaceOrderResponse {
                    order_no: "KR001".into(),
                    order_date: "20260327".into(),
                    order_time: "090000".into(),
                },
            ))),
            unfilled_orders_result: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
            order_history_result: std::sync::Arc::new(std::sync::Mutex::new(vec![])),
        }
    }
}

impl Default for MockDomesticKisApi {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl KisDomesticApi for MockDomesticKisApi {
    async fn domestic_stream(&self) -> Result<KisStream, KisError> {
        unimplemented!("MockDomesticKisApi::domestic_stream not used in tests")
    }
    async fn domestic_volume_ranking(
        &self,
        _: &DomesticExchange,
        _: u32,
    ) -> Result<Vec<DomesticRankingItem>, KisError> {
        Ok(vec![])
    }
    async fn domestic_holidays(&self, _: &str) -> Result<Vec<Holiday>, KisError> {
        Ok(vec![])
    }
    async fn domestic_place_order(
        &self,
        _: DomesticPlaceOrderRequest,
    ) -> Result<DomesticPlaceOrderResponse, KisError> {
        self.place_order_result
            .lock()
            .unwrap()
            .clone()
            .ok_or_else(|| KisError::Auth("mock error".into()))
    }
    async fn domestic_cancel_order(
        &self,
        _: DomesticCancelOrderRequest,
    ) -> Result<DomesticCancelOrderResponse, KisError> {
        Ok(DomesticCancelOrderResponse {
            order_no: String::new(),
            order_date: String::new(),
            order_time: String::new(),
        })
    }
    async fn domestic_daily_chart(
        &self,
        _: DomesticDailyChartRequest,
    ) -> Result<Vec<CandleBar>, KisError> {
        Ok(vec![])
    }
    async fn domestic_unfilled_orders(&self) -> Result<Vec<DomesticUnfilledOrder>, KisError> {
        Ok(self.unfilled_orders_result.lock().unwrap().clone())
    }
    async fn domestic_order_history(
        &self,
        _: DomesticOrderHistoryRequest,
    ) -> Result<Vec<DomesticOrderHistoryItem>, KisError> {
        Ok(self.order_history_result.lock().unwrap().clone())
    }
}
