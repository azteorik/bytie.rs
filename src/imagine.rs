use crate::context::{Context, Error};
use fal_rust::client::{ClientCredentials, FalClient};
use poise::serenity_prelude as serenity;

async fn fetch_image_url(prompt: &str) -> Result<String, Error> {
    let fal_client = FalClient::new(ClientCredentials::from_env());
    let res = fal_client
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
    return Ok(url.to_string());
}

#[poise::command(slash_command)]
pub async fn imagine(
    ctx: Context<'_>,
    #[description = "Prompt for the image"] prompt: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let image_url = fetch_image_url(&prompt).await?;
    let reply = poise::CreateReply::default()
        .content(String::new())
        .embed(serenity::CreateEmbed::new().title(prompt).image(image_url));
    ctx.send(reply).await?;
    Ok(())
}
