use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::interaction::{ApplicationCommand, ApplicationCommandAutocomplete, Interaction},
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
    match interaction {
        Interaction::ApplicationCommand(command) => handle_command(&ctx, *command).await?,
        Interaction::ApplicationCommandAutocomplete(autocomplete) => {
            handle_autocomplete(&ctx, *autocomplete).await?;
        }
        _ => {
            bail!("unknown interaction: {interaction:#?}");
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
        .map_or(&command.user, |member| &member.user)
        .as_ref()
        .context("the command doesn't have any attached user")?
        .id;

    let response = match command.data.name.as_str() {
        "copy" => copy::run(ctx, user_id, command.data).await?,
        "timezone" => timezone::run(ctx, user_id, command.data).await?,
        "help" => help::run(),
        _ => bail!("unknown command: {command:#?}"),
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
async fn handle_autocomplete(
    ctx: &Context,
    autocomplete: ApplicationCommandAutocomplete,
) -> Result<()> {
    let client = ctx.http.interaction(ctx.application_id);

    let response = match autocomplete.data.name.as_str() {
        "timezone" => timezone::run_autocomplete(ctx, autocomplete.data.options.into())?,
        _ => bail!("unknown autocomplete command: {autocomplete:#?}"),
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
