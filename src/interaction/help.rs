use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_interactions::command::CreateCommand;
use twilight_model::user::CurrentUser;
use twilight_util::builder::embed::{EmbedFieldBuilder, EmbedFooterBuilder, ImageSource};

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

fn help_reply(current_user: &CurrentUser) -> Reply {
    Reply::new().embed(
        embed()
            .title("tap tap click tap here!")
            .url(README_LINK)
            .description("that page above has more info than i could ever say here!")
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
            .field(EmbedFieldBuilder::new(
                "*the* server",
                SUPPORT_SERVER_INVITE,
            ))
            .footer(EmbedFooterBuilder::new("thank you for using me :)"))
            .build(),
    )
}

fn missing_permissions_reply() -> Reply {
    Reply::new().embed(
        embed()
            .title(":scream: permissions error detected, press here for a fix!")
            .url(MISSING_PERMISSIONS_LINK)
            .build(),
    )
}

impl InteractionContext<'_> {
    pub async fn handle_help_command(&self) -> Result<()> {
        if let Some(app_permissions) = self.interaction.app_permissions {
            let missing_permissions = REQUIRED_PERMISSIONS - app_permissions;

            if !missing_permissions.is_empty() {
                self.handle.reply(missing_permissions_reply()).await?;

                return Ok(());
            }
        }

        self.handle.reply(help_reply(&self.ctx.bot.user)).await?;

        self.ctx.insert_usage(UsageKind::Help).await?;
        Ok(())
    }
}
