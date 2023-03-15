use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    channel::message::{
        component::{ActionRow, Button, ButtonStyle},
        Component, Embed, ReactionType,
    },
    user::CurrentUser,
};
use twilight_util::builder::embed::{EmbedFooterBuilder, ImageSource};

use crate::{
    database::UsageKind, embed, interaction::InteractionContext, message::avatar_url,
    MISSING_PERMISSIONS_LINK, README_LINK, REQUIRED_PERMISSIONS, SUPPORT_SERVER_INVITE,
};

#[derive(CreateCommand)]
#[command(
    name = "help",
    desc = "get info about the bot or learn why it might not be working"
)]
pub struct Command {}

fn page_button() -> Component {
    Component::Button(Button {
        emoji: Some(ReactionType::Unicode {
            name: "📖".to_owned(),
        }),
        label: Some("the page of infinite knowledge (about the bot)".to_owned()),
        url: Some(README_LINK.to_owned()),
        style: ButtonStyle::Link,
        custom_id: None,
        disabled: false,
    })
}

pub fn server_button() -> Component {
    Component::Button(Button {
        emoji: Some(ReactionType::Unicode {
            name: "💬".to_owned(),
        }),
        label: Some("the server that you need to be in".to_owned()),
        url: Some(SUPPORT_SERVER_INVITE.to_owned()),
        style: ButtonStyle::Link,
        custom_id: None,
        disabled: false,
    })
}

fn missing_permissions_button() -> Component {
    Component::Button(Button {
        emoji: Some(ReactionType::Unicode {
            name: "🔧".to_owned(),
        }),
        label: Some("(hopefully) simple fix".to_owned()),
        url: Some(MISSING_PERMISSIONS_LINK.to_owned()),
        style: ButtonStyle::Link,
        custom_id: None,
        disabled: false,
    })
}

fn help_embed(current_user: &CurrentUser) -> Embed {
    embed()
        .title(":sos: support has arrived!")
        .description(
            "please use that page button below, it gives more info than i could ever give here!",
        )
        .thumbnail(
            ImageSource::url(avatar_url(
                None,
                current_user.avatar,
                current_user.id,
                None,
                current_user.discriminator,
            ))
            .unwrap(),
        )
        .footer(EmbedFooterBuilder::new("thank you for using me :)"))
        .build()
}

fn missing_permissions_embed() -> Embed {
    embed()
        .title(":scream: permissions error detected!")
        .description("don't worry, click the button")
        .footer(EmbedFooterBuilder::new("and be happy"))
        .build()
}

impl InteractionContext<'_> {
    pub async fn handle_help_command(&self) -> Result<()> {
        if let Some(app_permissions) = self.interaction.app_permissions {
            let missing_permissions = REQUIRED_PERMISSIONS - app_permissions;

            if !missing_permissions.is_empty() {
                self.handle
                    .reply(Reply::new().embed(missing_permissions_embed()).component(
                        Component::ActionRow(ActionRow {
                            components: vec![missing_permissions_button()],
                        }),
                    ))
                    .await?;

                return Ok(());
            }
        }

        self.handle
            .reply(
                Reply::new()
                    .embed(help_embed(&self.ctx.bot.user))
                    .component(Component::ActionRow(ActionRow {
                        components: vec![page_button(), server_button()],
                    })),
            )
            .await?;

        self.ctx.insert_usage(UsageKind::Help).await?;
        Ok(())
    }
}
