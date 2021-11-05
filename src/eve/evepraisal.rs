use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Debug)]
struct ReqItem {
    type_id: i32,
}

#[derive(Serialize, Debug)]
struct ReqItems {
    #[serde(rename = "market_name")]
    market: String,
    items: Vec<ReqItem>,
}

pub struct EvepraisalClient {
    client: reqwest::Client,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub appraisal: Appraisal,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Appraisal {
    pub created: u64,
    pub kind: String,
    pub market_name: String,
    pub totals: Totals,
    pub items: Vec<Item>,
    pub raw: String,
    pub unparsed: Value,
    pub private: bool,
    pub live: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Totals {
    pub buy: f64,
    pub sell: f64,
    pub volume: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub name: String,
    #[serde(rename = "typeID")]
    pub type_id: i32,
    pub type_name: String,
    pub type_volume: f64,
    pub quantity: u64,
    pub prices: Prices,
    pub meta: Meta,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Prices {
    pub all: All,
    pub buy: Buy,
    pub sell: Sell,
    pub updated: String,
    pub strategy: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct All {
    pub avg: f64,
    pub max: f64,
    pub median: f64,
    pub min: f64,
    pub percentile: f64,
    pub stddev: f64,
    pub volume: u64,
    pub order_count: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Buy {
    pub avg: f64,
    pub max: f64,
    pub median: f64,
    pub min: f64,
    pub percentile: f64,
    pub stddev: f64,
    pub volume: u64,
    pub order_count: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sell {
    pub avg: f64,
    pub max: f64,
    pub median: f64,
    pub min: f64,
    pub percentile: f64,
    pub stddev: f64,
    pub volume: u64,
    pub order_count: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meta {
}

impl EvepraisalClient {
    pub fn new() -> Self {
        let headers = {
            let mut map = header::HeaderMap::new();
            map.insert(
                header::USER_AGENT,
                header::HeaderValue::from_static("https://github.com/quantumferret/EveMarketeer"),
            );
            map.insert(
                header::ACCEPT,
                header::HeaderValue::from_static("application/json"),
            );
            map.insert(
                header::CONTENT_LANGUAGE,
                header::HeaderValue::from_static("en"),
            );
            map
        };
        let client = Client::builder().default_headers(headers).build().unwrap();
        EvepraisalClient { client }
    }

    pub async fn create_appraisal(
        &self,
        market: &str,
        type_id: i32,
    ) -> reqwest::Result<Root> {
        let item = ReqItem { type_id };
        let formatted_market = market.to_lowercase();
        let data = ReqItems {
            market: formatted_market,
            items: vec![item],
        };
        self.client
            .post("https://evepraisal.com/appraisal/structured.json")
            .json(&data)
            .send()
            .await?
            .json::<Root>()
            .await
    }
}
