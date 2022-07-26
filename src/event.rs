use std::sync::Arc;

use anyhow::Result;
use twilight_gateway::Event;
use twilight_webhook::cache::PermissionsSource;

use crate::{interaction, parse, Context};

/// handles the event, prints the returned error to stderr and tells the owner
#[allow(clippy::print_stderr)]
pub async fn handle(ctx: Context, event: Event) {
    if let Err(err) = _handle(Arc::clone(&ctx), event).await {
        ctx.error_handler.handle(&ctx.http, err).await;
    }
}

/// handles the event, passing on the returned error
#[allow(clippy::wildcard_enum_match_arm)]
pub async fn _handle(ctx: Context, event: Event) -> Result<()> {
    ctx.webhooks
        .update(
            &event,
            &ctx.http,
            PermissionsSource::Cached {
                cache: &ctx.cache,
                current_user_id: ctx.user_id,
            },
        )
        .await?;

    match event {
        Event::InteractionCreate(interaction) => interaction::handle(ctx, (*interaction).0).await?,
        Event::MessageCreate(message) => parse::send_time(ctx, (*message).0).await?,
        _ => (),
    }
    Ok(())
}
