use std::str::FromStr;

use anyhow::Result;
use chrono_tz::Tz;
use sqlx::SqlitePool;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::application_command::CommandData,
    id::{marker::UserMarker, Id},
};

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

/// run the command, returning the reply
pub async fn run(
    db: &SqlitePool,
    user_id: Id<UserMarker>,
    command_data: CommandData,
) -> Result<&'static str> {
    let tz =
        if let Ok(tz) = Tz::from_str(&Timezone::from_interaction(command_data.into())?.timezone) {
            tz
        } else {
            return Ok(
                "i couldn't find that timezone >.< if you're sure it's right check my discord \
                 profile for my page to report please..",
            );
        };

    database::set_timezone(db, user_id, tz).await?;

    Ok("tada! now you can use the `/time` command ^^")
}
