mod commands;
mod config;
mod eve;

use commands::ping::*;
use config::Config;
use eve::esi;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{macros::group, StandardFramework};
use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
};

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn resume(&self, _ctx: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

#[tokio::main]
async fn main() {
    let config_path = format!("{}/config.ron", env!("CARGO_MANIFEST_DIR"));
    let config: Config = Config::new_from_ron_file(config_path);
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/")) // set prefix
        .group(&GENERAL_GROUP);

    // Login with bot token
    let token = config.discord_token.as_str();
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    let evepraisal = eve::evepraisal::EvepraisalClient::new();

    let esi = esi::EsiStruct::new();
    let res = esi
        .search_item("Rifter", true)
        .await
        .expect("Couldn't deserialize search result");
    println!("{:?}", res);

    let search_id = res.search_ids[0];
    let type_info = esi
        .get_type_information(search_id)
        .await
        .expect("Couldn't deserialize universe/type/{search_id} response");

    println!("{}", serde_json::to_string_pretty(&type_info).unwrap());

    let name = type_info["name"].as_str().unwrap();

    let appraisal = evepraisal
        .create_appraisal(name, search_id)
        .await
        .expect("Couldn't deserialize evepraisal create_appraisal response");
    println!("{}", serde_json::to_string_pretty(&appraisal).unwrap());

    // start listening for events by starting as single shard
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
