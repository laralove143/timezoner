use anyhow::{bail, Context as _, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{
        callback::InteractionResponse,
        component::{button::ButtonStyle, ActionRow, Button, Component},
        interaction::{ApplicationCommand, Interaction, MessageComponentInteraction},
    },
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
        label: Some("Disable auto-conversion".to_owned()),
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
        "enable_auto_conversion" => {
            enable_auto_conversion::run(&ctx.db, command.guild_id, command.member, true).await?
        }
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

    let response = match component.data.custom_id.as_str() {
        "copy" => InteractionResponse::UpdateMessage(time::run_copy(component.message.content)),
        "undo_copy" => {
            InteractionResponse::UpdateMessage(time::run_undo_copy(component.message.content))
        }
        "parsing_disable" => InteractionResponse::ChannelMessageWithSource(
            enable_auto_conversion::run(&ctx.db, component.guild_id, component.member, false)
                .await?,
        ),
        _ => bail!("unknown custom id for component: {:?}", component),
    };

    client
        .interaction_callback(component.id, &component.token, &response)
        .exec()
        .await?;

    Ok(())
}

/// create the slash commands globally
pub async fn create(http: &Client, application_id: Id<ApplicationMarker>) -> Result<()> {
    http.interaction(application_id)
        .set_global_commands(&[
            Time::create_command().into(),
            Timezone::create_command().into(),
            EnableAutoConversion::create_command().into(),
        ])
        .exec()
        .await?;

    Ok(())
}
