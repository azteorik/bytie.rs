use crate::context::{Context, Error};
use poise::serenity_prelude as serenity;

// Draw a random number between 1 and 6
fn get_random_dice_number() -> u8 {
    let dice_id = rand::random::<u8>() % 6 + 1;
    return dice_id;
}

// Get the URL of the dice image
fn getdiceurl(dice_id: u8) -> String {
    let dice_url: String = match dice_id {
        1 => "https://upload.wikimedia.org/wikipedia/commons/c/c5/Dice-1.png".to_string(),
        2 => "https://upload.wikimedia.org/wikipedia/commons/1/18/Dice-2.png".to_string(),
        3 => "https://upload.wikimedia.org/wikipedia/commons/7/70/Dice-3.png".to_string(),
        4 => "https://upload.wikimedia.org/wikipedia/commons/a/a9/Dice-4.png".to_string(),
        5 => "https://upload.wikimedia.org/wikipedia/commons/6/6c/Dice-5.png".to_string(),
        6 => "https://upload.wikimedia.org/wikipedia/commons/5/5c/Dice-6.png".to_string(),
        _ => "It's best not to push your luck too far.".to_string(),
    };

    // Append a question mark and a random number at the end of the URL to prevent caching
    let dice_url_r = format!("{}?{}", dice_url, rand::random::<u32>());

    return dice_url_r;
}

/// Rolls a dice
#[poise::command(slash_command)]
pub async fn dice(ctx: Context<'_>) -> Result<(), Error> {
    let dice_id: u8 = get_random_dice_number();
    let image_url: String = getdiceurl(dice_id);
    let title: String = format!("{}", dice_id);
    let reply: poise::CreateReply = poise::CreateReply::default()
        .content(String::new())
        .embed(serenity::CreateEmbed::new().title(title).image(image_url));
    ctx.send(reply).await?;
    Ok(())
}
