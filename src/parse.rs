use std::str::Chars;

use anyhow::{Context as _, Result};
use chrono::{NaiveTime, Utc};
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_mention::{
    timestamp::{Timestamp, TimestampStyle},
    Mention,
};
use twilight_model::{
    channel::{ChannelType, Message},
    guild::Permissions,
    id::Id,
};

use crate::{
    database,
    interaction::{action_row, copy_button, disable_parsing_button, time::AmPm},
    Context,
};

/// the reaction request with the unknown timezone emoji
const UNKNOWN_TIMEZONE_EMOJI: RequestReactionType = RequestReactionType::Custom {
    id: Id::new(950_033_075_440_603_186),
    name: Some("use_timezone_command"),
};

/// extracts a date/time from text and the user's saved timezone and sends
/// discord formatted timestamp to the message's channel
pub async fn send_time(ctx: Context, message: Message) -> Result<()> {
    if message.author.bot
        || ctx
            .cache
            .channel(message.channel_id)
            .map_or(true, |c| c.kind != ChannelType::GuildText)
        || !ctx
            .cache
            .permissions()
            .in_channel(ctx.user_id, message.channel_id)?
            .contains(
                Permissions::SEND_MESSAGES
                    | Permissions::ADD_REACTIONS
                    | Permissions::USE_EXTERNAL_EMOJIS,
            )
        || database::parsing_disabled(
            &ctx.db,
            message
                .guild_id
                .context("message to parse is not in a guild")?,
        )
        .await?
    {
        return Ok(());
    }

    let time = if let Some(time) = time(&message.content) {
        time
    } else {
        return Ok(());
    };

    let tz = if let Some(tz) = database::timezone(&ctx.db, message.author.id).await? {
        tz
    } else {
        ctx.http
            .create_reaction(message.channel_id, message.id, &UNKNOWN_TIMEZONE_EMOJI)
            .exec()
            .await?;
        return Ok(());
    };

    let timestamp = Timestamp::new(
        Utc::today()
            .with_timezone(&tz)
            .and_time(time)
            .context("parsed naive time is invalid")?
            .timestamp()
            .try_into()?,
        Some(TimestampStyle::ShortTime),
    )
    .mention()
    .to_string();

    ctx.http
        .create_message(message.channel_id)
        .content(&timestamp)?
        .components(&[action_row(vec![copy_button(), disable_parsing_button()])])?
        .exec()
        .await?;

    Ok(())
}

/// parses a time in any format
fn time(s: &str) -> Option<NaiveTime> {
    let mut chars = s.chars();

    while chars.size_hint().0 != 0 {
        let time = try_time(&mut chars);
        if time.is_some() {
            return time;
        }
    }

    None
}

/// parses one possible occurrence of a time
#[allow(clippy::integer_arithmetic)]
fn try_time(chars: &mut Chars) -> Option<NaiveTime> {
    let mut current;
    let mut min_given = false;

    let mut hour = chars.find_map(|c| c.to_digit(10))?;
    let mut min = 0;

    current = chars.next()?;
    if let Some(digit) = current.to_digit(10) {
        hour = hour * 10 + digit;
        current = chars.next()?;
    }

    if current == ':' {
        min = chars.next()?.to_digit(10)? * 10 + chars.next()?.to_digit(10)?;
        current = match chars.next() {
            Some(c) => c,
            None => return NaiveTime::from_hms_opt(hour, min, 0),
        };
        min_given = true;
    }

    if current == ' ' {
        current = chars.next()?;
    }

    match chars.next().and_then(|char| {
        if let 'm' | 'M' = char {
            if let 'a' | 'A' = current {
                Some(AmPm::Am)
            } else if let 'p' | 'P' = current {
                Some(AmPm::Pm)
            } else {
                None
            }
        } else {
            None
        }
    }) {
        Some(am_pm) => hour = to_24_hour(hour, am_pm)?,
        None => {
            if !min_given {
                return None;
            }
        }
    }

    NaiveTime::from_hms_opt(hour, min, 0)
}

/// converts 12-hour to 24-hour format
#[allow(clippy::integer_arithmetic)]
pub const fn to_24_hour(hour: u32, am_pm: AmPm) -> Option<u32> {
    if hour > 12 {
        return None;
    }

    Some(match am_pm {
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
    })
}
