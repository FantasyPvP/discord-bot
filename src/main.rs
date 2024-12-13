#![feature(pattern)]

use dotenv::dotenv;
use serenity::all::{MessageReaction, RoleId, User, UserId};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;
use std::str::pattern::Pattern;
use std::sync::Arc;

struct Handler;

const STEVENIST_ROLE: u64 = 1315038173730181140;
const HERETIC_ROLE: u64 = 1315038448645832767;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let message = msg.content.to_lowercase();
        let guild_id = msg.guild_id;

        if message == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.channel_id == 1315115119096631337 && message.ends_with("?") {
            let _ = msg.react(ctx.clone(), '❗').await;
            let _ = msg.reply(ctx.clone(), "# QUESTION DETECTED").await;
            let _ = msg
                .channel_id
                .say(
                    &ctx.http,
                    format!(
                        "{} Has asked a Question. THIS IS HERESY!",
                        msg.author_nick(&ctx)
                            .await
                            .unwrap_or(msg.author.name.clone())
                    ),
                )
                .await;
        }

        if msg.channel_id == 1315118069101629530 && !msg.is_own(ctx.clone()) {
            if message != "fr" {
                let _ = msg.react(ctx.clone(), '❗').await;
                let _ = msg.reply(ctx.clone(), "NON 'FR' MESSAGE DETECTED").await;
                let _ = msg
                    .channel_id
                    .say(
                        &ctx.http,
                        format!("{} DIDNT SAY FR!", msg.author.name.clone()),
                    )
                    .await;
            } else {
                let _ = msg.react(ctx.clone(), '✅').await;
            }
        }

        if message.contains("kfc")
            | message.contains("yummy chicken")
            | message.contains("i kill chicken")
        {
            let _ = msg.channel_id.say(&ctx.http, ":exclamation:").await;
            let _ = msg.channel_id.say(&ctx.http, "# HERESY").await;
        }

        if message.contains("# praise steven") {
            let _ = msg.react(ctx.clone(), '✅').await;
        }

        let message = parse(&message);

        if let Some(&"<@1315046876969566218>") = message.get(0) {
            println!("detected");
            if let Some(&"judge") = message.get(1) {
                if let Some(user) = message.get(2) {
                    let user_id_str = user
                        .strip_prefix("<@")
                        .unwrap()
                        .strip_suffix(">")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap();
                    println!("{}", user_id_str);
                    let user_id = UserId::new(user_id_str);
                    if let Ok(user) = &ctx.http.get_member(guild_id.unwrap(), user_id).await {
                        if user.roles.contains(&RoleId::new(HERETIC_ROLE)) {
                            let _ = msg.reply(&ctx, "Heretic").await;
                        } else if user.roles.contains(&RoleId::new(STEVENIST_ROLE)) {
                            let _ = msg.reply(&ctx, "A Loyal Servant of Steven.").await;
                        } else {
                            let _ = msg.reply(&ctx, "Not a Heretic").await;
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token =
        env::var("DISCORD_TOKEN").expect("Please set the DISCORD_TOKEN environment variable");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

fn judge(ctx: Context, msg: Message) {}

fn parse(message: &str) -> Vec<&str> {
    message
        .split(" ")
        .filter(|x| x != &"")
        .collect::<Vec<&str>>()
}
