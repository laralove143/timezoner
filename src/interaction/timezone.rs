use std::str::FromStr;

use anyhow::Result;
use chrono_tz::Tz;
use sqlx::SqlitePool;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::application::callback::CallbackData;
use twilight_model::{
    application::interaction::application_command::CommandData,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::CallbackDataBuilder;

use crate::database;

/// the timezone command's options
#[derive(CommandModel, CreateCommand)]
#[command(
    name = "timezone",
    desc = "set your time zone so that you can actually use the `/time` command"
)]
pub struct Timezone {
    #[command(
        desc = "use `/time` (with a random hour/minute) to learn what to put here.. sorry for the \
                inconvenience >.<"
    )]
    /// the user's timezone
    timezone: String,
}

/// run the command, returning the callback data
pub async fn run(
    db: &SqlitePool,
    user_id: Id<UserMarker>,
    command_data: CommandData,
) -> Result<CallbackData> {
    let reply = _run(
        db,
        user_id,
        Timezone::from_interaction(command_data.into())?,
    )
    .await?;

    Ok(CallbackDataBuilder::new().content(reply.to_owned()).build())
}

/// run the command, returning the success or error message
async fn _run(db: &SqlitePool, user_id: Id<UserMarker>, options: Timezone) -> Result<&'static str> {
    let tz = if let Ok(tz) = Tz::from_str(&options.timezone) {
        tz
    } else {
        return Ok(
            "i couldn't find that timezone >.< if you're sure it's right check my discord profile \
             for my page to report please..",
        );
    };

    database::set_timezone(db, user_id, tz).await?;

    Ok("tada! now you can use the `/time` command ^^")
}
