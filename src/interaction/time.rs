use anyhow::{Context, Result};
use chrono::{Datelike, Utc};
use sqlx::SqlitePool;
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};
use twilight_model::{
    application::interaction::application_command::CommandData,
    http::interaction::InteractionResponseData,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    database,
    interaction::{action_row, copy_button, undo_copy_button},
    parse::to_24_hour,
};

#[derive(CreateOption, CommandOption, Clone, Copy)]
/// the choices for `am_pm` option
pub enum AmPm {
    /// time is am
    #[option(name = "am", value = "am")]
    Am,
    /// time is pm
    #[option(name = "pm", value = "pm")]
    Pm,
}

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "time",
    desc = "send a time that magically appears in everyone's own timezone"
)]
/// the time command's options
pub struct Time {
    #[command(desc = "in am/pm or 24-hour format ^^", min_value = 0, max_value = 23)]
    /// the hour to show
    hour: i64,
    #[command(desc = "between 0 and 59 obviously", min_value = 0, max_value = 59)]
    /// the minute to show
    minute: i64,
    #[command(desc = "leave empty if you used 24-hour format")]
    /// whether the time is am or pm or 24-hour
    am_pm: Option<AmPm>,
    #[command(desc = "leave empty for today", min_value = 0, max_value = 31)]
    /// the day of the date
    day: Option<i64>,
    #[command(desc = "leave empty for this month", min_value = 0, max_value = 12)]
    /// the month of the date
    month: Option<i64>,
    #[command(desc = "leave empty for this year", min_value = 0, max_value = 65535)]
    /// the year of the date
    year: Option<i64>,
}

/// run the command, returning the callback data
pub async fn run(
    db: &SqlitePool,
    user_id: Id<UserMarker>,
    command_data: CommandData,
) -> Result<InteractionResponseData> {
    let reply = _run(db, user_id, Time::from_interaction(command_data.into())?).await?;

    let callback = if reply.starts_with('<') {
        InteractionResponseDataBuilder::new().components([action_row(vec![copy_button()])])
    } else {
        InteractionResponseDataBuilder::new()
    };

    Ok(callback.content(reply).build())
}

/// run the command, returning the formatted string or the error message
async fn _run(db: &SqlitePool, user_id: Id<UserMarker>, options: Time) -> Result<String> {
    let tz = match database::timezone(db, user_id).await? {
        Some(tz) => tz,
        None => {
            return Ok(
                "i don't know your timezone yet, and tell me using `/timezone` please".to_owned(),
            )
        }
    };

    let hour = match options.am_pm {
        Some(am_pm) => match to_24_hour(options.hour.try_into()?, am_pm) {
            Some(hour) => hour,
            None => {
                return Ok("either keep am_pm empty or put an hour between 0 and 12...".to_owned())
            }
        },
        None => options.hour.try_into()?,
    };

    let mut datetime = Utc::today()
        .with_timezone(&tz)
        .and_hms(hour, options.minute.try_into()?, 0);

    let style = if options.day.or(options.month).or(options.year).is_some() {
        TimestampStyle::LongDateTime
    } else {
        TimestampStyle::ShortTime
    };

    if let Some(day) = options.day {
        datetime = datetime
            .with_day(day.try_into()?)
            .context("the day is invalid")?;
    };

    if let Some(month) = options.month {
        datetime = datetime
            .with_month(month.try_into()?)
            .context("the month is invalid")?;
    };

    if let Some(year) = options.year {
        datetime = datetime
            .with_year(year.try_into()?)
            .context("the year is invalid")?;
    };

    Ok(
        Timestamp::new(datetime.timestamp().try_into()?, Some(style))
            .mention()
            .to_string(),
    )
}

/// get the callback data with the time wrapped in backticks and the undo copy
/// button
pub fn run_copy(mut content: String) -> InteractionResponseData {
    content.insert(0, '`');
    content.push('`');

    InteractionResponseDataBuilder::new()
        .content(content)
        .components([action_row(vec![undo_copy_button()])])
        .build()
}

/// get the callback data with the time unwrapped from backticks and the copy
/// button
pub fn run_undo_copy(mut content: String) -> InteractionResponseData {
    content.remove(0);
    content.pop();

    InteractionResponseDataBuilder::new()
        .content(content)
        .components([action_row(vec![copy_button()])])
        .build()
}
