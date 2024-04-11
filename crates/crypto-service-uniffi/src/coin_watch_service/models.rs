use std::fmt::Debug;

use serde::{Deserialize, Serialize};
use uniffi::{Enum, Record};

#[derive(Serialize, Deserialize, Debug, Clone, Record)]
pub struct ListOfCoinsRequest {
    currency: String,
    sort: String,
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

impl CoinHistoryRequest {
    pub fn new(
        code: String,
        start: u64,
        end: u64,
        meta: bool,
    ) -> Self {
        Self {
            currency: "USD".into(),
            code,
            start,
            end,
            meta,
        }
    }
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
            sort: "rank".into(),
            order: "ascending".into(),
            offset: 0,
            limit,
            meta: true,
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

#[derive(Debug, Serialize, Deserialize, Record, Clone)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Record)]
#[serde(rename_all = "camelCase")]
pub struct CoinMeta {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub rank: Option<i64>,
    pub color: Option<String>,
    pub png64: Option<String>,
    pub webp64: Option<String>,
    #[serde(rename = "allTimeHighUSD")]
    pub all_time_high_usd: Option<f64>,
    pub code: Option<String>,
    pub rate: Option<f64>,
    pub delta: Option<Delta>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Record)]
#[serde(rename_all = "camelCase")]
pub struct CoinHistory {
    pub code: Option<String>,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub rank: Option<i64>,
    pub color: Option<String>,
    pub png64: Option<String>,
    pub webp64: Option<String>,
    #[serde(rename = "allTimeHighUSD")]
    pub all_time_high_usd: Option<f64>,
    pub links: Option<Links>,
    pub history: Option<Vec<History>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Record)]
pub struct History {
    pub date: Option<i64>,
    pub rate: Option<f64>,
    pub volume: Option<i64>,
    pub cap: Option<i64>,
    pub liquidity: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Record)]
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

#[cfg(test)]
mod tests {
    use crate::coin_watch_service::models::{
        AggregatedCoinInformation, Coin,
        CoinHistoryRequest, CoinMetaRequest, Delta,
        ListOfCoinsRequest,
    };

    #[test]
    fn new_coin_history_request_currency() {
        assert_eq!(
            CoinHistoryRequest::new(
                "BTC".into(),
                123456789,
                123456799,
                false
            )
            .currency,
            "USD".to_string()
        );
    }

    #[test]
    fn new_list_of_coins_request_limit() {
        assert_eq!(ListOfCoinsRequest::new(15).limit, 15);
    }

    #[test]
    fn new_list_of_coins_request_sort_rank() {
        assert_eq!(
            ListOfCoinsRequest::new(15).sort,
            "rank"
        );
    }

    #[test]
    fn new_list_of_coins_request_currency() {
        assert_eq!(
            ListOfCoinsRequest::new(15).currency,
            "USD"
        );
    }

    #[test]
    fn new_list_of_coins_request_order_ascending() {
        assert_eq!(
            ListOfCoinsRequest::new(15).order,
            "ascending"
        );
    }

    #[test]
    fn new_delta_hour() {
        assert_eq!(
            Delta::new(
                Some(12.0),
                Some(123.0),
                Some(1234.0),
                Some(12345.0),
                Some(123456.0),
                Some(1234567.0)
            )
            .hour,
            Some(12.0)
        );
    }

    #[test]
    fn new_coin() {
        assert_eq!(
            Coin::new(
                Some("BTC".into()),
                Some(1234.0),
                Some(1234567),
                Some(12),
                Delta::new(
                    Some(12.0),
                    Some(123.0),
                    Some(1234.0),
                    Some(12345.0),
                    Some(123456.0),
                    Some(1234567.0)
                )
            )
            .code,
            Some("BTC".into())
        );
    }

    #[test]
    fn new_coin_meta_request_currency() {
        assert_eq!(
            CoinMetaRequest::new("BTC".into()).currency,
            "USD".to_string()
        );
    }

    #[test]
    fn new_aggregated_coin_information_png64() {
        assert_eq!(
            AggregatedCoinInformation::new(
                "Bitcoin".to_string(),
                "BTC".to_string(),
                0,
                123456.0
            )
            .png64,
            ""
        );
    }

    #[test]
    fn new_aggregated_coin_information_color() {
        assert_eq!(
            AggregatedCoinInformation::new(
                "Bitcoin".to_string(),
                "BTC".to_string(),
                0,
                123456.0
            )
            .color,
            ""
        );
    }
}
