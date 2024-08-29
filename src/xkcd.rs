use crate::context::{Context, Error};
use poise::serenity_prelude as serenity;
use reqwest;
use serde_json::Value;

/// Responds with the XKCD comic with the given ID
#[poise::command(slash_command)]
pub async fn xkcd(ctx: Context<'_>, #[description = "Comic ID"] id: u32) -> Result<(), Error> {
    // Fetch the XKCD image URL
    let image_url = match fetch_xkcd_image_url(id).await {
        Ok(url) => url,
        Err(err) => {
            ctx.say(format!("Failed to fetch XKCD comic: {}", err))
                .await?;
            return Ok(());
        }
    };

    let reply = poise::CreateReply::default()
        .content(format!("XKCD Comic #{}", id))
        .embed(
            serenity::CreateEmbed::new()
                .title(format!("XKCD Comic #{}", id))
                .image(image_url),
        );
    ctx.send(reply).await?;
    Ok(())
}

async fn fetch_xkcd_image_url(id: u32) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let url = format!("https://xkcd.com/{}/info.0.json", id);
    let response = reqwest::get(&url).await?;
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }
    let response_text = response.text().await?;
    let data: Value = serde_json::from_str(&response_text)?;
    // Extract the image URL from the JSON response
    let image_url = data["img"]
        .as_str()
        .ok_or("Image URL not found")?
        .to_string();
    Ok(image_url)
}
