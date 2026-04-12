use crate::models::*;
use crate::{KisClient, KisEnv};
use dotenvy::dotenv;
use std::env;
use std::path::PathBuf;
use tokio::sync::OnceCell;

static SHARED_CLIENT: OnceCell<KisClient> = OnceCell::const_new();

async fn get_test_client() -> &'static KisClient {
    SHARED_CLIENT
        .get_or_init(|| async {
            dotenv().ok();
            let key = env::var("VTS_APP_KEY").expect("VTS_APP_KEY not set");
            let secret = env::var("VTS_APP_SECRET").expect("VTS_APP_SECRET not set");
            let cache_path = PathBuf::from(".token_cache.vts.json");
            KisClient::with_cache(&key, &secret, KisEnv::Vts, Some(cache_path))
                .await
                .expect("Failed to init client")
        })
        .await
}

mod real {
    use super::*;
    mod stock {
        use super::*;
        mod trading {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_inquire_psbl_rvsecncl() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_psbl_rvsecncl(InquirePsblRvsecnclRequest::default())
                    .await;
                println!(
                    "API 주식정정취소가능주문조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_psbl_sell() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_psbl_sell(InquirePsblSellRequest::default())
                    .await;
                println!(
                    "API 매도가능수량조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_credit_psamount() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_credit_psamount(InquireCreditPsamountRequest::default())
                    .await;
                println!(
                    "API 신용매수가능조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_order_resv_ccnl() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .order_resv_ccnl(OrderResvCcnlRequest::default())
                    .await;
                println!(
                    "API 주식예약주문조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_present_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_present_balance(InquirePresentBalanceRequest::default())
                    .await;
                println!(
                    "API 퇴직연금 체결기준잔고 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_ccld_next() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_daily_ccld_next(InquireDailyCcldNextRequest::default())
                    .await;
                println!(
                    "API 퇴직연금 미체결내역 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_psbl_order_next() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_psbl_order_next(InquirePsblOrderNextRequest::default())
                    .await;
                println!(
                    "API 퇴직연금 매수가능조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_deposit() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_deposit(InquireDepositRequest::default())
                    .await;
                println!(
                    "API 퇴직연금 예수금조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_balance_next() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_balance_next(InquireBalanceNextRequest::default())
                    .await;
                println!(
                    "API 퇴직연금 잔고조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_balance_rlz_pl() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_balance_rlz_pl(InquireBalanceRlzPlRequest::default())
                    .await;
                println!(
                    "API 주식잔고조회_실현손익 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_account_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_account_balance(InquireAccountBalanceRequest::default())
                    .await;
                println!(
                    "API 투자계좌자산현황조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_period_profit() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_period_profit(InquirePeriodProfitRequest::default())
                    .await;
                println!(
                    "API 기간별손익일별합산조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_period_trade_profit() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_period_trade_profit(InquirePeriodTradeProfitRequest::default())
                    .await;
                println!(
                    "API 기간별매매손익현황조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_intgr_margin() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .intgr_margin(IntgrMarginRequest::default())
                    .await;
                println!(
                    "API 주식통합증거금 현황 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_period_rights() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .period_rights(PeriodRightsRequest::default())
                    .await;
                println!(
                    "API 기간별계좌권리현황조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
        mod quotations {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_inquire_price_2() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_price_2(InquirePrice2Request::default())
                    .await;
                println!(
                    "API 주식현재가 시세2 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_time_dailychartprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_time_dailychartprice(InquireTimeDailychartpriceRequest::default())
                    .await;
                println!(
                    "API 주식일별분봉조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_overtime_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_overtime_price(InquireOvertimePriceRequest::default())
                    .await;
                println!(
                    "API 국내주식 시간외현재가 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_overtime_asking_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_overtime_asking_price(InquireOvertimeAskingPriceRequest::default())
                    .await;
                println!(
                    "API 국내주식 시간외호가 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_exp_closing_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .exp_closing_price(ExpClosingPriceRequest::default())
                    .await;
                println!(
                    "API 국내주식 장마감 예상체결가 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_index_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_index_price(InquireIndexPriceRequest::default())
                    .await;
                println!(
                    "API 국내업종 현재지수 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_index_daily_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_index_daily_price(InquireIndexDailyPriceRequest::default())
                    .await;
                println!(
                    "API 국내업종 일자별지수 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_index_tickprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_index_tickprice(InquireIndexTickpriceRequest::default())
                    .await;
                println!(
                    "API 국내업종 시간별지수(초) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_index_timeprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_index_timeprice(InquireIndexTimepriceRequest::default())
                    .await;
                println!(
                    "API 국내업종 시간별지수(분) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_time_indexchartprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_time_indexchartprice(InquireTimeIndexchartpriceRequest::default())
                    .await;
                println!(
                    "API 업종 분봉조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_index_category_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_index_category_price(InquireIndexCategoryPriceRequest::default())
                    .await;
                println!(
                    "API 국내업종 구분별전체시세 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_exp_index_trend() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .exp_index_trend(ExpIndexTrendRequest::default())
                    .await;
                println!(
                    "API 국내주식 예상체결지수 추이 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_exp_total_index() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .exp_total_index(ExpTotalIndexRequest::default())
                    .await;
                println!(
                    "API 국내주식 예상체결 전체지수 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_vi_status() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_vi_status(InquireViStatusRequest::default())
                    .await;
                println!(
                    "API 변동성완화장치(VI) 현황 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_comp_interest() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .comp_interest(CompInterestRequest::default())
                    .await;
                println!(
                    "API 금리 종합(국내채권/금리) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_news_title() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .news_title(NewsTitleRequest::default())
                    .await;
                println!(
                    "API 종합 시황/공시(제목) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_chk_holiday() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .chk_holiday(ChkHolidayRequest::default())
                    .await;
                println!(
                    "API 국내휴장일조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_market_time() {
                let client = super::super::super::get_test_client().await;
                let result = client.stock().quotations().market_time(()).await;
                println!(
                    "API 국내선물 영업일조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_search_info() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .search_info(SearchInfoRequest::default())
                    .await;
                println!(
                    "API 상품기본조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_search_stock_info() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .search_stock_info(SearchStockInfoRequest::default())
                    .await;
                println!(
                    "API 주식기본조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_credit_by_company() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .credit_by_company(CreditByCompanyRequest::default())
                    .await;
                println!(
                    "API 국내주식 당사 신용가능종목 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_estimate_perform() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .estimate_perform(EstimatePerformRequest::default())
                    .await;
                println!(
                    "API 국내주식 종목추정실적 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_lendable_by_company() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .lendable_by_company(LendableByCompanyRequest::default())
                    .await;
                println!(
                    "API 당사 대주가능 종목 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_invest_opinion() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .invest_opinion(InvestOpinionRequest::default())
                    .await;
                println!(
                    "API 국내주식 종목투자의견 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_invest_opbysec() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .invest_opbysec(InvestOpbysecRequest::default())
                    .await;
                println!(
                    "API 국내주식 증권사별 투자의견 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_psearch_title() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .psearch_title(PsearchTitleRequest::default())
                    .await;
                println!(
                    "API 종목조건검색 목록조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_psearch_result() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .psearch_result(PsearchResultRequest::default())
                    .await;
                println!(
                    "API 종목조건검색조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_intstock_grouplist() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .intstock_grouplist(IntstockGrouplistRequest::default())
                    .await;
                println!(
                    "API 관심종목 그룹조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_intstock_multprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .intstock_multprice(IntstockMultpriceRequest::default())
                    .await;
                println!(
                    "API 관심종목(멀티종목) 시세조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_intstock_stocklist_by_group() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .intstock_stocklist_by_group(IntstockStocklistByGroupRequest::default())
                    .await;
                println!(
                    "API 관심종목 그룹별 종목조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_foreign_institution_total() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .foreign_institution_total(ForeignInstitutionTotalRequest::default())
                    .await;
                println!(
                    "API 국내기관_외국인 매매종목가집계 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_frgnmem_trade_estimate() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .frgnmem_trade_estimate(FrgnmemTradeEstimateRequest::default())
                    .await;
                println!(
                    "API 외국계 매매종목 가집계 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_investor_trade_by_stock_daily() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .investor_trade_by_stock_daily(InvestorTradeByStockDailyRequest::default())
                    .await;
                println!(
                    "API 종목별 투자자매매동향(일별) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_investor_time_by_market() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_investor_time_by_market(InquireInvestorTimeByMarketRequest::default())
                    .await;
                println!(
                    "API 시장별 투자자매매동향(시세) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_investor_daily_by_market() {
                let client = super::super::super::get_test_client().await;
                let result =
                    client
                        .stock()
                        .quotations()
                        .inquire_investor_daily_by_market(
                            InquireInvestorDailyByMarketRequest::default(),
                        )
                        .await;
                println!(
                    "API 시장별 투자자매매동향(일별) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_frgnmem_pchs_trend() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .frgnmem_pchs_trend(FrgnmemPchsTrendRequest::default())
                    .await;
                println!(
                    "API 종목별 외국계 순매수추이 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_frgnmem_trade_trend() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .frgnmem_trade_trend(FrgnmemTradeTrendRequest::default())
                    .await;
                println!(
                    "API 회원사 실시간 매매동향(틱) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_member_daily() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_member_daily(InquireMemberDailyRequest::default())
                    .await;
                println!(
                    "API 주식현재가 회원사 종목매매동향 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_program_trade_by_stock() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .program_trade_by_stock(ProgramTradeByStockRequest::default())
                    .await;
                println!(
                    "API 종목별 프로그램매매추이(체결) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_program_trade_by_stock_daily() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .program_trade_by_stock_daily(ProgramTradeByStockDailyRequest::default())
                    .await;
                println!(
                    "API 종목별 프로그램매매추이(일별) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_investor_trend_estimate() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .investor_trend_estimate(InvestorTrendEstimateRequest::default())
                    .await;
                println!(
                    "API 종목별 외인기관 추정가집계 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_trade_volume() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_daily_trade_volume(InquireDailyTradeVolumeRequest::default())
                    .await;
                println!(
                    "API 종목별일별매수매도체결량 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_comp_program_trade_today() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .comp_program_trade_today(CompProgramTradeTodayRequest::default())
                    .await;
                println!(
                    "API 프로그램매매 종합현황(시간) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_comp_program_trade_daily() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .comp_program_trade_daily(CompProgramTradeDailyRequest::default())
                    .await;
                println!(
                    "API 프로그램매매 종합현황(일별) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_investor_program_trade_today() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .investor_program_trade_today(InvestorProgramTradeTodayRequest::default())
                    .await;
                println!(
                    "API 프로그램매매 투자자매매동향(당일) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_daily_credit_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .daily_credit_balance(DailyCreditBalanceRequest::default())
                    .await;
                println!(
                    "API 국내주식 신용잔고 일별추이 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_exp_price_trend() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .exp_price_trend(ExpPriceTrendRequest::default())
                    .await;
                println!(
                    "API 국내주식 예상체결가 추이 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_daily_short_sale() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .daily_short_sale(DailyShortSaleRequest::default())
                    .await;
                println!(
                    "API 국내주식 공매도 일별추이 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_tradprt_byamt() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .tradprt_byamt(TradprtByamtRequest::default())
                    .await;
                println!(
                    "API 국내주식 체결금액별 매매비중 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_mktfunds() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .mktfunds(MktfundsRequest::default())
                    .await;
                println!(
                    "API 국내 증시자금 종합 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_daily_loan_trans() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .daily_loan_trans(DailyLoanTransRequest::default())
                    .await;
                println!(
                    "API 종목별 일별 대차거래추이 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_capture_uplowprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .capture_uplowprice(CaptureUplowpriceRequest::default())
                    .await;
                println!(
                    "API 국내주식 상하한가 포착 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_pbar_tratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .pbar_tratio(PbarTratioRequest::default())
                    .await;
                println!(
                    "API 국내주식 매물대/거래비중 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_volume_rank() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .volume_rank(VolumeRankNextRequest::default())
                    .await;
                println!(
                    "API 거래량순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
        mod finance {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_balance_sheet() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .balance_sheet(BalanceSheetRequest::default())
                    .await;
                println!(
                    "API 국내주식 대차대조표 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_income_statement() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .income_statement(IncomeStatementRequest::default())
                    .await;
                println!(
                    "API 국내주식 손익계산서 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_financial_ratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .financial_ratio(FinancialRatioRequest::default())
                    .await;
                println!(
                    "API 국내주식 재무비율 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_profit_ratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .profit_ratio(ProfitRatioRequest::default())
                    .await;
                println!(
                    "API 국내주식 수익성비율 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_other_major_ratios() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .other_major_ratios(OtherMajorRatiosRequest::default())
                    .await;
                println!(
                    "API 국내주식 기타주요비율 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_stability_ratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .stability_ratio(StabilityRatioRequest::default())
                    .await;
                println!(
                    "API 국내주식 안정성비율 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_growth_ratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .finance()
                    .growth_ratio(GrowthRatioRequest::default())
                    .await;
                println!(
                    "API 국내주식 성장성비율 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
        mod ksdinfo {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_dividend() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .dividend(DividendRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(배당일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_purreq() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .purreq(PurreqRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(주식매수청구일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_merger_split() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .merger_split(MergerSplitRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(합병/분할일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_rev_split() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .rev_split(RevSplitRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(액면교체일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_cap_dcrs() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .cap_dcrs(CapDcrsRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(자본감소일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_list_info() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .list_info(ListInfoRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(상장정보일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_pub_offer() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .pub_offer(PubOfferRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(공모주청약일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_forfeit() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .forfeit(ForfeitRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(실권주일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_mand_deposit() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .mand_deposit(MandDepositRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(의무예치일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_paidin_capin() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .paidin_capin(PaidinCapinRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(유상증자일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_bonus_issue() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .bonus_issue(BonusIssueRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(무상증자일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_sharehld_meet() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ksdinfo()
                    .sharehld_meet(SharehldMeetRequest::default())
                    .await;
                println!(
                    "API 예탁원정보(주주총회일정) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
        mod ranking {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_overtime_exp_trans_fluct() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .overtime_exp_trans_fluct(OvertimeExpTransFluctRequest::default())
                    .await;
                println!(
                    "API 국내주식 시간외예상체결등락률 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_fluctuation() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .fluctuation(FluctuationRequest::default())
                    .await;
                println!(
                    "API 국내주식 등락률 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_quote_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .quote_balance(QuoteBalanceRequest::default())
                    .await;
                println!(
                    "API 국내주식 호가잔량 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_profit_asset_index() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .profit_asset_index(ProfitAssetIndexRequest::default())
                    .await;
                println!(
                    "API 국내주식 수익자산지표 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_market_cap() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .market_cap(MarketCapRequest::default())
                    .await;
                println!(
                    "API 국내주식 시가총액 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_finance_ratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .finance_ratio(FinanceRatioRequest::default())
                    .await;
                println!(
                    "API 국내주식 재무비율 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_after_hour_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .after_hour_balance(AfterHourBalanceRequest::default())
                    .await;
                println!(
                    "API 국내주식 시간외잔량 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_prefer_disparate_ratio() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .prefer_disparate_ratio(PreferDisparateRatioRequest::default())
                    .await;
                println!(
                    "API 국내주식 우선주/괴리율 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_disparity() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .disparity(DisparityRequest::default())
                    .await;
                println!(
                    "API 국내주식 이격도 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_market_value() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .market_value(MarketValueRequest::default())
                    .await;
                println!(
                    "API 국내주식 시장가치 순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_volume_power() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .volume_power(VolumePowerRequest::default())
                    .await;
                println!(
                    "API 국내주식 체결강도 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_top_interest_stock() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .top_interest_stock(TopInterestStockRequest::default())
                    .await;
                println!(
                    "API 국내주식 관심종목등록 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_exp_trans_updown() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .exp_trans_updown(ExpTransUpdownRequest::default())
                    .await;
                println!(
                    "API 국내주식 예상체결 상승/하락상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_traded_by_company() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .traded_by_company(TradedByCompanyRequest::default())
                    .await;
                println!(
                    "API 국내주식 당사매매종목 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_near_new_highlow() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .near_new_highlow(NearNewHighlowRequest::default())
                    .await;
                println!(
                    "API 국내주식 신고/신저근접종목 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_dividend_rate() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .dividend_rate(DividendRateRequest::default())
                    .await;
                println!(
                    "API 국내주식 배당률 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_bulk_trans_num() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .bulk_trans_num(BulkTransNumRequest::default())
                    .await;
                println!(
                    "API 국내주식 대량체결건수 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_credit_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .credit_balance(CreditBalanceRequest::default())
                    .await;
                println!(
                    "API 국내주식 신용잔고 상위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_short_sale() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .short_sale(ShortSaleRequest::default())
                    .await;
                println!(
                    "API 국내주식 공매도 상위종목 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_overtime_fluctuation() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .overtime_fluctuation(OvertimeFluctuationRequest::default())
                    .await;
                println!(
                    "API 국내주식 시간외등락율순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_overtime_volume() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .ranking()
                    .overtime_volume(OvertimeVolumeRequest::default())
                    .await;
                println!(
                    "API 국내주식 시간외거래량순위 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_hts_top_view() {
                let client = super::super::super::get_test_client().await;
                let result = client.stock().ranking().hts_top_view(()).await;
                println!(
                    "API HTS조회상위20종목 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
    }
}
mod vts {
    use super::*;
    mod stock {
        use super::*;
        mod trading {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_ccld() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_daily_ccld(InquireDailyCcldRequest::default())
                    .await;
                println!(
                    "API 주식일별주문체결조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_balance() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_balance(InquireBalanceRequest::default())
                    .await;
                println!(
                    "API 주식잔고조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_psbl_order() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .trading()
                    .inquire_psbl_order(InquirePsblOrderRequest::default())
                    .await;
                println!(
                    "API 매수가능조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
        mod quotations {
            use super::*;
            #[tokio::test]
            #[ignore]
            async fn test_inquire_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_price(InquirePriceRequest::default())
                    .await;
                println!(
                    "API 주식현재가 시세 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_ccnl() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_ccnl(InquireCcnlRequest::default())
                    .await;
                println!(
                    "API 주식현재가 체결 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_daily_price(InquireDailyPriceRequest::default())
                    .await;
                println!(
                    "API 주식현재가 일자별 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_asking_price_exp_ccn() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_asking_price_exp_ccn(InquireAskingPriceExpCcnRequest::default())
                    .await;
                println!(
                    "API 주식현재가 호가/예상체결 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_investor() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_investor(InquireInvestorRequest::default())
                    .await;
                println!(
                    "API 주식현재가 투자자 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_member() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_member(InquireMemberRequest::default())
                    .await;
                println!(
                    "API 주식현재가 회원사 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_itemchartprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_daily_itemchartprice(InquireDailyItemchartpriceRequest::default())
                    .await;
                println!(
                    "API 국내주식기간별시세(일/주/월/년) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_time_itemchartprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_time_itemchartprice(InquireTimeItemchartpriceRequest::default())
                    .await;
                println!(
                    "API 주식당일분봉조회 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_time_itemconclusion() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_time_itemconclusion(InquireTimeItemconclusionRequest::default())
                    .await;
                println!(
                    "API 주식현재가 당일시간대별체결 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_overtimeprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_daily_overtimeprice(InquireDailyOvertimepriceRequest::default())
                    .await;
                println!(
                    "API 주식현재가 시간외일자별주가 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_time_overtimeconclusion() {
                let client = super::super::super::get_test_client().await;
                let result =
                    client
                        .stock()
                        .quotations()
                        .inquire_time_overtimeconclusion(
                            InquireTimeOvertimeconclusionRequest::default(),
                        )
                        .await;
                println!(
                    "API 주식현재가 시간외시간별체결 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_elw_price() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_elw_price(InquireElwPriceRequest::default())
                    .await;
                println!(
                    "API ELW 현재가 시세 endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }

            #[tokio::test]
            #[ignore]
            async fn test_inquire_daily_indexchartprice() {
                let client = super::super::super::get_test_client().await;
                let result = client
                    .stock()
                    .quotations()
                    .inquire_daily_indexchartprice(InquireDailyIndexchartpriceRequest::default())
                    .await;
                println!(
                    "API 국내주식업종기간별시세(일/주/월/년) endpoint status: {:?}",
                    result.is_ok() || result.is_err()
                );
            }
        }
    }
}
