use std::env;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::async_trait;
use serenity::prelude::*;

struct Handler;

// implement EventHandler to handle Discord event
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(cause) = msg.channel_id.say(&ctx.http, "Hello World").await {
                println!("unable to send msg. reason: {:?}", cause);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("Logged in as: {}", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let discord_bot_token = env::var("DISCORD_BOT_TOKEN")
        .expect("missing bot token");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&discord_bot_token, intents)
        .event_handler(Handler)
        .await
        .expect("unable to create Discord client");
    if let Err(cause) = client.start().await {
        println!("error, cause: {:?}", cause);
    }
}