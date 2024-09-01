use crate::context::{Context, Error};
use poise::serenity_prelude as serenity;

/// Render a LaTeX formula
#[poise::command(slash_command)]
pub async fn latex(
    ctx: Context<'_>,
    #[description = "laTeX formula"] formula: String,
) -> Result<(), Error> {
    let image_url = format!("https://latex.codecogs.com/png.latex?{}", formula);
    let title = format!("LaTeX formula: {}", formula);
    let reply: poise::CreateReply = poise::CreateReply::default()
        .content(String::new())
        .embed(serenity::CreateEmbed::new().title(title).image(image_url));
    ctx.send(reply).await?;
    Ok(())
}
