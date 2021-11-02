use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct ReqItem<'a> {
    name: &'a str,
    type_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqItems<'a> {
    #[serde(rename = "market_name")]
    market: &'a str,
    items: Vec<ReqItem<'a>>,
}

pub struct EvepraisalClient {
    client: reqwest::Client,
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
        name: &str,
        market: &str,
        type_id: i32,
    ) -> reqwest::Result<Value> {
        let item = ReqItem { name, type_id };
        let data = ReqItems {
            market,
            items: vec![item],
        };
        self.client
            .post("https://evepraisal.com/appraisal/structured.json")
            .json(&data)
            .send()
            .await?
            .json::<Value>()
            .await
    }
}
