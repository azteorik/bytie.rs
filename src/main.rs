use poise::serenity_prelude as serenity;

pub mod usdtry;

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;


/// Responds with the USD/TRY parity
#[poise::command(slash_command)]
async fn usdtry(ctx: Context<'_>) -> Result<(), Error> {
    let parity = usdtry::get_usd_try().await;
    ctx.say(parity.get(0).unwrap()).await?;
    Ok(())
}


/// Responds with "Pong"
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// Responds with a brief introduction to the bot
#[poise::command(slash_command)]
async fn bytie(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Hey, I am the Rust version of the Bytie bot, originally created as a Python bot during the worldwide COVID-19 pandemic.").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), bytie(), usdtry()], // Add the commands to the framework
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
