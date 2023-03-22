use std::ops::Range;

use anyhow::Result;
use chrono::{DateTime, Datelike, TimeZone, Timelike, Utc};
use chrono_tz::Tz;
use lazy_regex::{lazy_regex, Captures, Lazy, Regex};
use sparkle_convenience::error::IntoError;
use twilight_model::id::{marker::UserMarker, Id};

use crate::{
    interaction::{date, date::Style},
    Context, CustomError, Error,
};

static REGEX_24_HOUR: Lazy<Regex> = lazy_regex!(r#"\b([0-1]?[0-9]|2[0-3]):([0-5][0-9])\b"#);
static REGEX_12_HOUR: Lazy<Regex> = lazy_regex!(r#"\b(1[0-2]|0?[1-9]) ?([AaPp][Mm])\b"#);
static REGEX_12_HOUR_WITH_MIN: Lazy<Regex> =
    lazy_regex!(r#"\b(1[0-2]|0?[1-9]):([0-5][0-9]) ?([AaPp][Mm])\b"#);

#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedTime {
    pub hour: u32,
    pub min: u32,
    pub range: Range<usize>,
}

impl ParsedTime {
    pub fn all_from_text(s: &str) -> Result<Vec<Self>> {
        let parsed_12_hour_with_min = Self::all_from_12_hour_with_min(s)?;
        if !parsed_12_hour_with_min.is_empty() {
            return Ok(parsed_12_hour_with_min);
        }

        let parsed_12_hour = Self::all_from_12_hour(s)?;
        if !parsed_12_hour.is_empty() {
            return Ok(parsed_12_hour);
        }

        let parsed_24_hour = Self::all_from_24_hour(s)?;
        if !parsed_24_hour.is_empty() {
            return Ok(parsed_24_hour);
        }

        Ok(vec![])
    }

    fn all_from_12_hour_with_min(s: &str) -> Result<Vec<Self>> {
        REGEX_12_HOUR_WITH_MIN
            .captures_iter(s)
            .map(|captures| {
                let hour = captures[1].parse()?;
                let min = captures[2].parse()?;
                let am_pm = &captures[3];
                Ok(Self {
                    hour: to_24_hour(hour, am_pm)?,
                    min,
                    range: Self::range(&captures)?,
                })
            })
            .collect()
    }

    fn all_from_12_hour(s: &str) -> Result<Vec<Self>> {
        REGEX_12_HOUR
            .captures_iter(s)
            .map(|captures| {
                let hour = captures[1].parse()?;
                let am_pm = &captures[2];
                Ok(Self {
                    hour: to_24_hour(hour, am_pm)?,
                    min: 0,
                    range: Self::range(&captures)?,
                })
            })
            .collect()
    }

    fn all_from_24_hour(s: &str) -> Result<Vec<Self>> {
        REGEX_24_HOUR
            .captures_iter(s)
            .map(|captures| {
                let hour = captures[1].parse()?;
                let min = captures[2].parse()?;
                Ok(Self {
                    hour,
                    min,
                    range: Self::range(&captures)?,
                })
            })
            .collect()
    }

    fn range(captures: &Captures<'_>) -> Result<Range<usize>> {
        Ok(captures.get(0).ok()?.range())
    }
}

impl Context {
    pub async fn user_time(
        &self,
        user_id: Id<UserMarker>,
        date: date::Command,
    ) -> Result<DateTime<Tz>> {
        let Some(tz) = self.timezone(user_id).await? else {
            return Err(CustomError::MissingTimezone(self.command_ids.timezone).into());
        };

        let now = Utc::now().with_timezone(&tz);
        Ok(tz
            .with_ymd_and_hms(
                date.year
                    .map_or_else(|| Ok(now.year()), TryInto::try_into)?,
                date.month
                    .map_or_else(|| Ok(now.month()), |month| month.value().try_into())?,
                date.day.map_or_else(|| Ok(now.day()), TryInto::try_into)?,
                date.hour
                    .map_or_else(|| Ok(now.hour()), TryInto::try_into)?,
                date.minute
                    .map_or_else(|| Ok(now.minute()), TryInto::try_into)?,
                date.second
                    .map_or_else(|| Ok(now.second()), TryInto::try_into)?,
            )
            .single()
            .ok_or(CustomError::BadDate)?)
    }
}

pub fn format(time: DateTime<Tz>, style: Option<Style>) -> String {
    format!(
        "<t:{}:{}>",
        time.timestamp(),
        style.unwrap_or(Style::LongDateTime).value()
    )
}

pub fn tz_from_locale(locale: &str) -> Option<Tz> {
    Some(match locale {
        "da" | "de" => Tz::Europe__Berlin,
        "fr" => Tz::Europe__Paris,
        "hr" => Tz::Europe__Zagreb,
        "it" => Tz::Europe__Rome,
        "lt" => Tz::Europe__Vilnius,
        "hu" | "ro" => Tz::Europe__Bucharest,
        "nl" => Tz::Europe__Amsterdam,
        "no" => Tz::Europe__Oslo,
        "pl" => Tz::Poland,
        "fi" => Tz::Europe__Helsinki,
        "sv-SE" => Tz::Europe__Stockholm,
        "vi" => Tz::Asia__Ho_Chi_Minh,
        "tr" => Tz::Europe__Istanbul,
        "cs" => Tz::Europe__Prague,
        "el" => Tz::Europe__Athens,
        "bg" => Tz::Europe__Sofia,
        "uk" => Tz::Europe__Kiev,
        "hi" => Tz::Asia__Calcutta,
        "th" => Tz::Asia__Bangkok,
        "ja" => Tz::Asia__Tokyo,
        "zh-TW" => Tz::Asia__Taipei,
        "ko" => Tz::Asia__Seoul,
        _ => return None,
    })
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
        _ => {
            return Err(Error::Hour12InvalidSuffix {
                hour,
                suffix: am_pm.to_owned(),
            }
            .into())
        }
    })
}
