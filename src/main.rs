use std::env;

extern crate dotenvy;
extern crate tokio;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;
use serenity::builder::*;

use regex::Regex;

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("y!ping") {
            let ping = Timestamp::now().timestamp_millis() - msg.timestamp.timestamp_millis();
            let embed = CreateEmbed::new()
                .title("Pong!")
                .description(format!("Ping: {}", ping));
            let builder = CreateMessage::new().embed(embed);

            if let Err(err) = msg.channel_id.send_message(&ctx.http, builder).await {
                println!("ERROR: {err}");
            }
        }

        let re = Regex::new(r"https://(x|twitter).com/.*/status/.*").unwrap();
        if re.is_match(&msg.content) {
            let re = Regex::new(r"(x|twitter).com").unwrap();
            let replaced_url = re.replace_all(&msg.content, "vxtwitter.com");
            if let Err(err) = msg.channel_id.say(&ctx, replaced_url).await {
                println!("ERROR: {err}");
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").unwrap();

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error on creating client");

    if let Err(err) = client.start().await {
        println!("ERROR: {err}");
    }
}
