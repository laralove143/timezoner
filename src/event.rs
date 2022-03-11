use twilight_gateway::Event;

use crate::{interaction, parse, Context};

/// handle an event and print the returned error to stderr
#[allow(clippy::wildcard_enum_match_arm)]
pub async fn handle(ctx: Context, event: Event) {
    if let Err(err) = match event {
        Event::InteractionCreate(interaction) => interaction::handle(ctx, (*interaction).0).await,
        Event::MessageCreate(message) => parse::send_time(ctx, (*message).0).await,
        _ => Ok(()),
    } {
        eprintln!("{}", err);
    }
}
