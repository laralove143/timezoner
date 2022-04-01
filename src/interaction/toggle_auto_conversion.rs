use anyhow::{Context, Result};
use sqlx::SqlitePool;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::message::MessageFlags,
    guild::{PartialMember, Permissions},
    http::interaction::InteractionResponseData,
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::database;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "toggle_auto_conversion",
    desc = "enable or disable the auto-conversion feature"
)]
/// the `toggle_auto_conversion` command
pub struct ToggleAutoConversion {}

/// run the command, returning the response data
pub async fn run(
    db: &SqlitePool,
    guild_id: Option<Id<GuildMarker>>,
    member: Option<PartialMember>,
) -> Result<InteractionResponseData> {
    let reply = _run(
        db,
        guild_id.context("enable_auto_conversion command isn't run in a guild")?,
        member.context("enable_auto_conversion command isn't run in a guild")?,
    )
    .await?;

    Ok(InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL)
        .content(reply.to_owned())
        .build())
}

/// run the command, returning the success or error message
async fn _run(
    db: &SqlitePool,
    guild_id: Id<GuildMarker>,
    member: PartialMember,
) -> Result<&'static str> {
    if !member
        .permissions
        .context("member isn't sent in an interaction")?
        .contains(Permissions::MANAGE_GUILD)
    {
        return Ok("you need the manage guild permission to use this..");
    }

    database::toggle_parsing(db, guild_id).await?;
    Ok("done!")
}
