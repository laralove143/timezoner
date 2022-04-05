use anyhow::{Context as _, Result};
use dashmap::mapref::one::Ref;
use twilight_model::{
    channel::webhook::Webhook,
    guild::PartialMember,
    id::{
        marker::{ChannelMarker, WebhookMarker},
        Id,
    },
    user::User,
};

use crate::Context;

/// a cached webhook
#[derive(Debug)]
pub struct CachedWebhook {
    /// the webhook's id
    pub id: Id<WebhookMarker>,
    /// the webhook's string
    pub token: String,
}

impl TryFrom<Webhook> for CachedWebhook {
    type Error = anyhow::Error;

    fn try_from(webhook: Webhook) -> Result<Self, Self::Error> {
        Ok(Self {
            id: webhook.id,
            token: webhook
                .token
                .context("the webhook is not an incoming webhook")?,
        })
    }
}

/// get a webhook from the cache, falling back to the http api, then to creating
/// the webhook
pub async fn get(
    ctx: &Context,
    channel_id: Id<ChannelMarker>,
) -> Result<Ref<'_, Id<ChannelMarker>, CachedWebhook>> {
    if let Some(pair) = ctx.webhooks.get(&channel_id) {
        Ok(pair)
    } else {
        let webhook = match ctx
            .http
            .channel_webhooks(channel_id)
            .exec()
            .await?
            .models()
            .await?
            .into_iter()
            .find(|webhook| webhook.application_id == Some(ctx.application_id))
        {
            Some(webhook) => webhook,
            None => {
                ctx.http
                    .create_webhook(channel_id, "tw or tag sender")
                    .exec()
                    .await?
                    .model()
                    .await?
            }
        };

        ctx.webhooks.insert(channel_id, webhook.try_into()?);
        Ok(ctx
            .webhooks
            .get(&channel_id)
            .context("just inserted webhook doesn't exist")?)
    }
}

/// remove the inexistent webhooks from the cache
pub async fn update(ctx: Context, channel_id: Id<ChannelMarker>) -> Result<()> {
    if let Some(webhook) = ctx.webhooks.get(&channel_id) {
        if !ctx
            .http
            .channel_webhooks(channel_id)
            .exec()
            .await?
            .models()
            .await?
            .iter()
            .any(|w| w.id == webhook.id)
        {
            ctx.webhooks.remove(&channel_id);
        }
    };

    Ok(())
}

/// send a webhook with the member's avatar and nick
pub async fn send_as_member(
    ctx: &Context,
    channel_id: Id<ChannelMarker>,
    member: &PartialMember,
    user: &User,
    content: &str,
) -> Result<()> {
    let channel = ctx
        .cache
        .channel(channel_id)
        .context("channel is not cached")?;
    let is_thread = channel.kind.is_thread();

    let webhook = if is_thread {
        get(
            ctx,
            channel
                .parent_id
                .context("thread channel doesn't have parent id")?,
        )
        .await?
    } else {
        get(ctx, channel_id).await?
    };

    let mut exec = ctx
        .http
        .execute_webhook(webhook.id, &webhook.token)
        .username(member.nick.as_ref().unwrap_or(&user.name))
        .content(content)?;

    if is_thread {
        exec = exec.thread_id(channel_id);
    }

    match member.avatar.or(user.avatar) {
        Some(avatar) => {
            exec.avatar_url(&format!(
                "https://cdn.discordapp.com/avatars/{}/{}.png",
                user.id, avatar
            ))
            .exec()
            .await?
        }
        None => exec.exec().await?,
    };

    Ok(())
}
