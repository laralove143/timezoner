use anyhow::Result;
use sqlx::SqlitePool;
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::CommandDataOption,
    },
    id::UserId,
};
use twilight_util::builder::command::{CommandBuilder, IntegerBuilder, StringBuilder};

use super::time;

pub async fn run(
    db: &SqlitePool,
    user_id: UserId,
    options: Vec<CommandDataOption>,
) -> Result<String> {
    time::run_datetime(
        db,
        user_id,
        &options[3],
        &options[4],
        &options[5],
        Some((&options[0], &options[1], &options[2])),
    )
    .await
}

pub fn build() -> Command {
    CommandBuilder::new(
        "date".to_string(),
        "send a date".to_string(),
        CommandType::ChatInput,
    )
    .option(
        IntegerBuilder::new("day".to_string(), "between 0 and 31 please".to_string())
            .required(true),
    )
    .option(
        IntegerBuilder::new("month".to_string(), "between 1 and 12 please".to_string())
            .required(true),
    )
    .option(
        IntegerBuilder::new("year".to_string(), "no limits this time!".to_string()).required(true),
    )
    .option(
        IntegerBuilder::new("hour".to_string(), "in am/pm format please".to_string())
            .required(true),
    )
    .option(
        IntegerBuilder::new(
            "minute".to_string(),
            "between 0 and 59 obviously".to_string(),
        )
        .required(true),
    )
    .option(
        StringBuilder::new("am_pm".to_string(), "is it am or is it pm?".to_string())
            .choices([
                ("am".to_string(), "am".to_string()),
                ("pm".to_string(), "pm".to_string()),
            ])
            .required(true),
    )
    .build()
}
