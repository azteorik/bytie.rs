use crate::context::{Context, Error};
use poise::serenity_prelude as serenity;
use reqwest;
use serde_json::Value;

struct XKCDComic {
    title: String,
    image_url: String,
}

/// Responds with the XKCD comic with the given ID
#[poise::command(slash_command)]
pub async fn xkcd(ctx: Context<'_>, #[description = "Comic ID"] id: u32) -> Result<(), Error> {
    // Fetch the XKCD image URL
    let xkcd = fetch_xkcd(id).await?;

    let reply = poise::CreateReply::default().content(String::new()).embed(
        serenity::CreateEmbed::new()
            .title(&xkcd.title)
            .image(xkcd.image_url),
    );
    ctx.send(reply).await?;
    Ok(())
}

async fn fetch_xkcd(id: u32) -> Result<XKCDComic, Box<dyn std::error::Error + Send + Sync>> {
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
    let title = data["title"].as_str().ok_or("Title not found")?.to_string();
    Ok(XKCDComic { title, image_url })
}
