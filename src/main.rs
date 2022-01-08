use std::env;

use serenity::prelude::*;
use serenity::{
    async_trait,
    model::gateway::Ready,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: Ready) {
        let owner = ctx.http.get_current_application_info().await.unwrap().owner;
        let private_channel = owner.create_dm_channel(&ctx).await.unwrap();
        private_channel.say(&ctx.http, "起動しました！").await.unwrap();
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("discord_bot_token").unwrap();
    
    let mut client = Client::builder(&token).event_handler(Handler).await.unwrap();
    
    client.start().await.unwrap();
}
