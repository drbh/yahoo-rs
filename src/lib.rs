use serde::{Deserialize, Serialize};

pub struct YahooFinance {
    symbol: String,
    base: String,
    interval: String,

    pre_post: Option<bool>,
    period_1: Option<i32>,
    period_2: Option<i32>,
    events: Vec<String>,
}

impl YahooFinance {
    pub fn new<P: Into<String>>(symbol: P) -> YahooFinance {
        YahooFinance {
            symbol: symbol.into(),
            base: String::from("https://query1.finance.yahoo.com/v8/finance/chart/"),
            interval: String::from("1m"),
            pre_post: None,
            period_1: None,
            period_2: None,
            events: Vec::new(),
        }
    }

    pub fn interval<'a, P: Into<String>>(&'a mut self, arg: P) -> &'a mut YahooFinance {
        self.interval = arg.into();
        self
    }

    pub fn include_pre_post<'a, P: Into<bool>>(&'a mut self, arg: P) -> &'a mut YahooFinance {
        self.pre_post = Some(arg.into());
        self
    }

    pub fn set_period1<'a, P: Into<i32>>(&'a mut self, arg: P) -> &'a mut YahooFinance {
        self.period_1 = Some(arg.into());
        self
    }

    pub fn set_period2<'a, P: Into<i32>>(&'a mut self, arg: P) -> &'a mut YahooFinance {
        self.period_2 = Some(arg.into());
        self
    }

    pub fn events<'a, P: Into<String>>(&'a mut self, arg: P) -> &'a mut YahooFinance {
        // div%2Csplit
        self.events.push(arg.into());
        self
    }

    pub fn fetch(&self) -> Response {
        let mut extra_params = String::new();

        match self.pre_post {
            Some(val) => extra_params = format!("{}&includePrePost={}", extra_params, val),
            None => (),
        }

        match self.period_1 {
            Some(val) => extra_params = format!("{}&period1={}", extra_params, val),
            None => (),
        }

        match self.period_2 {
            Some(val) => extra_params = format!("{}&period2={}", extra_params, val),
            None => (),
        }

        let mut events = String::new();

        if self.events.len() > 0 {
            events = format!("&events={}", self.events.join("%2C"));
        }
        extra_params = format!("{}{}", extra_params, events);
        // println!("{:?}", extra_params);
        let params = format!(
            "?symbol={}&interval={}{}",
            self.symbol, self.interval, extra_params
        );
        let url = format!("{}{}{}", self.base, self.symbol, params);
        println!("{:?}", url);
        let response = minreq::get(url).send().unwrap();
        let json: Response = response.json().unwrap();
        json
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub chart: Chart,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub result: Vec<Result>,

    #[serde(skip)]
    pub error: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub meta: Meta,
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub currency: String,
    pub symbol: String,
    pub exchange_name: String,
    pub instrument_type: String,
    pub first_trade_date: i64,
    pub regular_market_time: i64,
    pub gmtoffset: i64,
    pub timezone: String,
    pub exchange_timezone_name: String,
    pub regular_market_price: f64,
    pub chart_previous_close: f64,
    pub previous_close: f64,
    pub scale: i64,
    pub price_hint: i64,
    pub current_trading_period: CurrentTradingPeriod,
    pub trading_periods: Vec<Vec<TradingPeriod>>,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTradingPeriod {
    pub pre: Pre,
    pub regular: Regular,
    pub post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pre {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regular {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPeriod {
    pub timezone: String,
    pub start: i64,
    pub end: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub high: Vec<Option<f64>>,
    pub open: Vec<Option<f64>>,
    pub close: Vec<Option<f64>>,
    pub volume: Vec<Option<i64>>,
    pub low: Vec<Option<f64>>,
}
