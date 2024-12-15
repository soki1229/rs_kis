use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct StockData {
    pub 종목코드: String,
    pub 현재가: f64,
    pub 매수잔량: u64,
    pub 매도잔량: u64,
    pub 거래량: u64,
    pub 매수체결량: u64,
    pub 매도체결량: u64,
}

pub struct StockAnalyzer {
    data_window: VecDeque<StockData>,
    window_size: usize,
}

impl StockAnalyzer {
    pub fn new(window_size: usize) -> Self {
        StockAnalyzer {
            data_window: VecDeque::with_capacity(window_size),
            window_size,
        }
    }

    pub fn add_data(&mut self, data: StockData) {
        if self.data_window.len() >= self.window_size {
            self.data_window.pop_front();
        }
        self.data_window.push_back(data);
    }

    pub fn analyze(&self) -> String {
        if self.data_window.is_empty() {
            return "데이터가 충분하지 않습니다.".to_string();
        }

        let latest = self.data_window.back().unwrap();
        let buying_pressure = self.calculate_buying_pressure();
        let volume_trend = self.calculate_volume_trend();
        let price_trend = self.calculate_price_trend();
        let order_imbalance = self.calculate_order_imbalance();

        format!(
            "종목코드: {}\n현재가: {:.2}\n매수압력: {:.2}%\n거래량 추세: {}\n가격 추세: {}\n주문 불균형: {:.2}\n예상 움직임: {}",
            latest.종목코드,
            latest.현재가,
            buying_pressure * 100.0,
            volume_trend,
            price_trend,
            order_imbalance,
            self.predict_movement(buying_pressure, volume_trend, order_imbalance)
        )
    }

    fn calculate_buying_pressure(&self) -> f64 {
        let total_buy = self.data_window.iter().map(|d| d.매수체결량).sum::<u64>() as f64;
        let total_sell = self.data_window.iter().map(|d| d.매도체결량).sum::<u64>() as f64;
        total_buy / (total_buy + total_sell)
    }

    fn calculate_volume_trend(&self) -> &str {
        if self.data_window.len() < 2 {
            return "데이터 부족";
        }
        let first = self.data_window.front().unwrap().거래량;
        let last = self.data_window.back().unwrap().거래량;
        if last > first {
            "증가"
        } else if last < first {
            "감소"
        } else {
            "유지"
        }
    }

    fn calculate_price_trend(&self) -> &str {
        if self.data_window.len() < 2 {
            return "데이터 부족";
        }
        let first = self.data_window.front().unwrap().현재가;
        let last = self.data_window.back().unwrap().현재가;
        if last > first {
            "상승"
        } else if last < first {
            "하락"
        } else {
            "보합"
        }
    }

    fn calculate_order_imbalance(&self) -> f64 {
        let latest = self.data_window.back().unwrap();
        (latest.매수잔량 as f64 - latest.매도잔량 as f64)
            / (latest.매수잔량 as f64 + latest.매도잔량 as f64)
    }

    fn predict_movement(
        &self,
        buying_pressure: f64,
        volume_trend: &str,
        order_imbalance: f64,
    ) -> &str {
        if buying_pressure > 0.55 && volume_trend == "증가" && order_imbalance > 0.1 {
            "강한 상승 가능성"
        } else if buying_pressure < 0.45 && volume_trend == "증가" && order_imbalance < -0.1 {
            "강한 하락 가능성"
        } else if buying_pressure > 0.5 && order_imbalance > 0.0 {
            "약한 상승 가능성"
        } else if buying_pressure < 0.5 && order_imbalance < 0.0 {
            "약한 하락 가능성"
        } else {
            "횡보 예상"
        }
    }
}
