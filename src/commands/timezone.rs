use std::str::FromStr;

use anyhow::{bail, Result};
use chrono_tz::Tz;
use sqlx::SqlitePool;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::{CommandDataOption, CommandOptionValue},
    },
    id::UserId,
};
use twilight_util::builder::command::{CommandBuilder, StringBuilder};

use crate::database;

pub async fn run(
    db: &SqlitePool,
    user_id: UserId,
    options: Vec<CommandDataOption>,
) -> Result<String> {
    let tz = if let CommandOptionValue::String(tz_name) = &options[0].value {
        match Tz::from_str(tz_name) {
            Ok(tz) => tz,
            Err(_) => return Ok("i couldn't find that timezone >.< if you're sure it's right see my profile to report please".to_string()),
        }
    } else {
        bail!("first option for timezone is not string: {:?}", options);
    };

    database::set_timezone(db, user_id, &tz).await?;

    Ok("tada! now you can use the `/time` command ^^".to_string())
}

pub fn build() -> Command {
    CommandBuilder::new(
        "timezone".to_string(),
        "set your time zone so that you can actually use the `/time` command".to_string(),
        CommandType::ChatInput,
    )
    .option(
        StringBuilder::new(
            "timezone".to_string(),
            "use `/time` (with a random hour/minute) to learn what to put here.. sorry for the inconvenience >.<"
            .to_string(),
        )
        .required(true),
    )
    .build()
}
