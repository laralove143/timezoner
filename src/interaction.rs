use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{callback::InteractionResponse, interaction::Interaction},
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    interaction::{time::Time, timezone::Timezone},
    Context,
};

/// functions to build and run the time command
mod time;
/// functions to build and run the timezone command
mod timezone;

/// handle a slash command
pub async fn handle(ctx: Context, interaction: Interaction) -> Result<()> {
    let command = if let Interaction::ApplicationCommand(command) = interaction {
        command
    } else {
        bail!("unknown interaction: {:?}", interaction);
    };

    let client = ctx.http.interaction(ctx.application_id);

    let user_id = command
        .member
        .as_ref()
        .context("command is not run in a guild")?
        .user
        .as_ref()
        .context("the member info sent in the command doesn't have an attached user")?
        .id;

    let callback = match command.data.name.as_str() {
        "time" => time::run(&ctx.db, user_id, command.data).await?,
        "timezone" => timezone::run(&ctx.db, user_id, command.data).await?,
        _ => bail!("unknown command: {:?}", command),
    };

    client
        .interaction_callback(
            command.id,
            &command.token,
            &InteractionResponse::ChannelMessageWithSource(callback),
        )
        .exec()
        .await?;

    Ok(())
}

/// create commands globally
pub async fn create(http: &Client, application_id: Id<ApplicationMarker>) -> Result<()> {
    http.interaction(application_id)
        .set_global_commands(&[
            Time::create_command().into(),
            Timezone::create_command().into(),
        ])
        .exec()
        .await?;

    Ok(())
}
