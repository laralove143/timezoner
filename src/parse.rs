use std::fmt::Write;

use anyhow::{Context as _, IntoResult, Result};
use chrono::NaiveTime;
use logos::{Lexer, Logos};
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_mention::Mention;
use twilight_model::{
    channel::{ChannelType, Message},
    guild::Permissions,
    id::Id,
};
use twilight_webhook::util::{MinimalMember, MinimalWebhook};

use crate::{database, parse::token::Format, Context};

/// the reaction request with the unknown timezone emoji
const UNKNOWN_TIMEZONE_EMOJI: RequestReactionType = RequestReactionType::Custom {
    id: Id::new(950_033_075_440_603_186),
    name: Some("use_timezone_command"),
};

/// whether the time is am or pm, if 12-hour
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AmPm {
    /// time is am
    Am,
    /// time is pm
    Pm,
}

/// in a module to allow lints in the derive macro
#[allow(clippy::use_self, clippy::indexing_slicing)]
pub mod token {
    use chrono::{NaiveTime, Utc};
    use chrono_tz::Tz;
    use logos::Logos;
    use twilight_mention::timestamp::{Timestamp, TimestampStyle};

    use super::{hour_12, hour_12_minute, hour_24};

    /// the format of the time
    #[derive(Logos, Debug, PartialEq)]
    pub enum Format {
        /// a format like `14:33`
        #[regex(r"\d\d?:\d\d", hour_24)]
        Hour24(NaiveTime),
        /// a format like `11am`
        #[regex(r"\d\d? ?(?:am|pm|Am|Pm|AM|PM)", hour_12)]
        Hour12(NaiveTime),
        /// a format like `11:33am`
        #[regex(r"\d\d?:\d\d ?(?:am|pm|Am|Pm|AM|PM)", hour_12_minute)]
        Hour12Minute(NaiveTime),
        /// required by logos
        #[error]
        Error,
    }

    impl Format {
        /// returns the timestamp of the parsed time, if any
        pub fn timestamp(self, tz: Tz) -> Option<Timestamp> {
            match self {
                Format::Hour24(time) | Format::Hour12(time) | Format::Hour12Minute(time) => {
                    Some(Timestamp::new(
                        Utc::today()
                            .with_timezone(&tz)
                            .and_time(time)?
                            .timestamp()
                            .try_into()
                            .ok()?,
                        Some(TimestampStyle::ShortTime),
                    ))
                }
                Format::Error => None,
            }
        }
    }
}

/// adds discord formatted timestamp after times from message content and its
/// author's saved timezone and sends it to the message's channel impersonating
/// the author
#[allow(clippy::integer_arithmetic, unused_must_use)]
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
    {
        return Ok(());
    }

    let tz = if let Some(tz) = database::timezone(&ctx.db, message.author.id).await? {
        tz
    } else {
        ctx.http
            .create_reaction(message.channel_id, message.id, &UNKNOWN_TIMEZONE_EMOJI)
            .exec()
            .await?;
        return Ok(());
    };

    let lex = Format::lexer(&message.content).spanned();
    let mut content = String::with_capacity(message.content.len() + 70);
    let mut pushed_until = 0;
    for (format, span) in lex {
        if let Some(timestamp) = format.timestamp(tz) {
            content.push_str(message.content.get(pushed_until..span.end).ok()?);
            pushed_until = span.end;
            write!(content, " ({})", timestamp.mention());
        };
    }
    content.push_str(message.content.get(pushed_until..).ok()?);
    if content.is_empty() || content == message.content {
        return Ok(());
    }

    ctx.http
        .delete_message(message.channel_id, message.id)
        .exec()
        .await?;

    let webhook = ctx
        .webhooks
        .get_infallible(&ctx.http, message.channel_id, "time sender")
        .await?;

    MinimalWebhook::try_from(&*webhook)?
        .execute_as_member(
            &ctx.http,
            None,
            &MinimalMember::from_partial_member(
                &message
                    .member
                    .context("message doesn't have member attached")?,
                Some(message.guild_id.context("message isn't in a guild")?),
                &message.author,
            ),
        )
        .content(&content)?
        .exec()
        .await?;

    Ok(())
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

/// parses a time in 24 hour format
fn hour_24(lex: &mut Lexer<Format>) -> Option<NaiveTime> {
    let (hour, minute) = lex.slice().split_once(':')?;
    NaiveTime::from_hms_opt(hour.parse().ok()?, minute.parse().ok()?, 0)
}

/// parses a time in 12 hour format
fn hour_12(lex: &mut Lexer<Format>) -> Option<NaiveTime> {
    let slice = lex.slice();
    let (hour, _) = slice.split_once(&['a', 'A', 'p', 'P'])?;
    let am_pm = if slice.ends_with("am") || slice.ends_with("Am") || slice.ends_with("AM") {
        AmPm::Am
    } else {
        AmPm::Pm
    };
    NaiveTime::from_hms_opt(to_24_hour(hour.trim().parse().ok()?, am_pm)?, 0, 0)
}

/// parses a time with minute in 12 hour format
fn hour_12_minute(lex: &mut Lexer<Format>) -> Option<NaiveTime> {
    let (hour, after) = lex.slice().split_once(':')?;
    let minute = after.get(..2)?;
    let am_pm = if after.ends_with("am") || after.ends_with("Am") || after.ends_with("AM") {
        AmPm::Am
    } else {
        AmPm::Pm
    };
    NaiveTime::from_hms_opt(
        to_24_hour(hour.parse().ok()?, am_pm)?,
        minute.parse().ok()?,
        0,
    )
}
