use crate::{Context, Error};
use poise::serenity_prelude::model::permissions::Permissions;
use poise::serenity_prelude::{
    ChannelId, ChannelType, Color, PermissionOverwrite, PermissionOverwriteType, StageInstance, User, UserId,
};
use rand::seq::SliceRandom;

const GREETINGS: &[&str] = &["なんすか", "なんか用すか", "はいはい、こんにちは", "元気だけはあるっすね", "なんかばかみたいっすね", "なんかそういうデータあるんですか？" , "うそはうそであると見抜ける人でないと難しい" ,"レターパックで現金送れ"];

#[derive(Debug, poise::ChoiceParameter)]
pub enum SpaceCommandType {
    Open,
    Close,
}

#[poise::command(prefix_command, hide_in_help)]
pub async fn register(ctx: Context<'_>, #[flag] global: bool) -> Result<(), Error> {
    poise::builtins::register_application_commands(ctx, global).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    let greeting = GREETINGS.choose(&mut rand::thread_rng()).unwrap();
    ctx.say(greeting.to_string()).await?;
    Ok(())
}

fn generate_pseudo_stage_moderator_permission(user_id: UserId) -> Vec<PermissionOverwrite> {
    return vec![
        PermissionOverwrite {
            allow: Permissions::REQUEST_TO_SPEAK,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::MANAGE_CHANNELS,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::MOVE_MEMBERS,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: PermissionOverwriteType::Member(user_id),
        },
        PermissionOverwrite {
            allow: Permissions::MUTE_MEMBERS,
            deny: Permissions::SEND_TTS_MESSAGES,
            kind: PermissionOverwriteType::Member(user_id),
        },
    ];
}

async fn open_space(ctx: Context<'_>) -> Result<(), Error> {
    let mut exist = false;
    ctx.clone()
        .guild()
        .unwrap()
        .channels
        .into_iter()
        .for_each(|channel| match channel.1.guild() {
            Some(guild) => {
                if guild.name == format!("{}のスペース", ctx.author().name) {
                    exist = true;
                }
            }
            None => {
                print!("NONE!")
            }
        });

    if exist {
        ctx.send(|message| {
            message.embed(|embed| {
                embed.author(|author| {
                    author.icon_url(ctx.author().face());
                    author.name(&ctx.author().name)
                });
                embed.title("もう既に開いてるっす。");
                embed.color(Color::RED);
                embed
            })
        })
        .await?;
        return Ok(());
    }

    let _ = ctx
        .guild_id()
        .unwrap()
        .create_channel(ctx, |c| {
            c.name(format!("{}のスペース", ctx.author().name))
                .kind(ChannelType::Stage)
                .permissions(generate_pseudo_stage_moderator_permission(ctx.author().id))
                .category(ChannelId(1124968492991524864))
        })
        .await;

    ctx.send(|message| {
        message.embed(|embed| {
            embed.author(|author| {
                author.icon_url(ctx.author().face());
                author.name(&ctx.author().name)
            });
            embed.title("スペースを作ったっすよ");
            embed.color(Color::BLUE);
            embed
        })
    })
    .await?;
    return Ok(());
}

async fn close_space(ctx: Context<'_>) -> Result<(), Error> {
    let target = ctx
        .clone()
        .guild()
        .unwrap()
        .channels
        .into_iter()
        .find(|channel| match channel.clone().1.guild() {
            Some(guild) => guild.name == format!("{}のスペース", ctx.author().name),
            None => false,
        });
    match target {
        Some(target) => {
            target.1.delete(ctx).await?;
            ctx.send(|message| {
                message.embed(|embed| {
                    embed.author(|author| {
                        author.icon_url(ctx.author().face());
                        author.name(&ctx.author().name)
                    });
                    embed.title("スペースを閉じたっす");
                    embed.color(Color::BLUE);
                    embed
                })
            })
            .await?;
            Ok(())
        }
        None => {
            ctx.send(|message| {
                message.embed(|embed| {
                    embed.author(|author| {
                        author.icon_url(ctx.author().face());
                        author.name(&ctx.author().name)
                    });
                    embed.title("あなたはスペース開いてないっす");
                    embed.color(Color::RED);
                    embed
                })
            })
            .await?;
            Ok(())
        }
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn space(
    ctx: Context<'_>,
    #[description = "サブコマンド"] sub_command: SpaceCommandType,
) -> Result<(), Error> {
    match sub_command {
        SpaceCommandType::Open => open_space(ctx).await,
        SpaceCommandType::Close => close_space(ctx).await
    }
}
