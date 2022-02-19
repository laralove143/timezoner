mod commands;
mod database;
mod events;

use std::{env, sync::Arc};

use anyhow::Result;
use futures::StreamExt;
use sqlx::SqlitePool;
use twilight_gateway::{Cluster, EventTypeFlags, Intents};
use twilight_http::Client;
use twilight_model::id::marker::ApplicationMarker;
use twilight_model::id::Id;

type Context = Arc<ContextValue>;

pub struct ContextValue {
    http: Client,
    db: SqlitePool,
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
        tokio::spawn(events::handle(ctx.clone(), event));
    }

    Ok(())
}
