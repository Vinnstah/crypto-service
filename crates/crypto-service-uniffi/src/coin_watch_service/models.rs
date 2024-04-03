use serde::{Deserialize, Serialize};
use uniffi::{Enum, Record};

#[derive(Serialize, Deserialize, Debug, Clone, Record)]
pub struct ListOfCoinsRequest {
    currency: String,
    sort: Sort,
    order: String,
    offset: u8,
    limit: u32,
    meta: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Record)]
pub struct CoinHistoryRequest {
    currency: String,
    code: String,
    start: u64,
    end: u64,
    meta: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Enum)]
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
            limit,
            meta: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Record)]
pub struct Coin {
    pub code: Option<String>,
    pub rate: Option<f64>,
    pub volume: Option<i64>,
    pub cap: Option<i64>,
    pub delta: Delta,
}

#[derive(Debug, Serialize, Deserialize, Record)]
pub struct Delta {
    pub hour: Option<f64>,
    pub day: Option<f64>,
    pub week: Option<f64>,
    pub month: Option<f64>,
    pub quarter: Option<f64>,
    pub year: Option<f64>,
}

impl Delta {
    pub fn new(
        hour: Option<f64>,
        day: Option<f64>,
        week: Option<f64>,
        month: Option<f64>,
        quarter: Option<f64>,
        year: Option<f64>,
    ) -> Self {
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
    pub fn new(
        code: Option<String>,
        rate: Option<f64>,
        volume: Option<i64>,
        cap: Option<i64>,
        delta: Delta,
    ) -> Self {
        Self {
            code,
            rate,
            volume,
            cap,
            delta,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Record)]
pub struct CoinMetaRequest {
    pub currency: String,
    pub code: String,
    pub meta: bool,
}

impl CoinMetaRequest {
    pub fn new(code: String) -> Self {
        Self {
            currency: "USD".into(),
            code,
            meta: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Record)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Record)]
pub struct History {
    pub date: i64,
    pub rate: f64,
    pub volume: i64,
    pub cap: i64,
}

#[derive(Debug, Serialize, Deserialize, Record)]
pub struct Links {
    pub website: Option<String>,
    pub whitepaper: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Record)]
pub struct AggregatedCoinInformation {
    pub name: String,
    pub symbol: String,
    pub rank: i64,
    pub rate: f64,
    pub color: String,
    pub png64: String,
}

impl AggregatedCoinInformation {
    pub fn new(
        name: String,
        symbol: String,
        rank: i64,
        rate: f64,
    ) -> Self {
        Self {
            name,
            symbol,
            rank,
            rate,
            color: "".into(),
            png64: "".into(),
        }
    }
}
