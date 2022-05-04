use anyhow::{bail, Context as _, IntoResult, Result};
use chrono::Utc;
use chrono_tz::Tz;
use regex::{Captures, Regex};
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

use crate::{database, webhooks, Context};

/// the reaction request with the unknown timezone emoji
const UNKNOWN_TIMEZONE_EMOJI: RequestReactionType = RequestReactionType::Custom {
    id: Id::new(950_033_075_440_603_186),
    name: Some("use_timezone_command"),
};

/// whether the time is am or pm, if 12-hour
#[derive(Clone, Copy)]
pub enum AmPm {
    /// time is am
    Am,
    /// time is pm
    Pm,
}

/// whether the time is 12-hour or 24-hour format
#[derive(Clone, Copy)]
pub enum Format {
    /// time is in 12-hour format
    Hour12,
    /// time is in 24-hour format
    Hour24,
}

/// returns the regex to parse times in 12 hour format
pub fn regex_12_hour() -> Result<Regex> {
    Ok(Regex::new(
        r"(?:^|\s)(?P<hour>1[0-2]|0?[1-9])(?::(?P<minute>[0-5]\d))?\s*(?P<am_pm>am|pm)\b",
    )?)
}

/// returns the regex to parse times in 24 hour format
pub fn regex_24_hour() -> Result<Regex> {
    Ok(Regex::new(
        r"(?:^|\s)(?P<hour>[0-1]?\d|2[0-3]):(?P<minute>[0-5]\d)\b",
    )?)
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
    {
        return Ok(());
    }

    let mut top_captures: Vec<(Captures, Format)> = ctx
        .regex_12_hour
        .captures_iter(&message.content)
        .map(|captures| (captures, Format::Hour12))
        .chain(
            ctx.regex_24_hour
                .captures_iter(&message.content)
                .map(|captures| (captures, Format::Hour24)),
        )
        .collect();

    top_captures.sort_unstable_by(|old, new| {
        old.0
            .get(0)
            .map_or(0, |m| m.start())
            .cmp(&new.0.get(0).map_or(0, |m| m.start()))
    });

    if top_captures.is_empty() {
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

    let mut content = String::with_capacity(70);
    let mut last_pushed_idx = 0;
    for captures in top_captures {
        let timestamp = time(&captures.0, captures.1, tz)?;

        let push_idx = captures.0.iter().last().ok()?.ok()?.end();
        let old_content = message.content.get(last_pushed_idx..push_idx).ok()?;
        last_pushed_idx = push_idx;
        content.push_str(&format!(
            "{} ({} in your timezone)",
            old_content,
            timestamp.mention()
        ));
    }
    content.push_str(message.content.get(last_pushed_idx..).ok()?);

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
    )
    .await?;

    Ok(())
}

/// parses a time from captures
pub fn time(captures: &Captures, format: Format, tz: Tz) -> Result<Timestamp> {
    match format {
        Format::Hour12 => parse_12_hour(captures, tz),
        Format::Hour24 => parse_24_hour(captures, tz),
    }
}

/// parses a time in 24 hour format from captures
fn parse_24_hour(captures: &Captures, tz: Tz) -> Result<Timestamp> {
    let hour: u32 = captures.name("hour").ok()?.as_str().parse()?;
    let minute: u32 = captures
        .name("minute")
        .map_or("0", |m| m.as_str())
        .parse()?;

    Ok(Timestamp::new(
        Utc::today()
            .with_timezone(&tz)
            .and_hms_opt(hour, minute, 0)
            .ok()?
            .timestamp()
            .try_into()?,
        Some(TimestampStyle::ShortTime),
    ))
}

/// parses a time in 12 hour format from captures
fn parse_12_hour(captures: &Captures, tz: Tz) -> Result<Timestamp> {
    let hour: u32 = captures.name("hour").ok()?.as_str().parse()?;
    let minute: u32 = captures
        .name("minute")
        .map_or("0", |m| m.as_str())
        .parse()?;
    let am_pm = match captures
        .name("am_pm")
        .ok()?
        .as_str()
        .to_lowercase()
        .as_ref()
    {
        "am" => AmPm::Am,
        "pm" => AmPm::Pm,
        _ => bail!("am_pm capture isn't am or pm"),
    };

    Ok(Timestamp::new(
        Utc::today()
            .with_timezone(&tz)
            .and_hms_opt(to_24_hour(hour, am_pm).ok()?, minute, 0)
            .ok()?
            .timestamp()
            .try_into()?,
        Some(TimestampStyle::ShortTime),
    ))
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
