use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListOfCoinsRequest {
    currency: String,
    sort: String,
    order: String,
    offset: u8,
    limit: u32, 
    meta: bool
}

impl ListOfCoinsRequest {
    pub fn new(limit: u32) -> Self {
        Self { currency: "USD".into(), sort: "rank".into(), order: "descending".into(), offset: 0, limit: limit, meta: false }
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
        Self { hour, day, week, month, quarter, year }
    }
}

impl Coin {
    pub fn new(code: String, rate: f64, volume: i64, cap: i64, delta: Delta) -> Self {
        Self { code, rate, volume, cap, delta }
    }
}
