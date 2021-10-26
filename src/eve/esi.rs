use rfesi::prelude::*;
use crate::config::Config;

pub fn create_esi(config: Config) -> Esi {
    EsiBuilder::new()
        .user_agent("EveMarketeer")
        .client_id(&config.esi_client_id)
        .client_secret(&config.discord_client_secret)
        .callback_url(&config.callback_url)
        .build()
        .expect("Couldn't build Esi struct")
}