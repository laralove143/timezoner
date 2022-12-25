#![warn(
    clippy::nursery,
    clippy::pedantic,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    warnings,
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

use std::{
    env,
    error::Error,
    fmt::{Display, Formatter},
    sync::Arc,
};

use anyhow::{anyhow, Result};
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
    application::command::Command,
    gateway::{event::Event, Intents},
    id::{marker::CommandMarker, Id},
};
use twilight_standby::Standby;

use crate::interaction::set_commands;

mod database;
mod interaction;
mod message;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CustomError {
    BadTimezone,
    MissingTimezone(Id<CommandMarker>),
    MessageTooLong,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("a user error has been handled like an internal error")
    }
}

impl Error for CustomError {}

#[derive(Debug)]
pub struct Context {
    bot: Bot,
    db: PgPool,
    standby: Standby,
    commands: Vec<Command>,
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
        env::var("TIMEZONER_BOT_TOKEN")?,
        Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILD_MESSAGE_REACTIONS,
        EventTypeFlags::INTERACTION_CREATE
            | EventTypeFlags::MESSAGE_CREATE
            | EventTypeFlags::REACTION_ADD,
    )
    .await?;
    bot.set_logging_channel(env::var("LOGGING_CHANNEL_ID")?.parse()?)
        .await?;
    bot.set_logging_file("timezoner_errors.txt".to_owned());

    let db = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::migrate!().run(&db).await?;

    let commands = set_commands(&bot).await?;

    let ctx = Arc::new(Context {
        bot,
        db,
        standby: Standby::new(),
        commands,
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

fn err_reply(err: &anyhow::Error) -> Result<Reply> {
    let message = if let Some(user_err) = err.user() {
        match user_err {
            UserError::MissingPermissions(permissions) => format!(
                "Please beg the mods to give me these permissions first:\n{}",
                permissions.ok()?.prettify()
            ),
            UserError::Ignore => {
                return Err(anyhow!("tried to handle an error that should be ignored"))
            }
        }
    } else if let Some(custom_err) = err.downcast_ref::<CustomError>() {
        match custom_err {
            CustomError::BadTimezone => {
                "I looked and looked but couldn't find that timezone anywhere... If you're sure \
                 the timezone is right, please join the support server"
            }
            .to_owned(),
            CustomError::MissingTimezone(timezone) => format!(
                "Bad news, I need to know your timezone first, good news, it's really easy to \
                 tell me, just press </timezone:{timezone}> and smash that send button"
            ),
            CustomError::MessageTooLong => "That message is too long, maybe you're using your \
                                            super nitro powers or it's right at the edge of the \
                                            character limit"
                .to_owned(),
        }
    } else {
        "Something went terribly wrong there... I spammed Lara with the error, I'm sure they'll \
         look at it ASAP"
            .to_owned()
    };

    Ok(Reply::new().ephemeral().content(message))
}
