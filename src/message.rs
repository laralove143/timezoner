use std::ops::Range;

use anyhow::{anyhow, Result};
use chrono::{offset::Local, Datelike, TimeZone};
use lazy_regex::{lazy_regex, Lazy, Regex};
use sparkle_convenience::{
    error::{conversion::IntoError, ErrorExt},
    http::message::CreateMessageExt,
};
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_model::{
    channel::{message::ReactionType, Message},
    gateway::payload::incoming::ReactionAdd,
    guild::Permissions,
};

use crate::{err_reply, Context, CustomError};

static REGEX_24_HOUR: Lazy<Regex> = lazy_regex!(r#"\b([0-1]?[0-9]|2[0-3]):([0-5][0-9])\b"#);
static REGEX_12_HOUR: Lazy<Regex> = lazy_regex!(r#"\b(1[0-2]|0?[1-9]) ?([AaPp][Mm])\b"#);
static REGEX_12_HOUR_WITH_MIN: Lazy<Regex> =
    lazy_regex!(r#"\b(1[0-2]|0?[1-9]):([0-5][0-9]) ?([AaPp][Mm])\b"#);
const REACTION_EMOJI: &str = "⏰";

fn required_permissions() -> Permissions {
    Permissions::MANAGE_MESSAGES | Permissions::ADD_REACTIONS | Permissions::MANAGE_WEBHOOKS
}

impl Context {
    pub async fn handle_message(&self, message: Message) -> Result<()> {
        if message.author.bot {
            return Ok(());
        }
        let channel_id = message.channel_id;

        if let Err(mut err) = self.handle_time_message(message).await {
            if err.ignore() {
                return Ok(());
            }

            err.with_permissions(required_permissions());

            self.bot
                .http
                .create_message(channel_id)
                .with_reply(&err_reply(&err)?)?
                .execute_ignore_permissions()
                .await?;

            if let Some(err) = err.internal::<CustomError>() {
                return Err(err);
            }
        };

        Ok(())
    }

    async fn handle_time_message(&self, mut message: Message) -> Result<()> {
        let Some((hour, minute, range)) = parse_time(&message.content)? else {
            return Ok(());
        };

        self.bot
            .http
            .create_reaction(
                message.channel_id,
                message.id,
                &RequestReactionType::Unicode {
                    name: REACTION_EMOJI,
                },
            )
            .await?;

        self.standby
            .wait_for_reaction(message.id, move |reaction: &ReactionAdd| {
                reaction.user_id == message.author.id
                    && ReactionType::Unicode {
                        name: REACTION_EMOJI.to_owned(),
                    } == reaction.emoji
            })
            .await?;

        let Some(tz) = self.timezone(message.author.id).await? else {
            return Err(CustomError::MissingTimezone(self.timezone_command_id()?).into());
        };

        let now = Local::now();

        message.content.replace_range(
            range,
            &format!(
                "<t:{}:t>",
                tz.with_ymd_and_hms(now.year(), now.month(), now.day(), hour, minute, 0)
                    .single()
                    .ok()?
                    .timestamp()
            ),
        );

        self.bot
            .http
            .delete_message(message.channel_id, message.id)
            .await?;

        self.bot
            .http
            .create_message(message.channel_id)
            .content(&message.content)?
            .await?;

        Ok(())
    }
}

fn parse_time(s: &str) -> Result<Option<(u32, u32, Range<usize>)>> {
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
