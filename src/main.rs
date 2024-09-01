use poise::serenity_prelude as serenity;

mod context;
use crate::context::Data;
mod bytie;
mod dice;
mod imagine;
mod lisp;
mod ping;
mod stock;
mod usdtry;
mod xkcd;
mod collatz;
mod latex;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                ping::ping(),
                bytie::bytie(),
                usdtry::usdtry(),
                stock::stock(),
                xkcd::xkcd(),
                lisp::lisp(),
                imagine::imagine(),
                dice::dice(),
                collatz::collatz(),
                latex::latex(),
            ], // Add the commands to the framework
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::non_privileged())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
