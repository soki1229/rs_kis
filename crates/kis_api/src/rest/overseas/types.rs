use std::fmt;

/// 해외 거래소 코드
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Exchange {
    NASD,
    NYSE,
    AMEX,
    SEHK,
    SHAA,
    SZAA,
    TKSE,
    HASE,
    VNSE,
    Other(String),
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Exchange::NASD => "NASD",
            Exchange::NYSE => "NYSE",
            Exchange::AMEX => "AMEX",
            Exchange::SEHK => "SEHK",
            Exchange::SHAA => "SHAA",
            Exchange::SZAA => "SZAA",
            Exchange::TKSE => "TKSE",
            Exchange::HASE => "HASE",
            Exchange::VNSE => "VNSE",
            Exchange::Other(s) => s.as_str(),
        };
        write!(f, "{}", s)
    }
}

impl From<&str> for Exchange {
    fn from(s: &str) -> Self {
        match s {
            "NASD" => Exchange::NASD,
            "NYSE" => Exchange::NYSE,
            "AMEX" => Exchange::AMEX,
            "SEHK" => Exchange::SEHK,
            "SHAA" => Exchange::SHAA,
            "SZAA" => Exchange::SZAA,
            "TKSE" => Exchange::TKSE,
            "HASE" => Exchange::HASE,
            "VNSE" => Exchange::VNSE,
            other => Exchange::Other(other.to_string()),
        }
    }
}

impl Exchange {
    /// 시세 조회용 3자리 코드 (NAS, NYS, AMS, HKS, SHS, SZS, TSE, HNX, HSX)
    pub fn to_price_code(&self) -> &str {
        match self {
            Exchange::NASD => "NAS",
            Exchange::NYSE => "NYS",
            Exchange::AMEX => "AMS",
            Exchange::SEHK => "HKS",
            Exchange::SHAA => "SHS",
            Exchange::SZAA => "SZS",
            Exchange::TKSE => "TSE",
            Exchange::HASE => "HNX",
            Exchange::VNSE => "HSX",
            Exchange::Other(s) => s.as_str(),
        }
    }

    /// 미국 거래소 여부
    pub fn is_us(&self) -> bool {
        matches!(self, Exchange::NASD | Exchange::NYSE | Exchange::AMEX)
    }
}

/// 매수/매도 구분
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderSide::Buy => write!(f, "Buy"),
            OrderSide::Sell => write!(f, "Sell"),
        }
    }
}

/// 주문 유형
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
    LimitAon,
    MarketClose,
    Other(String),
}

impl OrderType {
    pub fn to_ord_dvsn(&self) -> &str {
        match self {
            OrderType::Market => "01",
            OrderType::Limit => "00",
            OrderType::LimitAon => "32",
            OrderType::MarketClose => "34",
            OrderType::Other(s) => s.as_str(),
        }
    }
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_ord_dvsn())
    }
}

/// 계좌번호를 CANO(앞 8자리)와 ACNT_PRDT_CD(뒤 2자리)로 분리
pub fn split_account(account: &str) -> (&str, &str) {
    let mut it = account.splitn(2, '-');
    (it.next().unwrap_or(""), it.next().unwrap_or(""))
}

/// 주문 TR ID 반환
pub fn order_tr_id(exchange: &Exchange, side: &OrderSide, mock: bool) -> &'static str {
    match (exchange, side, mock) {
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Buy, false) => "TTTT1002U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Buy, true) => "VTTT1002U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Sell, false) => "TTTT1006U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, OrderSide::Sell, true) => "VTTT1006U",
        (Exchange::SEHK, OrderSide::Buy, false) => "TTTS1002U",
        (Exchange::SEHK, OrderSide::Buy, true) => "VTTS1002U",
        (Exchange::SEHK, OrderSide::Sell, false) => "TTTS1001U",
        (Exchange::SEHK, OrderSide::Sell, true) => "VTTS1001U",
        (Exchange::SHAA, OrderSide::Buy, false) => "TTTS0202U",
        (Exchange::SHAA, OrderSide::Buy, true) => "VTTS0202U",
        (Exchange::SHAA, OrderSide::Sell, false) => "TTTS1005U",
        (Exchange::SHAA, OrderSide::Sell, true) => "VTTS1005U",
        (Exchange::SZAA, OrderSide::Buy, false) => "TTTS0305U",
        (Exchange::SZAA, OrderSide::Buy, true) => "VTTS0305U",
        (Exchange::SZAA, OrderSide::Sell, false) => "TTTS0304U",
        (Exchange::SZAA, OrderSide::Sell, true) => "VTTS0304U",
        (Exchange::TKSE, OrderSide::Buy, false) => "TTTS0308U",
        (Exchange::TKSE, OrderSide::Buy, true) => "VTTS0308U",
        (Exchange::TKSE, OrderSide::Sell, false) => "TTTS0307U",
        (Exchange::TKSE, OrderSide::Sell, true) => "VTTS0307U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Buy, false) => "TTTS0311U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Buy, true) => "VTTS0311U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Sell, false) => "TTTS0310U",
        (Exchange::HASE | Exchange::VNSE, OrderSide::Sell, true) => "VTTS0310U",
        _ => "",
    }
}

