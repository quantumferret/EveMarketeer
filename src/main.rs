mod commands;
mod eve;
mod config;

use ron::ser::to_string_pretty;
use commands::{ping::*};
use serenity::client::{Client, Context, EventHandler};
use serenity::{
    async_trait,
    model::{gateway::Ready, event::ResumedEvent},
    prelude::*
};
use serenity::framework::standard::{StandardFramework, macros::group};
use config::Config;

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
    let pretty = ron::ser::PrettyConfig::new();
    let pretty_config = to_string_pretty(&config, pretty)
        .expect("Prettification failed");
    println!("{}", pretty_config);
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))   // set prefix
        .group(&GENERAL_GROUP);

    // Login with bot token
    let token = config.discord_token;
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting as ingle shard
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}