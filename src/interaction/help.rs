use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_interactions::command::CreateCommand;

use crate::{
    database::UsageKind, interaction::InteractionContext, README_LINK, REQUIRED_PERMISSIONS,
};

#[derive(CreateCommand)]
#[command(
    name = "help",
    desc = "get info about the bot or learn why it might not be working"
)]
pub struct Command {}

fn help_reply() -> Reply {
    Reply::new().content(format!(
        "please check my page out for a list of features with gifs, info on contact, \
         troubleshooting, sponsors and how to sponsor me :pleading_face:\n{README_LINK}"
    ))
}

fn missing_permissions_reply() -> Reply {
    Reply::new().content(format!(
        "permissions error detected :scream: don't worry though! just click below for a \
         fix:\n{README_LINK}#missing-permissions"
    ))
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

        self.handle.reply(help_reply()).await?;

        self.ctx.insert_usage(UsageKind::Help).await?;
        Ok(())
    }
}
