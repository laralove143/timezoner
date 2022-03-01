use anyhow::anyhow;
use twilight_gateway::Event;

use crate::{commands, Context};

/// handle an event and print the returned error to stderr
#[allow(clippy::wildcard_enum_match_arm)]
pub async fn handle(ctx: Context, event: Event) {
    if let Err(err) = match event {
        Event::InteractionCreate(interaction) => commands::handle(ctx, (*interaction).0).await,
        _ => Err(anyhow!("unknown event: {:?}", event)),
    } {
        eprintln!("{}", err);
    }
}
