use anyhow::{bail, Result};

use chrono::{TimeZone, Utc};
use sqlx::SqlitePool;
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};
use twilight_model::{
    application::{
        command::{Command, CommandType},
        interaction::application_command::{CommandDataOption, CommandOptionValue},
    },
    id::UserId,
};
use twilight_util::builder::command::{CommandBuilder, IntegerBuilder, StringBuilder};

use crate::database;

enum AmPm {
    Am,
    Pm,
}

pub async fn run(
    db: &SqlitePool,
    user_id: UserId,
    options: Vec<CommandDataOption>,
) -> Result<String> {
    run_datetime(db, user_id, &options[0], &options[1], &options[2], None).await
}

pub fn build() -> Command {
    CommandBuilder::new(
        "time".to_string(),
        "send a time that magically appears in everyone's own timezone".to_string(),
        CommandType::ChatInput,
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

pub async fn run_datetime(
    db: &SqlitePool,
    user_id: UserId,
    hour: &CommandDataOption,
    minute: &CommandDataOption,
    am_pm: &CommandDataOption,
    day_month_year: Option<(&CommandDataOption, &CommandDataOption, &CommandDataOption)>,
) -> Result<String> {
    let am_pm = if let CommandOptionValue::String(am_pm) = &am_pm.value {
        match am_pm.as_str() {
            "am" => AmPm::Am,
            "pm" => AmPm::Pm,
            _ => bail!("am_pm option is not am or pm: {:?}", am_pm),
        }
    } else {
        bail!("am_pm option is not string: {:?}", am_pm);
    };

    let hour = if let CommandOptionValue::Integer(hour) = hour.value {
        if !(0..=12).contains(&hour) {
            return Ok("hour has to be between 0 and 12 please :(".to_string());
        }
        to_24_hour(hour as u32, am_pm)
    } else {
        bail!("hour option is not an integer: {:?}", hour);
    };

    let minute = if let CommandOptionValue::Integer(minute) = minute.value {
        if !(0..60).contains(&minute) {
            return Ok("minute has to be between 0 and 59 please :(".to_string());
        }
        minute as u32
    } else {
        bail!("minute option is not integer: {:?}", minute);
    };

    let tz = match database::timezone(db, user_id).await? {
        Some(tz) => tz,
        None => {
            return Ok(
                "it looks like i don't know your timezone yet, copy it from https://kevinnovak.github.io/Time-Zone-Picker and tell it to me using `/set_timezone` please"
                .to_string(),
            )
        }
    };

    match day_month_year {
        Some((day, month, year)) => {
            let day = if let CommandOptionValue::Integer(day) = day.value {
                if !(1..=31).contains(&day) {
                    return Ok("day has to be between 1 and 31 please :(".to_string());
                }
                day as u32
            } else {
                bail!("day option is not an integer {:?}", day);
            };

            let month = if let CommandOptionValue::Integer(month) = month.value {
                if !(1..=12).contains(&month) {
                    return Ok("month has to be between 1 and 12 please :(".to_string());
                }
                month as u32
            } else {
                bail!("month option is not an integer {:?}", month);
            };

            let year = if let CommandOptionValue::Integer(year) = year.value {
                match year.try_into() {
                    Ok(year) => year,
                    Err(_) => return Ok("the year is too big or too small".to_string()),
                }
            } else {
                bail!("year option is not an integer {:?}", year);
            };

            let timestamp = tz
                .ymd(year, month, day)
                .and_hms(hour, minute, 0)
                .timestamp() as u64;

            return Ok(
                Timestamp::new(timestamp, Some(TimestampStyle::LongDateTime))
                    .mention()
                    .to_string(),
            );
        }
        None => {
            let timestamp = Utc::today()
                .with_timezone(&tz)
                .and_hms(hour, minute, 0)
                .timestamp() as u64;

            return Ok(Timestamp::new(timestamp, Some(TimestampStyle::ShortTime))
                .mention()
                .to_string());
        }
    }
}

fn to_24_hour(hour: u32, am_pm: AmPm) -> u32 {
    match am_pm {
        AmPm::Am => {
            if hour == 12 {
                hour - 12
            } else {
                hour
            }
        }
        AmPm::Pm => {
            if hour == 12 {
                hour
            } else {
                hour + 12
            }
        }
    }
}
