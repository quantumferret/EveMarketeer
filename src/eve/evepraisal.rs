use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct ReqItem {
    name: String,
    type_id: i32,
}

pub struct EvepraisalClient {
    client: reqwest::Client,
}

impl EvepraisalClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        EvepraisalClient { client }
    }

    pub async fn create_appraisal(&self, name: &str, type_id: i32) -> reqwest::Result<Value> {
        let body = ReqItem {
            name: String::from(name),
            type_id,
        };
        self.client
            .post("https://evepraisal.com/appraisal/structured.json")
            .json(&body)
            .send()
            .await
            .unwrap()
            .json::<Value>()
            .await
    }
}
