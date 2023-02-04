#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use std::{env, sync::Arc};

use anyhow::Result;
use dotenvy::dotenv;
use futures::stream::StreamExt;
use sparkle_convenience::{
    error::{conversion::IntoError, ErrorExt, UserError},
    prettify::Prettify,
    reply::Reply,
    Bot,
};
use sqlx::PgPool;
use twilight_gateway::EventTypeFlags;
use twilight_model::{
    gateway::{event::Event, Intents},
    guild::Permissions,
    id::{
        marker::{ChannelMarker, CommandMarker, GuildMarker},
        Id,
    },
};
use twilight_standby::Standby;

use crate::interaction::{set_commands, CommandIds};

mod database;
mod interaction;
mod message;
mod time;

const ACCENT_COLOR: u32 = 0x00d4_f1f9;
const LOGGING_CHANNEL_ID: Id<ChannelMarker> = Id::new(1_002_953_459_890_397_287);
const TEST_GUILD_ID: Id<GuildMarker> = Id::new(903_367_565_349_384_202);
const BOT_INVITE: &str = "https://discord.com/api/oauth2/authorize?\
client_id=909820903574106203&permissions=536947776&scope=bot%20applications.commands";
const SUPPORT_SERVER_INVITE: &str = "https://discord.gg/6vAzfFj8xG";
const REQUIRED_PERMISSIONS: Permissions = Permissions::MANAGE_WEBHOOKS
    .union(Permissions::VIEW_CHANNEL)
    .union(Permissions::SEND_MESSAGES)
    .union(Permissions::MANAGE_MESSAGES)
    .union(Permissions::READ_MESSAGE_HISTORY)
    .union(Permissions::ADD_REACTIONS);

trait HandleExitResult<E> {
    fn handle(self) -> Option<(Reply, Option<E>)>;
}

impl<T> HandleExitResult<anyhow::Error> for Result<T> {
    fn handle(self) -> Option<(Reply, Option<anyhow::Error>)> {
        match self {
            Ok(_) => None,
            Err(err) if err.ignore() => None,
            Err(err) => {
                let reply = err_reply(&err).unwrap();

                if let Some(err) = err.internal::<CustomError>() {
                    Some((reply, Some(err)))
                } else {
                    Some((reply, None))
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    TimezoneParseError(String),
    #[error("unknown command: {0}")]
    UnknownCommand(String),
    #[error("tried to handle an error that should be ignored")]
    IgnoreErrorHandled,
    #[error("time doesn't end in am or pm")]
    Hour12InvalidSuffix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum CustomError {
    #[error(
        "i looked and looked but couldnt find that timezone anywhere... if you're sure the \
         timezone is right, please join the support server"
    )]
    BadTimezone,
    #[error(
        "bad news, i need to know your timezone first, good news, its really easy to tell me, \
         just press </timezone:{0}> and smash that send or enter button"
    )]
    MissingTimezone(Id<CommandMarker>),
    #[error(
        "that message is too long, maybe you're using your super nitro powers or its right at the \
         edge of the character limit"
    )]
    MessageTooLong,
}

#[derive(Debug)]
pub struct Context {
    bot: Bot,
    db: PgPool,
    standby: Standby,
    command_ids: CommandIds,
}

impl Context {
    async fn handle_event(&self, event: Event) -> Result<()> {
        self.standby.process(&event);

        match event {
            Event::InteractionCreate(interaction) => {
                self.handle_interaction(interaction.0).await?;
            }
            Event::MessageCreate(message) => {
                self.handle_message(message.0).await?;
            }
            _ => {}
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    let (mut bot, mut events) = Bot::new(
        env::var("BOT_TOKEN")?,
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGE_REACTIONS,
        EventTypeFlags::INTERACTION_CREATE
            | EventTypeFlags::MESSAGE_CREATE
            | EventTypeFlags::REACTION_ADD,
    )
    .await?;
    bot.set_logging_channel(LOGGING_CHANNEL_ID).await?;
    bot.set_logging_file("log.txt".to_owned());

    let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&db).await?;

    let command_ids = set_commands(&bot).await?;

    let ctx = Arc::new(Context {
        bot,
        db,
        standby: Standby::new(),
        command_ids,
    });

    while let Some((_, event)) = events.next().await {
        let ctx_ref = Arc::clone(&ctx);
        tokio::spawn(async move {
            if let Err(err) = ctx_ref.handle_event(event).await {
                ctx_ref.bot.log(format!("{err:?}")).await;
            };
        });
    }

    Ok(())
}

async fn log(bot: &Bot, err: &anyhow::Error) {
    bot.log(format!("{err:?}")).await;
}

fn err_reply(err: &anyhow::Error) -> Result<Reply> {
    let message = if let Some(user_err) = err.user() {
        match user_err {
            UserError::MissingPermissions(permissions) => format!(
                "please beg the mods to give me these permissions first:\n{}",
                permissions.ok()?.prettify()
            ),
            UserError::Ignore => return Err(Error::IgnoreErrorHandled.into()),
        }
    } else if let Some(custom_err) = err.downcast_ref::<CustomError>() {
        custom_err.to_string()
    } else {
        "something went terribly wrong there... i spammed Lara with the error, im sure they'll \
         look at it asap"
            .to_owned()
    };

    Ok(Reply::new().ephemeral().content(message))
}
