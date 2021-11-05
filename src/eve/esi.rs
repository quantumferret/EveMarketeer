use reqwest::{header, Client};
use serde::Deserialize;
use std::time::Duration;

const SEARCH_ENDPOINT: &str = "https://esi.evetech.net/latest/search/";

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    #[serde(rename = "inventory_type")]
    pub search_ids: Vec<i32>,
}

pub struct EsiStruct {
    client: reqwest::Client,
}

impl EsiStruct {
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
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .default_headers(headers)
            .build()
            .unwrap();
        EsiStruct { client }
    }

    pub async fn search_item(
        &self,
        search: &str,
        is_strict: bool,
    ) -> reqwest::Result<SearchResult> {
        self.client
            .get(SEARCH_ENDPOINT)
            .query(&[
                ("categories", "inventory_type"),
                ("datasource", "tranquility"),
                ("language", "en"),
                ("search", search),
                ("strict", &is_strict.to_string()),
            ])
            .send()
            .await?
            .json::<SearchResult>()
            .await
    }
}