use poise::serenity_prelude as serenity;
use std::env::var;
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
use dotenv::dotenv;

pub struct Data {}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command '{}': {:?}", ctx.command().name, error);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[poise::command(slash_command, prefix_command)]
pub async fn hello_world(ctx: Context<'_>) -> Result<(), Error> {
    poise::say_reply(ctx, "Hello, world!").await?;
    Ok(())
}

pub async fn start_bot() {
    dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("!".into()),
            // edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("token"))
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .options(options)
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        );

    framework.run().await.unwrap();
}
