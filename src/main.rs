//! a discord bot that makes timezone conversions really easy

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::shadow_same,
    clippy::pattern_type_mismatch
)]

/// functions to set up, update and retrieve timezone information from the
/// sqlite database
mod database;
/// functions to handle events
mod event;
/// functions to create and handle interaction
mod interaction;
/// functions to parse date/time from strings and format them into discord's
/// epoch formatting
mod parse;

use std::{env, sync::Arc};

use anyhow::Result;
use futures::StreamExt;
use sqlx::SqlitePool;
use twilight_cache_inmemory::{InMemoryCache, ResourceType};
use twilight_gateway::{Cluster, EventTypeFlags, Intents};
use twilight_http::Client;
use twilight_model::id::{
    marker::{ApplicationMarker, UserMarker},
    Id,
};

/// arced context data for thread safety
type Context = Arc<ContextValue>;

/// inner data of the context
pub struct ContextValue {
    /// used to make http requests to discord
    http: Client,
    /// used to check send messages permissions
    cache: InMemoryCache,
    /// used for the user's timezone information
    db: SqlitePool,
    /// used for creating the interaction client
    application_id: Id<ApplicationMarker>,
    /// used for permissions cache
    user_id: Id<UserMarker>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let intents = Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT | Intents::GUILDS;
    let event_types = EventTypeFlags::INTERACTION_CREATE
        | EventTypeFlags::MESSAGE_CREATE
        | EventTypeFlags::GUILD_CREATE
        | EventTypeFlags::GUILD_UPDATE
        | EventTypeFlags::GUILD_DELETE
        | EventTypeFlags::ROLE_CREATE
        | EventTypeFlags::ROLE_UPDATE
        | EventTypeFlags::ROLE_DELETE
        | EventTypeFlags::CHANNEL_CREATE
        | EventTypeFlags::CHANNEL_UPDATE
        | EventTypeFlags::CHANNEL_DELETE
        | EventTypeFlags::MEMBER_ADD
        | EventTypeFlags::MEMBER_CHUNK
        | EventTypeFlags::MEMBER_UPDATE
        | EventTypeFlags::MEMBER_REMOVE;
    let resource_types =
        ResourceType::GUILD | ResourceType::CHANNEL | ResourceType::MEMBER | ResourceType::ROLE;

    let token = env::var("TIMEZONER_BOT_TOKEN")?;

    let (cluster, mut events) = Cluster::builder(token.clone(), intents)
        .event_types(event_types)
        .build()
        .await?;
    let cluster_spawn = Arc::new(cluster);
    tokio::spawn(async move { cluster_spawn.up().await });

    let http = Client::new(token);
    let application_id = http
        .current_user_application()
        .exec()
        .await?
        .model()
        .await?
        .id;
    let user_id = http.current_user().exec().await?.model().await?.id;

    interaction::create(&http, application_id).await?;

    let db = database::new().await?;
    let cache = InMemoryCache::builder()
        .resource_types(resource_types)
        .build();

    let ctx = Arc::new(ContextValue {
        http,
        cache,
        db,
        application_id,
        user_id,
    });

    while let Some((_, event)) = events.next().await {
        ctx.cache.update(&event);
        tokio::spawn(event::handle(Arc::clone(&ctx), event));
    }

    Ok(())
}
