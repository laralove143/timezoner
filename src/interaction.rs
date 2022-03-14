use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{
        component::{button::ButtonStyle, ActionRow, Button, Component},
        interaction::{
            ApplicationCommand, ApplicationCommandAutocomplete, Interaction,
            MessageComponentInteraction,
        },
    },
    http::interaction::{InteractionResponse, InteractionResponseType},
    id::{marker::ApplicationMarker, Id},
};

use crate::{
    interaction::{enable_auto_conversion::EnableAutoConversion, time::Time, timezone::Timezone},
    Context,
};

/// functions to enable or disable parsing for a guild
mod enable_auto_conversion;
/// functions to build and run the time command
pub mod time;
/// functions to build and run the timezone command
mod timezone;

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

/// make the disable parsing button
pub fn disable_parsing_button() -> Component {
    Component::Button(Button {
        custom_id: Some("parsing_disable".to_owned()),
        label: Some("Disable auto-conversion (moderator only)".to_owned()),
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

    let response = match command.data.name.as_str() {
        "time" => time::run(&ctx.db, user_id, command.data).await?,
        "timezone" => timezone::run(&ctx.db, user_id, command.data).await?,
        "enable_auto_conversion" => {
            enable_auto_conversion::run(&ctx.db, command.guild_id, command.member, None).await?
        }
        _ => bail!("unknown command: {:?}", command),
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
        _ => bail!("unknown autocomplete command: {:?}", autocomplete),
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

/// handle a component interaction
async fn handle_component(ctx: &Context, component: MessageComponentInteraction) -> Result<()> {
    let client = ctx.http.interaction(ctx.application_id);

    let response = match component.data.custom_id.as_str() {
        "copy" => InteractionResponse {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(time::run_copy(component.message.content)),
        },
        "undo_copy" => InteractionResponse {
            kind: InteractionResponseType::UpdateMessage,
            data: Some(time::run_undo_copy(component.message.content)),
        },
        "parsing_disable" => InteractionResponse {
            kind: InteractionResponseType::ChannelMessageWithSource,
            data: Some(
                enable_auto_conversion::run(
                    &ctx.db,
                    component.guild_id,
                    component.member,
                    Some((&ctx.http, component.message)),
                )
                .await?,
            ),
        },
        _ => bail!("unknown custom id for component: {:?}", component),
    };

    client
        .create_response(component.id, &component.token, &response)
        .exec()
        .await?;

    Ok(())
}

/// create the slash commands globally
pub async fn create(http: &Client, application_id: Id<ApplicationMarker>) -> Result<()> {
    http.interaction(application_id)
        .set_global_commands(&[
            Time::create_command().into(),
            Timezone::build(),
            EnableAutoConversion::create_command().into(),
        ])
        .exec()
        .await?;

    Ok(())
}
