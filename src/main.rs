mod command_parser;
mod config;
mod eve;

#[macro_use]
extern crate lazy_static;
extern crate inflector;
extern crate round;

use command_parser::*;
use config::Config;
use eve::{esi, evepraisal};
use inflector::Inflector;
use round::round;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::command, macros::group, Args, CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::utils::Content;
use serenity::utils::ContentModifier::{Bold, Code};
use serenity::{
    async_trait,
    model::{event::ResumedEvent, gateway::Ready},
};
use thousands::Separable;
use crate::eve::evepraisal::Item;

#[group]
#[commands(ping, price, item)]
struct General;

struct Handler;

lazy_static! {
    static ref ESI: esi::EsiStruct = esi::EsiStruct::new();
    static ref EP: evepraisal::EvepraisalClient = evepraisal::EvepraisalClient::new();
}

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

    // start listening for events by starting as single shard
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "pong!").await?;
    Ok(())
}

#[command]
pub async fn price(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let arg_string: &str = &args.raw().collect::<Vec<&str>>().join(" ");
    let parsed = parse_price_args(arg_string);

    let res = ESI
        .search_item(
            parsed
                .get("item")
                .expect("Couldn't get item from parsed input"),
            true,
        )
        .await?;

    let msg = if res.search_ids.len() > 0 {
        let search_id = res.search_ids[0];
        let market = parsed.get(&"market").unwrap();
        let appraisal = &EP
            .create_appraisal(market, search_id)
            .await?
            .appraisal;
        let items = &appraisal.items;
        let item = &items[0];

        price_reply(item.type_id, &appraisal.market_name, item, ctx, msg).await
    } else {
        error_reply(ctx, msg).await
    };

    Ok(())
}

#[command]
pub async fn item(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    unimplemented!()
}

async fn price_reply(
    type_id: i32,
    market_name: &str,
    item: &Item,
    ctx: &Context,
    msg: &Message,
) -> serenity::Result<Message> {
    let sell_min: Content = Code
        + (round(item.prices.sell.min, 2).separate_with_commas()
            + " ISK");
    let sell_avg: Content = Code
        + (round(item.prices.sell.avg, 2).separate_with_commas()
            + " ISK");
    let sell_field = format!(
        "• Lowest: {}\n• Average: {}",
        sell_min.to_string(),
        sell_avg.to_string()
    );

    let buy_max: Content = Code
        + (round(item.prices.buy.max, 2).separate_with_commas()
            + " ISK");
    let buy_avg: Content = Code
        + (round(item.prices.buy.avg, 2).separate_with_commas()
            + " ISK");
    let buy_field = format!(
        "• Highest: {}\n• Average: {}",
        buy_max.to_string(),
        buy_avg.to_string()
    );

    let mut author = serenity::builder::CreateEmbedAuthor::default();
    author.name(&item.type_name);
    author.icon_url("https://data.saturnserver.org/eve/Icons/UI/WindowIcons/wallet.png");

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.set_author(author);
                e.description(format!(
                    "Price information for {}",
                    (Bold + market_name.to_title_case()).to_string()
                ));
                e.thumbnail(format!("https://images.evetech.net/types/{}/icon", type_id));
                e.field(
                    format!(
                        "Sell ( {} orders, {} items )",
                        &item.prices.sell.order_count.separate_with_commas(),
                        &item.prices.sell.volume.separate_with_commas(),
                    ),
                    sell_field,
                    false,
                );
                e.field(
                    format!(
                        "Buy ( {} orders, {} items )",
                        &item.prices.buy.order_count.separate_with_commas(),
                        &item.prices.buy.volume.separate_with_commas(),
                    ),
                    buy_field,
                    false,
                );

                e
            });
            m
        })
        .await
}

async fn error_reply(ctx: &Context, msg: &Message) -> serenity::Result<Message> {
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("No data");
                e.description("Request failed");
                e
            });
            m
        })
        .await
}