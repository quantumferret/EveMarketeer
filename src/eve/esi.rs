use reqwest::{header, Client};
use serde::Deserialize;
use serde_json::Value;
use std::time::Duration;

const BASE_URL: &str = "https://esi.evetech.net/";

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
        EsiStruct {
            client,
        }
    }

    pub async fn search_item(
        &self,
        search: &str,
        is_strict: bool,
    ) -> reqwest::Result<SearchResult> {
        let strict = match is_strict {
            true => "true",
            false => "false",
        };
        let endpoint = format!("{}{}{}", BASE_URL, "latest", "/search/");
        self.client
            .get(endpoint)
            .query(&[
                ("categories", "inventory_type"),
                ("datasource", "tranquility"),
                ("language", "en"),
                ("search", search),
                ("strict", strict),
            ])
            .send()
            .await
            .unwrap()
            .json::<SearchResult>()
            .await
    }

    pub async fn get_type_information(&self, type_id: i32) -> reqwest::Result<Value> {
        let endpoint = format!("{}{}{}{}/", BASE_URL, "latest", "/universe/types/", type_id);
        self.client
            .get(endpoint)
            .query(&[("datasource", "tranquility"), ("language", "en")])
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
    }
}
