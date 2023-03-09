use std::{fmt::Write, future::IntoFuture, time::Duration};

use anyhow::Result;
use chrono::{offset::Local, Datelike, TimeZone};
use sparkle_convenience::error::{ErrorExt, IntoError};
use tokio::time::timeout;
use twilight_http::{
    request::channel::reaction::RequestReactionType,
    response::{marker::EmptyBody, ResponseFuture},
    Response,
};
use twilight_model::{
    channel::{message::ReactionType, Message},
    gateway::payload::incoming::ReactionAdd,
    id::{
        marker::{GuildMarker, UserMarker},
        Id,
    },
    util::ImageHash,
};

use crate::{database::UsageKind, err_reply, time::ParsedTime, Context, CustomError};

const REACTION_EMOJI: &str = "⏰";

impl Context {
    pub async fn handle_message(&self, message: Message) {
        if message.author.bot {
            return;
        }
        let channel_id = message.channel_id;

        let message_handle_result = self.handle_time_message(message).await;

        if let Err(err) = message_handle_result {
            let maybe_err_response = self
                .bot
                .handle_error::<CustomError>(channel_id, err_reply(&err), err)
                .await;

            if let Some(err_response) = maybe_err_response {
                if let Err(Some(delete_err)) = self
                    .delete_err_response(err_response)
                    .await
                    .map_err(ErrorExt::internal::<CustomError>)
                {
                    self.bot.log(delete_err).await;
                }
            }
        }
    }

    async fn delete_err_response(&self, response: Response<Message>) -> Result<()> {
        let message = response.model().await?;

        tokio::time::sleep(Duration::from_secs(60 * 5)).await;

        self.bot
            .http
            .delete_message(message.channel_id, message.id)
            .await?;

        Ok(())
    }

    async fn handle_time_message(&self, mut message: Message) -> Result<()> {
        let parsed_times = ParsedTime::all_from_text(&message.content)?;
        if parsed_times.is_empty() {
            return Ok(());
        }
        self.insert_usage(UsageKind::TimeDetect).await?;

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
            return Ok(());
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

        let message_channel_id = message.channel_id;
        let message_id = message.id;

        let exec_webhook = self.execute_webhook_as_member(message).await?;

        self.bot
            .http
            .delete_message(message_channel_id, message_id)
            .await?;

        exec_webhook.await?;

        self.insert_usage(UsageKind::TimeConvert).await?;
        Ok(())
    }

    async fn execute_webhook_as_member(
        &self,
        message: Message,
    ) -> Result<ResponseFuture<EmptyBody>> {
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

        let username = message
            .member
            .as_ref()
            .and_then(|member| member.nick.as_ref())
            .map_or(message.author.name, |nick| {
                if nick.len() == 1 {
                    format!("{nick}{REACTION_EMOJI}")
                } else {
                    nick.clone()
                }
            });
        let mut execute_webhook = self
            .bot
            .http
            .execute_webhook(webhook.id, &webhook_token)
            .content(&message.content)
            .map_err(|_| CustomError::MessageTooLong)?
            .username(&username)?;

        if let Some(thread_id) = thread_id {
            execute_webhook = execute_webhook.thread_id(thread_id);
        }

        Ok(execute_webhook
            .avatar_url(&avatar_url(
                message.member.and_then(|member| member.avatar),
                message.author.avatar,
                message.author.id,
                message.guild_id,
                message.author.discriminator,
            ))
            .into_future())
    }
}

pub fn avatar_url(
    member_avatar: Option<ImageHash>,
    user_avatar: Option<ImageHash>,
    user_id: Id<UserMarker>,
    guild_id: Option<Id<GuildMarker>>,
    user_discriminator: u16,
) -> String {
    member_avatar.zip(guild_id).map_or_else(
        || {
            user_avatar
                .map(|avatar| {
                    format!(
                        "https://cdn.discordapp.com/avatars/{}/{}.{}",
                        user_id,
                        avatar,
                        if avatar.is_animated() { "gif" } else { "png" }
                    )
                })
                .unwrap_or(format!(
                    "https://cdn.discordapp.com/embed/avatars/{}.png",
                    user_discriminator % 5
                ))
        },
        |(avatar, guild_id)| {
            format!(
                "https://cdn.discordapp.com/guilds/{guild_id}/users/{}/avatar/{}.{}",
                user_id,
                avatar,
                if avatar.is_animated() { "gif" } else { "png" }
            )
        },
    )
}
