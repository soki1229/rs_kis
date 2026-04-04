mod auth;
mod config;
mod error;
mod event;
pub mod rest;
mod stream;
mod traits;

mod client;
mod domestic_client;
#[cfg(any(test, feature = "test-utils"))]
mod mock_domestic;
pub use auth::{ApprovalKeyManager, TokenManager};
pub use client::KisClient;
pub use config::{KisConfig, KisConfigBuilder};
pub use domestic_client::KisDomesticClient;
pub use error::KisError;
pub use event::{KisEvent, OrderConfirmData, QuoteData, TransactionData};
#[cfg(any(test, feature = "test-utils"))]
pub use mock_domestic::MockDomesticKisApi;
pub use rest::domestic::{
    DomesticCancelOrderRequest, DomesticCancelOrderResponse, DomesticDailyChartRequest,
    DomesticExchange, DomesticOrderHistoryItem, DomesticOrderHistoryRequest, DomesticOrderType,
    DomesticPlaceOrderRequest, DomesticPlaceOrderResponse, DomesticRankingItem,
    DomesticUnfilledOrder,
};
pub use rest::overseas::analysis::market::{
    HighLowKind, MarketCapItem, NewHighLowItem, TradeTurnoverItem, VolumePowerItem,
};
pub use rest::overseas::analysis::ranking::{
    RankingItem, RankingRequest, RankingSort, VolumeSurgeItem,
};
pub use rest::overseas::deposit::DepositInfo;
pub use rest::overseas::inquiry::balance::{BalanceItem, BalanceResponse, BalanceSummary};
pub use rest::overseas::inquiry::orders::{OrderHistoryItem, OrderHistoryRequest, UnfilledOrder};
pub use rest::overseas::inquiry::profit::{
    BuyableAmountRequest, BuyableAmountResponse, PeriodProfitRequest, PeriodProfitResponse,
    ProfitItem, ProfitSummary,
};
pub use rest::overseas::order::cancel::{CancelKind, CancelOrderRequest, CancelOrderResponse};
pub use rest::overseas::order::place::{PlaceOrderRequest, PlaceOrderResponse};
pub use rest::overseas::quote::chart::{
    CandleBar, ChartPeriod, DailyChartRequest, MinuteBar, MinuteChartRequest,
};
pub use rest::overseas::quote::corporate::{DividendItem, Holiday, NewsItem};
pub use rest::overseas::quote::orderbook::{OrderbookLevel, OrderbookResponse};
pub use rest::overseas::quote::price::PriceResponse;
pub use rest::overseas::quote::search::{SearchResult, SymbolInfo};
pub use rest::overseas::types::{Exchange, OrderSide, OrderType};
pub use stream::{EventReceiver, KisStream, SubscriptionKind};
pub use traits::KisApi;
pub use traits::KisDomesticApi;
