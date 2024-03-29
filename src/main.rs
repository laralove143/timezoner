#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use std::{env, sync::Arc, time::Duration};

use anyhow::Result;
use dotenvy::dotenv;
use futures::stream::StreamExt;
use sparkle_convenience::{
    error::{ErrorExt, UserError},
    log::DisplayFormat,
    prettify::Prettify,
    reply::Reply,
    Bot,
};
use sqlx::PgPool;
use twilight_gateway::{
    error::ReceiveMessageErrorType, stream::ShardEventStream, EventTypeFlags, MessageSender, Shard,
};
use twilight_model::{
    channel::message::{component::ActionRow, Component},
    gateway::{event::Event, Intents},
    guild::Permissions,
    id::{
        marker::{ChannelMarker, CommandMarker, GuildMarker},
        Id,
    },
};
use twilight_util::builder::embed::{EmbedBuilder, EmbedFooterBuilder};

use crate::{
    interaction::{help::server_button, set_commands, CommandIds},
    metrics::{JsonStorageClient, Metrics},
};

mod database;
mod interaction;
mod message;
mod metrics;
mod time;

const LOGGING_CHANNEL_ID: Id<ChannelMarker> = Id::new(1_002_953_459_890_397_287);
const TEST_GUILD_ID: Id<GuildMarker> = Id::new(903_367_565_349_384_202);

const README_LINK: &str = "https://github.com/laralove143/timezoner/blob/main/README.md#timezoner";
const MISSING_PERMISSIONS_LINK: &str =
    "https://github.com/laralove143/timezoner/blob/main/README.md#missing-permissions";
const SUPPORT_SERVER_INVITE: &str = "https://discord.com/invite/KUMdnjcE97";

const REQUIRED_PERMISSIONS: Permissions = Permissions::MANAGE_WEBHOOKS
    .union(Permissions::VIEW_CHANNEL)
    .union(Permissions::SEND_MESSAGES)
    .union(Permissions::MANAGE_MESSAGES)
    .union(Permissions::READ_MESSAGE_HISTORY)
    .union(Permissions::ADD_REACTIONS);

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("metrics weren't updated: put: {put:?}, got: {get:?}")]
    MetricsUpdateFail { get: Metrics, put: Metrics },
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("couldn't parse timezone from database: {0}")]
    TimezoneParseDatabase(String),
    #[error("couldn't parse detected timezone: {0}")]
    TimezoneParseDetected(String),
    #[error("time doesn't end in am or pm: hour: {hour}, suffix: {suffix}")]
    Hour12InvalidSuffix { hour: u32, suffix: String },
    #[error("message without a time has time detect reaction by the bot")]
    FalseTimeDetectReaction,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
#[rustfmt::skip]
pub enum CustomError {
    #[error(
        "i looked and looked but couldn't find that timezone anywhere :pensive:\n\
        if you're sure the timezone is right, please join the support server"
    )]
    BadTimezone,
    #[error(
        "bad news, i need to know your timezone first :scream:\n\
        good news, its really easy to tell me :relieved:\n\
        just press </timezone:{0}> and smash that send or enter button"
    )]
    MissingTimezone(Id<CommandMarker>),
    #[error(
        "that user hasn't set their timezone yet :rolling_eyes:\n\
        but they can do that using the </timezone:{0}> command"
    )]
    OtherUserMissingTimezone(Id<CommandMarker>),
    #[error(
        "that message is too long :sob:\n\
        maybe you're using your super nitro powers or its right at the edge of the character limit"
    )]
    MessageTooLong,
    #[error(
        "that's not a valid date :rage:\n\
        or maybe it is idk, join the support server if it is!"
    )]
    BadDate,
}

#[derive(Debug)]
pub struct Context {
    bot: Bot,
    shards: Vec<MessageSender>,
    db: PgPool,
    json_storage: JsonStorageClient,
    command_ids: CommandIds,
}

