#![warn(clippy::str_to_string)]

mod commands;

use dotenv::dotenv;
use poise::serenity_prelude::{self as serenity, GuildId};
use serenity::{GatewayIntents};
use std::{collections::HashMap, env, sync::Mutex};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(thiserror::Error, Debug)]
enum AppError {
    #[error("{0}")]
    Serenity(#[from] serenity::Error),
}

pub struct Data {
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx } => {
            println!("Error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error while handling error: {}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let options = poise::FrameworkOptions {
        commands: vec![commands::register(), commands::hello(), commands::space()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("+!".into()),
            ..Default::default()
        },
        on_error: |err| Box::pin(on_error(err)),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(env::var("TOKEN").expect("Missing Token!!!!"))
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .intents(
            GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}
