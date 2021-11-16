use anyhow::{bail, Result};

use chrono::{Datelike, Utc};
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
    mut options: Vec<CommandDataOption>,
) -> Result<String> {
    let am_pm = if let CommandOptionValue::String(option) = options.remove(2).value {
        match option.as_str() {
            "am" => AmPm::Am,
            "pm" => AmPm::Pm,
            _ => bail!("am_pm option is not am or pm: {:?}", options),
        }
    } else {
        bail!("am_pm option is not string: {:?}", options);
    };

    let hour = if let CommandOptionValue::Integer(option) = options.remove(0).value {
        if !(1..=12).contains(&option) {
            return Ok("hour has to be between 0 and 12 please :(".to_string());
        }
        to_24_hour(option as u32, am_pm)
    } else {
        bail!("hour option is not an integer: {:?}", options);
    };

    let minute = if let CommandOptionValue::Integer(option) = options.remove(0).value {
        if !(0..60).contains(&option) {
            return Ok("minute has to be between 0 and 59 please :(".to_string());
        }
        option as u32
    } else {
        bail!("minute option is not an integer: {:?}", options);
    };

    let tz = match database::timezone(db, user_id).await? {
        Some(tz) => tz,
        None => {
            return Ok(
                "it looks like i don't know your timezone yet, copy it from https://kevinnovak.github.io/Time-Zone-Picker and tell it to me using `/set_timezone` please"
                .to_string(),
            )
        }
    };

    let mut datetime = Utc::today().with_timezone(&tz).and_hms(hour, minute, 0);
    let mut style = TimestampStyle::ShortTime;

    for option in &options {
        match option.name.as_str() {
            "day" => {
                if let CommandOptionValue::Integer(option) = option.value {
                    if !(1..=31).contains(&option) {
                        return Ok("day has to be between 1 and 31 please :(".to_string());
                    }
                    style = TimestampStyle::LongDateTime;
                    datetime = datetime.with_day(option as u32).unwrap();
                } else {
                    bail!("day option is not an integer: {:?}", options);
                }
            }
            "month" => {
                if let CommandOptionValue::Integer(option) = option.value {
                    if !(1..=12).contains(&option) {
                        return Ok("month has to be between 1 and 12 please :(".to_string());
                    }
                    style = TimestampStyle::LongDateTime;
                    datetime = datetime.with_month(option as u32).unwrap();
                } else {
                    bail!("hour option is not an integer: {:?}", options);
                }
            }
            "year" => {
                if let CommandOptionValue::Integer(option) = option.value {
                    match u16::try_from(option) {
                        Ok(year) => {
                            style = TimestampStyle::LongDateTime;
                            datetime = datetime.with_year(year.into()).unwrap()
                        }
                        Err(_) => return Ok("the year is too big or negative ;-;".to_string()),
                    }
                } else {
                    bail!("hour option is not an integer: {:?}", options);
                }
            }
            _ => bail!("unmatched option: {:?}", options),
        }
    }

    Ok(Timestamp::new(datetime.timestamp() as u64, Some(style))
        .mention()
        .to_string())
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
    .option(IntegerBuilder::new(
        "day".to_string(),
        "leave empty for today".to_string(),
    ))
    .option(IntegerBuilder::new(
        "month".to_string(),
        "leave empty for this month".to_string(),
    ))
    .option(IntegerBuilder::new(
        "year".to_string(),
        "leave empty for this year".to_string(),
    ))
    .build()
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
