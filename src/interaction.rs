use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{
        callback::InteractionResponse,
        interaction::{ApplicationCommand, Interaction, MessageComponentInteraction},
    },
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

/// handle an interaction
#[allow(clippy::wildcard_enum_match_arm)]
pub async fn handle(ctx: Context, interaction: Interaction) -> Result<()> {
    match interaction {
        Interaction::ApplicationCommand(command) => handle_command(&ctx, *command).await?,
        Interaction::MessageComponent(component) => handle_component(&ctx, *component).await?,
        _ => {
            bail!("unknown interaction: {:?}", interaction);
        }
    };

    Ok(())
}

/// handle a slash command
async fn handle_command(ctx: &Context, command: ApplicationCommand) -> Result<()> {
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

/// handle a component interaction
async fn handle_component(ctx: &Context, component: MessageComponentInteraction) -> Result<()> {
    let client = ctx.http.interaction(ctx.application_id);

    let callback = match component.data.custom_id.as_str() {
        "copy" => time::run_copy(component.message.content),
        "undo_copy" => time::run_undo_copy(component.message.content),
        _ => bail!("unknown custom id for component: {:?}", component),
    };

    client
        .interaction_callback(
            component.id,
            &component.token,
            &InteractionResponse::UpdateMessage(callback),
        )
        .exec()
        .await?;

    Ok(())
}

/// create interaction globally
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
