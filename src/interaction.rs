use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::interaction::{Interaction, InteractionData, InteractionType},
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    interaction::{copy::Copy, help::Help, timezone::Timezone},
    Context,
};

/// functions to build and run the copy command
mod copy;
/// functions to build and run the `help` command
mod help;
/// functions to build and run the `timezone` command
mod timezone;

/// handle an interaction
#[allow(clippy::wildcard_enum_match_arm)]
pub async fn handle(ctx: Context, interaction: Interaction) -> Result<()> {
    match interaction.kind {
        InteractionType::ApplicationCommand => handle_command(&ctx, interaction).await?,
        InteractionType::ApplicationCommandAutocomplete => {
            handle_autocomplete(&ctx, interaction).await?;
        }
        _ => {
            bail!("unknown interaction: {interaction:#?}");
        }
    };

    Ok(())
}

/// handle a slash command
async fn handle_command(ctx: &Context, command: Interaction) -> Result<()> {
    let client = ctx.http.interaction(ctx.application_id);

    let user_id = command
        .member
        .as_ref()
        .map_or(&command.user, |member| &member.user)
        .as_ref()
        .context("the command doesn't have any attached user")?
        .id;

    let data = if let InteractionData::ApplicationCommand(data) = command
        .data
        .context("slash command interaction doesn't have data attached")?
    {
        data
    } else {
        bail!("data attached to slash command isn't application command variant")
    };

    let response = match data.name.as_str() {
        "copy" => copy::run(ctx, user_id, *data).await?,
        "timezone" => timezone::run(ctx, user_id, *data).await?,
        "help" => help::run(),
        _ => bail!("unknown command: {data:#?}"),
    };

    client
        .create_response(
            command.id,
            &command.token,
            &InteractionResponse {
                kind: InteractionResponseType::ChannelMessageWithSource,
                data: Some(response),
            },
        )
        .exec()
        .await?;

    Ok(())
}

/// handle a sent autocomplete data
async fn handle_autocomplete(ctx: &Context, autocomplete: Interaction) -> Result<()> {
    let client = ctx.http.interaction(ctx.application_id);

    let data = if let InteractionData::ApplicationCommand(data) = autocomplete
        .data
        .context("autocomplete interaction doesn't have data attached")?
    {
        data
    } else {
        bail!("data attached to autocomplete isn't application command variant")
    };

    let response = match data.name.as_str() {
        "timezone" => timezone::run_autocomplete(ctx, data.options.try_into()?)?,
        _ => bail!("unknown autocomplete command: {data:#?}"),
    };

    client
        .create_response(
            autocomplete.id,
            &autocomplete.token,
            &InteractionResponse {
                kind: InteractionResponseType::ApplicationCommandAutocompleteResult,
                data: Some(response),
            },
        )
        .exec()
        .await?;

    Ok(())
}

/// create the slash commands globally
pub async fn create(http: &Client, application_id: Id<ApplicationMarker>) -> Result<()> {
    http.interaction(application_id)
        .set_global_commands(&[
            Copy::create_command().into(),
            Timezone::build(),
            Help::create_command().into(),
        ])
        .exec()
        .await?;

    Ok(())
}
