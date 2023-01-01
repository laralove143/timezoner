use anyhow::Result;
use chrono::{offset::Local, Datelike};
use sparkle_convenience::{
    error::{conversion::IntoError, ErrorExt},
    http::message::CreateMessageExt,
};
use twilight_http::request::channel::reaction::RequestReactionType;
use twilight_model::{
    channel::{message::ReactionType, Message},
    gateway::payload::incoming::ReactionAdd,
};

use crate::{err_reply, time::parse, Context, CustomError, REQUIRED_PERMISSIONS};

const REACTION_EMOJI: &str = "⏰";

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

            err.with_permissions(REQUIRED_PERMISSIONS);

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
        let Some((hour, min, range)) = parse(&message.content)? else {
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

        let now = Local::now();

        message.content.replace_range(
            range,
            &format!(
                "<t:{}:t>",
                self.user_timestamp(
                    message.author.id,
                    now.year(),
                    now.month(),
                    now.day(),
                    hour,
                    min
                )
                .await?
            ),
        );

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
