use std::str::CharIndices;

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
    interaction::{action_row, copy_button, time::AmPm},
    webhooks, Context,
};

/// the reaction request with the unknown timezone emoji
const UNKNOWN_TIMEZONE_EMOJI: RequestReactionType = RequestReactionType::Custom {
    id: Id::new(950_033_075_440_603_186),
    name: Some("use_set_timezone_command"),
};

/// whether the time is am or pm, if 12-hour
#[derive(Clone, Copy)]
pub enum AmPm {
    /// time is am
    Am,
    /// time is pm
    Pm,
}

/// adds discord formatted timestamp after times from message content and its
/// author's saved timezone and sends it to the message's channel impersonating
/// the author
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
                Permissions::MANAGE_WEBHOOKS
                    | Permissions::MANAGE_MESSAGES
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

    let timezone = database::timezone(&ctx.db, message.author.id).await?;

    let mut chars = message.content.char_indices();
    let mut pushed_idx = 0;
    let mut content = "".to_owned();
    while chars.size_hint().0 != 0 {
        if let Some(time) = try_time(&mut chars) {
            let tz = if let Some(tz) = timezone {
                tz
            } else {
                ctx.http
                    .create_reaction(message.channel_id, message.id, &UNKNOWN_TIMEZONE_EMOJI)
                    .exec()
                    .await?;
                return Ok(());
            };

            let idx = chars.next().map_or(message.content.len(), |(idx, _)| idx);
            content.push_str(
                message
                    .content
                    .get(pushed_idx..idx)
                    .context("chars index is larger than string length")?,
            );
            pushed_idx = idx;
            content.push_str(&format!(
                " *({} in your clock)*",
                Timestamp::new(
                    Utc::today()
                        .with_timezone(&tz)
                        .and_time(time)
                        .context("parsed naive time is invalid")?
                        .timestamp()
                        .try_into()?,
                    Some(TimestampStyle::ShortTime),
                )
                .mention()
            ));
        }
    }

    if content.is_empty() {
        return Ok(());
    }

    content.push_str(
        message
            .content
            .get(pushed_idx..message.content.len())
            .context("message content length is larger than message content length?")?,
    );

    ctx.http
        .delete_message(message.channel_id, message.id)
        .exec()
        .await?;

    webhooks::send_as_member(
        &ctx,
        message.channel_id,
        &message
            .member
            .context("message doesn't have member attached")?,
        &message.author,
        &content,
        Some(&[action_row(vec![copy_button()])]),
    )
    .await?;

    Ok(())
}

/// parses one possible occurrence of a time, returning that and the index thats
/// iterated over so far
#[allow(clippy::integer_arithmetic)]
pub fn try_time(chars: &mut CharIndices) -> Option<NaiveTime> {
    let mut current;
    let mut min_given = false;

    let mut hour = chars.find_map(|(_, c)| c.to_digit(10))?;
    let mut min = 0;

    current = chars.next()?;
    if let Some(digit) = current.1.to_digit(10) {
        hour = hour * 10 + digit;
        current = chars.next()?;
    }

    if current.1 == ':' {
        min = chars.next()?.1.to_digit(10)? * 10 + chars.next()?.1.to_digit(10)?;
        current = match chars.next() {
            Some(c) => c,
            None => return NaiveTime::from_hms_opt(hour, min, 0),
        };
        min_given = true;
    }

    if current.1 == ' ' {
        current = chars.next()?;
    }

    match chars.next().and_then(|(_, char)| {
        if let 'm' | 'M' = char {
            if let 'a' | 'A' = current.1 {
                Some(AmPm::Am)
            } else if let 'p' | 'P' = current.1 {
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