/// 정정/취소 TR ID 반환
pub fn cancel_tr_id(exchange: &Exchange, mock: bool) -> &'static str {
    match (exchange, mock) {
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, false) => "TTTT1004U",
        (Exchange::NASD | Exchange::NYSE | Exchange::AMEX, true) => "VTTT1004U",
        (_, false) => "TTTS1003U",
        (_, true) => "VTTS1003U",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exchange_display() {
        assert_eq!(Exchange::NASD.to_string(), "NASD");
        assert_eq!(Exchange::Other("XLON".to_string()).to_string(), "XLON");
    }

    #[test]
    fn exchange_from_str() {
        assert_eq!(Exchange::from("NYSE"), Exchange::NYSE);
        assert_eq!(Exchange::from("XPAR"), Exchange::Other("XPAR".to_string()));
    }

    #[test]
    fn exchange_price_code() {
        assert_eq!(Exchange::NASD.to_price_code(), "NAS");
        assert_eq!(Exchange::SEHK.to_price_code(), "HKS");
    }

    #[test]
    fn exchange_is_us() {
        assert!(Exchange::NASD.is_us());
        assert!(Exchange::NYSE.is_us());
        assert!(!Exchange::SEHK.is_us());
    }

    #[test]
    fn split_account_basic() {
        let (cano, prdt) = split_account("50123456-01");
        assert_eq!(cano, "50123456");
        assert_eq!(prdt, "01");
    }

    #[test]
    fn order_tr_id_nasd_buy_real() {
        assert_eq!(
            order_tr_id(&Exchange::NASD, &OrderSide::Buy, false),
            "TTTT1002U"
        );
    }

    #[test]
    fn order_tr_id_nasd_buy_vts() {
        assert_eq!(
            order_tr_id(&Exchange::NASD, &OrderSide::Buy, true),
            "VTTT1002U"
        );
    }

    #[test]
    fn order_tr_id_nasd_sell_real() {
        assert_eq!(
            order_tr_id(&Exchange::NASD, &OrderSide::Sell, false),
            "TTTT1006U"
        );
    }

    #[test]
    fn order_tr_id_nasd_sell_vts() {
        assert_eq!(
            order_tr_id(&Exchange::NASD, &OrderSide::Sell, true),
            "VTTT1006U"
        );
    }

    #[test]
    fn order_tr_id_sehk() {
        assert_eq!(
            order_tr_id(&Exchange::SEHK, &OrderSide::Buy, false),
            "TTTS1002U"
        );
        assert_eq!(
            order_tr_id(&Exchange::SEHK, &OrderSide::Sell, false),
            "TTTS1001U"
        );
        assert_eq!(
            order_tr_id(&Exchange::SEHK, &OrderSide::Buy, true),
            "VTTS1002U"
        );
    }

    #[test]
    fn order_tr_id_other_returns_empty() {
        assert_eq!(
            order_tr_id(&Exchange::Other("XLON".to_string()), &OrderSide::Buy, false),
            ""
        );
    }

    #[test]
    fn cancel_tr_id_us_real() {
        assert_eq!(cancel_tr_id(&Exchange::NASD, false), "TTTT1004U");
        assert_eq!(cancel_tr_id(&Exchange::NYSE, false), "TTTT1004U");
        assert_eq!(cancel_tr_id(&Exchange::AMEX, false), "TTTT1004U");
    }

    #[test]
    fn cancel_tr_id_us_vts() {
        assert_eq!(cancel_tr_id(&Exchange::NASD, true), "VTTT1004U");
    }

    #[test]
    fn cancel_tr_id_non_us() {
        assert_eq!(cancel_tr_id(&Exchange::SEHK, false), "TTTS1003U");
        assert_eq!(cancel_tr_id(&Exchange::SEHK, true), "VTTS1003U");
        assert_eq!(cancel_tr_id(&Exchange::TKSE, false), "TTTS1003U");
    }

    #[test]
    fn order_type_to_ord_dvsn() {
        assert_eq!(OrderType::Limit.to_ord_dvsn(), "00");
        assert_eq!(OrderType::Market.to_ord_dvsn(), "01");
        assert_eq!(OrderType::LimitAon.to_ord_dvsn(), "32");
        assert_eq!(OrderType::MarketClose.to_ord_dvsn(), "34");
    }
}
