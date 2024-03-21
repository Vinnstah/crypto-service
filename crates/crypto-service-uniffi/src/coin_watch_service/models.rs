use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfCoinsRequest {
    currency: String,
    sort: Sort,
    order: String,
    offset: u8,
    limit: u32,
    meta: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoinHistoryRequest {
    currency: String,
    code: String,
    start: u64,
    end: u64,
    meta: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Rank,
    Price,
    Volume,
    Code,
    Name,
    Age,
}

impl ListOfCoinsRequest {
    pub fn new(limit: u32) -> Self {
        Self {
            currency: "USD".into(),
            sort: Sort::Rank,
            order: "ascending".into(),
            offset: 0,
            limit: limit,
            meta: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coin {
    pub code: String,
    pub rate: f64,
    pub volume: i64,
    pub cap: i64,
    pub delta: Delta,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Delta {
    pub hour: f64,
    pub day: f64,
    pub week: f64,
    pub month: f64,
    pub quarter: f64,
    pub year: f64,
}

impl Delta {
    pub fn new(hour: f64, day: f64, week: f64, month: f64, quarter: f64, year: f64) -> Self {
        Self {
            hour,
            day,
            week,
            month,
            quarter,
            year,
        }
    }
}

impl Coin {
    pub fn new(code: String, rate: f64, volume: i64, cap: i64, delta: Delta) -> Self {
        Self {
            code,
            rate,
            volume,
            cap,
            delta,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CoinMetaRequest {
    pub currency: String,
    pub code: String,
    pub meta: bool,
}

impl CoinMetaRequest {
    pub fn new(code: String) -> Self {
        Self { currency: "USD".into(), code, meta: true }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinMeta {
    pub code: Option<String>,
    pub name: String,
    pub symbol: Option<String>,
    pub rank: i64,
    pub age: i64,
    pub color: String,
    pub png32: String,
    pub png64: String,
    pub webp32: String,
    pub webp64: String,
    #[serde(rename = "allTimeHighUSD")]
    pub all_time_high_usd: f64,
    pub links: Links,
    pub delta: Option<Delta>,
    pub history: Option<Vec<History>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct History {
    pub date: i64,
    pub rate: f64,
    pub volume: i64,
    pub cap: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub website: Option<String>,
    pub whitepaper: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AggregatedCoinInformation {
    pub name: String,
    pub symbol: String,
    pub rank: i64,
    pub rate: f64,
    pub color: String,
    pub png64: String
}

impl AggregatedCoinInformation {
    pub fn new(name: String, symbol: String, rank: i64, rate: f64) -> Self {
        Self { name, symbol, rank, rate, color: "".into(), png64: "".into() }
    }
}