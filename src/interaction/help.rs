use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_interactions::command::CreateCommand;

use crate::{interaction::InteractionContext, README_LINK, REQUIRED_PERMISSIONS};

#[derive(CreateCommand)]
#[command(
    name = "help",
    desc = "get info about the bot or learn why it might not be working"
)]
pub struct Command {}

fn help_reply() -> Reply {
    Reply::new().content(format!(
        "my dev worked a lot for my page :pleading_face: so please check that out:\n{README_LINK}"
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

        Ok(())
    }
}
