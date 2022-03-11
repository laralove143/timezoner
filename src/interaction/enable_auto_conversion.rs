use anyhow::{Context, Result};
use sqlx::SqlitePool;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::callback::CallbackData,
    guild::{PartialMember, Permissions},
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::CallbackDataBuilder;

use crate::database;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "enable_auto_conversion",
    desc = "enable the auto-conversion feature again"
)]
/// the `enable_auto_conversion` command
pub struct EnableAutoConversion {}

/// run the command, returning the callback data
pub async fn run(
    db: &SqlitePool,
    guild_id: Option<Id<GuildMarker>>,
    member: Option<PartialMember>,
    enable: bool,
) -> Result<CallbackData> {
    let reply = _run(
        db,
        guild_id.context("enable_auto_conversion command isn't run in a guild")?,
        member.context("enable_auto_conversion command isn't run in a guild")?,
        enable,
    )
    .await?;

    Ok(CallbackDataBuilder::new().content(reply.to_owned()).build())
}

/// run the command, returning the success or error message
async fn _run(
    db: &SqlitePool,
    guild_id: Id<GuildMarker>,
    member: PartialMember,
    enable: bool,
) -> Result<&'static str> {
    if !member
        .permissions
        .context("member isn't sent in an interaction")?
        .contains(Permissions::MANAGE_GUILD)
    {
        return Ok("you need the manage guild permission to use this..");
    }

    if enable {
        database::enable_parsing(db, guild_id).await?;
        Ok("tada! now i'll automatically convert any time i see in the chat")
    } else {
        database::disable_parsing(db, guild_id).await?;
        Ok("okay.. i'll stop annoying you then")
    }
}
