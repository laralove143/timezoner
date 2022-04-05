use anyhow::Result;
use chrono::{Timelike, Utc};
use sqlx::SqlitePool;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};
use twilight_model::{
    application::interaction::application_command::CommandData,
    channel::message::MessageFlags,
    http::interaction::InteractionResponseData,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{database, parse};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "copy",
    desc = "copy the time so you can paste it in dms and all"
)]
/// the time command's options
pub struct Copy {
    #[command(desc = "the time you want to share")]
    /// the time to share
    time: String,
}

/// run the command, returning the response data
pub async fn run(
    db: &SqlitePool,
    user_id: Id<UserMarker>,
    command_data: CommandData,
) -> Result<InteractionResponseData> {
    let reply = _run(db, user_id, Copy::from_interaction(command_data.into())?).await?;

    Ok(InteractionResponseDataBuilder::new()
        .content(reply)
        .flags(MessageFlags::EPHEMERAL)
        .build())
}

/// run the command, returning the formatted string or the error message
async fn _run(db: &SqlitePool, user_id: Id<UserMarker>, options: Copy) -> Result<String> {
    let tz = match database::timezone(db, user_id).await? {
        Some(tz) => tz,
        None => {
            return Ok(
                "i don't know your timezone yet, and tell me using `/timezone` please".to_owned(),
            )
        }
    };

    let time = if let Some(time) = parse::try_time(&mut options.time.char_indices()) {
        Utc::today()
            .with_timezone(&tz)
            .and_hms(time.hour(), time.minute(), 0)
    } else {
        return Ok("i couldn't find a time there, sorry :(".to_owned());
    };

    Ok(format!(
        "`{}`",
        Timestamp::new(
            time.timestamp().try_into()?,
            Some(TimestampStyle::ShortTime)
        )
        .mention()
    ))
}
