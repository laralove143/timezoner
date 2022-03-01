//! a discord bot that makes timezone conversions really easy

#![warn(clippy::cargo, clippy::nursery, clippy::pedantic, clippy::restriction)]
#![allow(
    clippy::blanket_clippy_restriction_lints,
    clippy::implicit_return,
    clippy::shadow_same,
    clippy::pattern_type_mismatch
)]

/// functions to create and handle commands
mod commands;
/// functions to set up, update and retrieve timezone information from the
/// sqlite database
mod database;
/// functions to handle events
mod events;

use std::{env, sync::Arc};

use anyhow::Result;
use futures::StreamExt;
use sqlx::SqlitePool;
use twilight_gateway::{Cluster, EventTypeFlags, Intents};
use twilight_http::Client;
use twilight_model::id::{marker::ApplicationMarker, Id};

/// arced context data for thread safety
type Context = Arc<ContextValue>;

/// inner data of the context
pub struct ContextValue {
    /// used to make http requests to discord
    http: Client,
    /// used for the user's timezone information
    db: SqlitePool,
    /// used for creating the interaction client
    application_id: Id<ApplicationMarker>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let intents = Intents::empty();
    let event_types = EventTypeFlags::INTERACTION_CREATE;

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

    commands::create(&http, application_id).await?;

    let db = database::new().await?;

    let ctx = Arc::new(ContextValue {
        http,
        db,
        application_id,
    });

    while let Some((_, event)) = events.next().await {
        tokio::spawn(events::handle(Arc::clone(&ctx), event));
    }

    Ok(())
}
