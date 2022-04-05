use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::interaction::{ApplicationCommand, ApplicationCommandAutocomplete, Interaction},
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    interaction::{copy::Copy, timezone::Timezone},
    Context,
};

/// functions to build and run the copy command
mod copy;
/// functions to build and run the `timezone` command
mod timezone;
/// functions to enable or disable parsing for a guild
mod toggle_auto_conversion;

/// make an action row with the given components
pub const fn action_row(components: Vec<Component>) -> Component {
    Component::ActionRow(ActionRow { components })
}

/// make the copy button
pub fn copy_button() -> Component {
    Component::Button(Button {
        custom_id: Some("copy".to_owned()),
        label: Some("Copy".to_owned()),
        style: ButtonStyle::Primary,
        disabled: false,
        emoji: None,
        url: None,
    })
}

/// make the undo copy button
pub fn undo_copy_button() -> Component {
    Component::Button(Button {
        custom_id: Some("undo_copy".to_owned()),
        label: Some("Undo Copy".to_owned()),
        style: ButtonStyle::Danger,
        disabled: false,
        emoji: None,
        url: None,
    })
}

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
        .context("command is not run in a guild")?
        .user
        .as_ref()
        .context("the member info sent in the command doesn't have an attached user")?
        .id;

    let response = match command.data.name.as_str() {
        "copy" => copy::run(&ctx.db, user_id, command.data).await?,
        "timezone" => timezone::run(&ctx.db, user_id, command.data).await?,
        "toggle_auto_conversion" => {
            toggle_auto_conversion::run(&ctx.db, command.guild_id, command.member).await?
        }
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
        .set_global_commands(&[Copy::create_command().into(), Timezone::build()])
        .exec()
        .await?;

    Ok(())
}
