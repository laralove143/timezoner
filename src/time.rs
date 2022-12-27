use std::ops::Range;

use anyhow::{anyhow, Result};
use chrono::TimeZone;
use lazy_regex::{lazy_regex, Lazy, Regex};
use sparkle_convenience::error::conversion::IntoError;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{Context, CustomError};

static REGEX_24_HOUR: Lazy<Regex> = lazy_regex!(r#"\b([0-1]?[0-9]|2[0-3]):([0-5][0-9])\b"#);
static REGEX_12_HOUR: Lazy<Regex> = lazy_regex!(r#"\b(1[0-2]|0?[1-9]) ?([AaPp][Mm])\b"#);
static REGEX_12_HOUR_WITH_MIN: Lazy<Regex> =
    lazy_regex!(r#"\b(1[0-2]|0?[1-9]):([0-5][0-9]) ?([AaPp][Mm])\b"#);

impl Context {
    pub async fn user_timestamp(
        &self,
        user_id: Id<UserMarker>,
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        min: u32,
    ) -> Result<i64> {
        let Some(tz) = self.timezone(user_id).await? else {
            return Err(CustomError::MissingTimezone(self.command_ids.timezone).into());
        };

        Ok(tz
            .with_ymd_and_hms(year, month, day, hour, min, 0)
            .single()
            .ok()?
            .timestamp())
    }
}

pub fn parse_time(s: &str) -> Result<Option<(u32, u32, Range<usize>)>> {
    if let Some(captures) = REGEX_12_HOUR_WITH_MIN.captures(s) {
        let hour = captures[1].parse()?;
        let min = captures[2].parse()?;
        let am_pm = &captures[3];
        Ok(Some((
            to_24_hour(hour, am_pm)?,
            min,
            captures.get(0).ok()?.range(),
        )))
    } else if let Some(captures) = REGEX_12_HOUR.captures(s) {
        let hour = captures[1].parse()?;
        let am_pm = &captures[2];
        Ok(Some((
            to_24_hour(hour, am_pm)?,
            0,
            captures.get(0).ok()?.range(),
        )))
    } else if let Some(captures) = REGEX_24_HOUR.captures(s) {
        let hour = captures[1].parse()?;
        let min = captures[2].parse()?;
        Ok(Some((hour, min, captures.get(0).ok()?.range())))
    } else {
        Ok(None)
    }
}

fn to_24_hour(hour: u32, am_pm: &str) -> Result<u32> {
    Ok(match am_pm.to_ascii_lowercase().as_str() {
        "am" => {
            if hour == 12 {
                0
            } else {
                hour
            }
        }
        "pm" => {
            if hour == 12 {
                hour
            } else {
                hour + 12
            }
        }
        _ => return Err(anyhow!("time doesn't end in am or pm")),
    })
}
