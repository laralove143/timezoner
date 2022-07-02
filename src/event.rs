use std::sync::Arc;

use anyhow::Result;
use twilight_gateway::Event;

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
    match event {
        Event::InteractionCreate(interaction) => interaction::handle(ctx, interaction.0).await?,
        Event::WebhooksUpdate(webhooks) => {
            ctx.webhooks
                .validate(
                    &ctx.http,
                    webhooks.channel_id,
                    ctx.cache
                        .permissions()
                        .in_channel(ctx.user_id, webhooks.channel_id)?,
                )
                .await?;
        }
        Event::MessageCreate(message) => parse::send_time(ctx, (*message).0).await?,
        _ => (),
    }
    Ok(())
}
