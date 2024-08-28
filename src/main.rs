use fal_rust::{
    client::{ClientCredentials, FalClient},
    utils::download_image,
};
use serenity::async_trait;
use serenity::builder::{CreateAttachment, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::env;

struct Handler {
    fal_client: FalClient,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!imagine ") {
            let prompt = msg.content.trim_start_matches("!imagine ").to_string();
            if let Err(why) = msg.channel_id.say(&ctx.http, "Generating image...").await {
                println!("Error sending message: {:?}", why);
                return;
            }
            match self.generate_image(&prompt).await {
                Ok(filename) => {
                    let attachment = CreateAttachment::path(&filename).await;
                    if let Err(why) = attachment {
                        println!("Error creating attachment: {:?}", why);
                        return;
                    }
                    let attachment = attachment.unwrap();

                    let message = CreateMessage::default().add_file(attachment);
                    if let Err(why) = msg.channel_id.send_message(&ctx.http, message).await {
                        println!("Error sending message: {:?}", why);
                    }

                }
                Err(e) => {
                    if let Err(why) = msg
                        .channel_id
                        .say(&ctx.http, format!("Error generating image: {}", e))
                        .await
                    {
                        println!("Error sending error message: {:?}", why);
                    }
                }
            }
        } else {
            match msg.content.as_str() {
                "!ping" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                        println!("Error sending message: {:?}", why);
                    }
                },
                "!bytie" => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Hey, I am the Rust version of the Bytie bot, originally created as a Python bot during the worldwide COVID-19 pandemic.").await {
                        println!("Error sending message: {:?}", why);
                    }
                },
                _ => {}
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    async fn generate_image(&self, prompt: &str) -> Result<String, String> {
        let res = self
            .fal_client
            .run(
                "fal-ai/flux/dev",
                serde_json::json!({
                    "prompt": prompt,
                }),
            )
            .await
            .map_err(|e| format!("Fal AI error: {:?}", e))?;
        let output: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("JSON parsing error: {:?}", e))?;
        let url = output["images"][0]["url"]
            .as_str()
            .ok_or_else(|| "Failed to get image URL".to_string())?;
        let filename = url.split('/').last().unwrap_or("generated_image.png");
        let path = format!("images/{}", filename);
        download_image(url, &path)
            .await
            .map_err(|e| format!("Download error: {:?}", e))?;
        Ok(path)
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
    let fal_client = FalClient::new(ClientCredentials::from_env());
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler { fal_client })
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
