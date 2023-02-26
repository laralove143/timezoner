use std::{fmt::Write, time::Duration};

use anyhow::Result;
use chrono::{offset::Local, Datelike, TimeZone};
use sparkle_convenience::error::IntoError;
use tokio::time::timeout;
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_model::{
    channel::{message::ReactionType, Message},
    gateway::payload::incoming::ReactionAdd,
};

use crate::{err_reply, time::ParsedTime, Context, CustomError};

const REACTION_EMOJI: &str = "⏰";

impl Context {
    pub async fn handle_message(&self, message: Message) {
        if message.author.bot {
            return;
        }
        let channel_id = message.channel_id;

        let message_handle_result = self.handle_time_message(message).await;

        if let Err(err) = message_handle_result {
            self.bot
                .handle_error::<CustomError>(channel_id, err_reply(&err), err)
                .await;
        }
    }

    async fn handle_time_message(&self, mut message: Message) -> Result<()> {
        let parsed_times = ParsedTime::all_from_text(&message.content)?;
        if parsed_times.is_empty() {
            return Ok(());
        }

        let request_reaction_type = RequestReactionType::Unicode {
            name: REACTION_EMOJI,
        };

        self.bot
            .http
            .create_reaction(message.channel_id, message.id, &request_reaction_type)
            .await?;

        if timeout(Duration::from_secs(60 * 5), async {
            self.standby
                .wait_for_reaction(message.id, move |reaction: &ReactionAdd| {
                    reaction.user_id == message.author.id
                        && ReactionType::Unicode {
                            name: REACTION_EMOJI.to_owned(),
                        } == reaction.emoji
                })
                .await
        })
        .await
        .is_err()
        {
            self.bot
                .http
                .delete_reaction(
                    message.channel_id,
                    message.id,
                    &request_reaction_type,
                    self.bot.user.id,
                )
                .await?;
        }

        let now = Local::now();
        let Some(tz) = self.timezone(message.author.id).await? else {
            return Err(CustomError::MissingTimezone(self.command_ids.timezone).into());
        };

        let mut content = String::new();
        let mut push_start = 0;
        for time in parsed_times {
            write!(
                content,
                "{}<t:{}:t>",
                message.content.get(push_start..time.range.start).ok()?,
                tz.with_ymd_and_hms(now.year(), now.month(), now.day(), time.hour, time.min, 0)
                    .single()
                    .ok()?
                    .timestamp()
            )?;
            push_start = time.range.end;
        }
        content.push_str(message.content.get(push_start..).ok()?);
        message.content = content;

        self.bot
            .http
            .delete_message(message.channel_id, message.id)
            .await?;

        self.execute_webhook_as_member(message).await?;

        Ok(())
    }

    async fn execute_webhook_as_member(&self, message: Message) -> Result<()> {
        let mut channel_id = message.channel_id;
        let mut thread_id = None;
        let channel = self
            .bot
            .http
            .channel(message.channel_id)
            .await?
            .model()
            .await?;
        if channel.kind.is_thread() {
            channel_id = channel.parent_id.ok()?;
            thread_id = Some(message.channel_id);
        };

        let webhook = match self
            .bot
            .http
            .channel_webhooks(channel_id)
            .await?
            .models()
            .await?
            .into_iter()
            .find(|webhook| webhook.token.is_some())
        {
            Some(webhook) => webhook,
            None => {
                self.bot
                    .http
                    .create_webhook(channel_id, "time sender")?
                    .await?
                    .model()
                    .await?
            }
        };
        let webhook_token = webhook.token.ok()?;

        let mut execute_webhook = self
            .bot
            .http
            .execute_webhook(webhook.id, &webhook_token)
            .content(&message.content)
            .map_err(|_| CustomError::MessageTooLong)?
            .username(
                message
                    .member
                    .as_ref()
                    .and_then(|member| member.nick.as_ref())
                    .unwrap_or(&message.author.name),
            )?;

        if let Some(thread_id) = thread_id {
            execute_webhook = execute_webhook.thread_id(thread_id);
        }

        if let Some(avatar_url) = message
            .member
            .as_ref()
            .and_then(|member| member.avatar)
            .zip(message.guild_id)
            .map(|(avatar, guild_id)| {
                format!(
                    "https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatar/{}.png",
                    message.author.id, avatar
                )
            })
            .or_else(|| {
                message.author.avatar.map(|avatar| {
                    format!(
                        "https://cdn.discordapp.com/avatars/{}/{}.png",
                        message.author.id, avatar
                    )
                })
            })
        {
            execute_webhook.avatar_url(&avatar_url).await?;
        } else {
            execute_webhook.await?;
        }

        Ok(())
    }
}
