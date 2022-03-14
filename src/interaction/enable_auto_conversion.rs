use anyhow::{Context, Result};
use sqlx::SqlitePool;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    channel::{message::MessageFlags, Message},
    guild::{PartialMember, Permissions},
    http::interaction::InteractionResponseData,
    id::{marker::GuildMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

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
    disable_message: Option<(&Client, Message)>,
) -> Result<InteractionResponseData> {
    let reply = _run(
        db,
        guild_id.context("enable_auto_conversion command isn't run in a guild")?,
        member.context("enable_auto_conversion command isn't run in a guild")?,
        disable_message,
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
    disable_message: Option<(&Client, Message)>,
) -> Result<&'static str> {
    if !member
        .permissions
        .context("member isn't sent in an interaction")?
        .contains(Permissions::MANAGE_GUILD)
    {
        return Ok("you need the manage guild permission to use this..");
    }

    if let Some((client, message)) = disable_message {
        database::disable_parsing(db, guild_id).await?;
        client
            .delete_message(message.channel_id, message.id)
            .exec()
            .await?;
        Ok("okay.. i'll stop annoying you then")
    } else {
        database::enable_parsing(db, guild_id).await?;
        Ok("tada! now i'll automatically convert any time i see in the chat")
    }
}
