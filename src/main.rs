mod commands;

use std::env;
// use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::application::{ Command, Interaction };
use serenity::async_trait;
use serenity::builder::{ CreateInteractionResponse, CreateInteractionResponseMessage };
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Logged in as: {}", ready.user.name);
        // register commands
        let _ = Command::create_global_command(
            &ctx.http,
            commands::ping::register()
        ).await;
        let _ = Command::create_global_command(
            &ctx.http,
            commands::id::register()
        ).await;
        println!("Slash commands registered.");
    }
    
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "!hello" {
    //         if let Err(cause) = msg.channel_id.say(&ctx.http, "Hello World").await {
    //             println!("unable to send msg. reason: {:?}", cause);
    //         }
    //     }
    // }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("interaction: {command:#?}");
            // handle different commands
            let content = match command.data.name.as_str() {
                "ping" => Some(commands::ping::run(&command.data.options())),
                "id" => Some(commands::id::run(&command.data.options())),
                _ => Some("not implemented".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(cause) = command.create_response(&ctx.http, builder).await {
                    println!("unable to respond to slash command {cause}");
                }
            }
        }
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