impl Context {
    async fn handle_event(&self, event: Event) {
        match event {
            Event::InteractionCreate(interaction) => self.handle_interaction(interaction.0).await,
            Event::MessageCreate(message) => self.handle_message(message.0).await,
            Event::MessageUpdate(message) => self.handle_message_update(*message).await,
            Event::ReactionAdd(reaction) => self.handle_reaction(reaction.0).await,
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let (mut bot, mut shards) = Bot::new(
        env::var("BOT_TOKEN")?,
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGE_REACTIONS,
        EventTypeFlags::INTERACTION_CREATE
            | EventTypeFlags::MESSAGE_CREATE
            | EventTypeFlags::MESSAGE_UPDATE
            | EventTypeFlags::REACTION_ADD,
    )
    .await?;
    bot.set_logging_format(DisplayFormat::Debug);
    bot.set_logging_channel(LOGGING_CHANNEL_ID).await?;
    bot.set_logging_file("log.txt".to_owned());

    let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;

    let command_ids = set_commands(&bot).await?;

    let ctx = Arc::new(Context {
        bot,
        shards: shards.iter().map(Shard::sender).collect(),
        db,
        json_storage: JsonStorageClient {
            http: reqwest::Client::new(),
            api_key: env::var("JSONSTORAGE_API_KEY")?,
            url: env::var("METRICS_URL")?,
        },
        command_ids,
    });

    let mut metrics_update_interval = tokio::time::interval(Duration::from_secs(60 * 60));
    let ctx_metrics_ref = Arc::clone(&ctx);
    tokio::spawn(async move {
        loop {
            metrics_update_interval.tick().await;
            if let Err(err) = ctx_metrics_ref.update_metrics().await {
                ctx_metrics_ref.bot.log(err).await;
            }
        }
    });

    let mut events = ShardEventStream::new(shards.iter_mut());
    while let Some((_, event_res)) = events.next().await {
        let ctx_event_ref = Arc::clone(&ctx);
        match event_res {
            Ok(event) => {
                tokio::spawn(async move {
                    ctx_event_ref.handle_event(event).await;
                });
            }
            Err(err)
                if !matches!(
                    err.kind(),
                    ReceiveMessageErrorType::Deserializing { .. } | ReceiveMessageErrorType::Io
                ) =>
            {
                ctx_event_ref.bot.log(&err).await;

                if err.is_fatal() {
                    break;
                }
            }
            Err(_) => {}
        };
    }

    Ok(())
}

fn embed() -> EmbedBuilder {
    EmbedBuilder::new()
        .color(0x00d4_f1f9)
        .footer(EmbedFooterBuilder::new(
            "wondering anything? check out the link in my bio!",
        ))
}

fn err_reply(err: &anyhow::Error) -> Reply {
    _err_reply(err, false)
}

fn err_reply_timed(err: &anyhow::Error) -> Reply {
    _err_reply(err, true)
}

fn _err_reply(err: &anyhow::Error, is_timed: bool) -> Reply {
    #[rustfmt::skip]
    const INTERNAL_ERROR_MESSAGE: &str = "something went terribly wrong there :facepalm:\n\
    i spammed lara (the dev) with the error, im sure they'll look at it asap";

    let mut reply = Reply::new();

    let description = if let Some(UserError::MissingPermissions(permissions)) = err.user() {
        format!(
            "please beg the mods to give me these permissions first:\n{}",
            permissions.unwrap_or(REQUIRED_PERMISSIONS).prettify()
        )
    } else if let Some(custom_err) = err.downcast_ref::<CustomError>() {
        if let CustomError::BadTimezone | CustomError::BadDate = custom_err {
            reply = reply.component(Component::ActionRow(ActionRow {
                components: vec![server_button()],
            }));
        }

        if let CustomError::BadTimezone
        | CustomError::MissingTimezone(_)
        | CustomError::MessageTooLong
        | CustomError::BadDate = custom_err
        {
            reply = reply.ephemeral();
        };

        custom_err.to_string()
    } else {
        INTERNAL_ERROR_MESSAGE.to_owned()
    };

    let mut embed = embed()
        .title("❌ catastrophic failure")
        .description(description);

    if is_timed {
        embed = embed.footer(EmbedFooterBuilder::new(
            "🧨 this message will self destruct in a minute",
        ));
    }

    reply.embed(embed.build())
}
