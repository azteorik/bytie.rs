use crate::context::{Context, Error};

/// Responds with a brief introduction to the bot
#[poise::command(slash_command)]
pub async fn bytie(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hey, I am the Rust version of the Bytie bot, originally created as a Python bot during the worldwide COVID-19 pandemic.").await?;
    Ok(())
}