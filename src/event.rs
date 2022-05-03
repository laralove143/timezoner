use std::sync::Arc;

use anyhow::{IntoResult, Result};
use twilight_gateway::Event;
use twilight_http::Client;

use crate::{interaction, parse, webhooks, Context};

/// handles the event, prints the returned error to stderr and tells the owner
#[allow(clippy::print_stderr)]
pub async fn handle(ctx: Context, event: Event) {
    if let Err(err) = _handle(Arc::clone(&ctx), event).await {
        if let Err(inform_error) = inform_owner(&ctx.http).await {
            eprintln!("informing the owner also failed: {inform_error:?}");
        }
        eprintln!("{err:?}");
    }
}

/// handles the event, passing on the returned error
#[allow(clippy::wildcard_enum_match_arm)]
pub async fn _handle(ctx: Context, event: Event) -> Result<()> {
    match event {
        Event::InteractionCreate(interaction) => interaction::handle(ctx, interaction.0).await?,
        Event::WebhooksUpdate(update) => webhooks::update(ctx, update.channel_id).await?,
        Event::MessageCreate(message) => parse::send_time(ctx, (*message).0).await?,
        _ => (),
    }
    Ok(())
}

/// tell the owner there was an error
async fn inform_owner(http: &Client) -> Result<()> {
    http.create_message(
        http.create_private_channel(
            http.current_user_application()
                .exec()
                .await?
                .model()
                .await?
                .owner
                .ok()?
                .id,
        )
        .exec()
        .await?
        .model()
        .await?
        .id,
    )
    .content("an error occurred :( check the stderr")?
    .exec()
    .await?;

    Ok(())
}
