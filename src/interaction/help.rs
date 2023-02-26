use anyhow::Result;
use sparkle_convenience::{prettify::Prettify, reply::Reply};
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    channel::message::{embed::EmbedField, Embed},
    guild::Permissions,
};

use crate::{
    interaction::{CommandIds, InteractionContext},
    ACCENT_COLOR, BOT_INVITE, REQUIRED_PERMISSIONS, SUPPORT_SERVER_INVITE,
};

#[derive(CreateCommand)]
#[command(
    name = "help",
    desc = "get info about the bot or learn why it might not be working"
)]
pub struct Command {}

fn help_embed(command_ids: CommandIds) -> Embed {
    Embed {
        color: Some(ACCENT_COLOR),
        fields: vec![
            EmbedField {
                name: "add to your server (ugly link below)".to_owned(),
                value: BOT_INVITE.to_owned(),
                inline: false,
            },
            EmbedField {
                name: "get support/give feedback/get updates/meet lara (the dev)/join please \
                       :pleading_face:"
                    .to_owned(),
                value: SUPPORT_SERVER_INVITE.to_owned(),
                inline: false,
            },
            EmbedField {
                name: "how do i use this?".to_owned(),
                value: "just type a time in the chat and hit that reaction i send to your message"
                    .to_owned(),
                inline: false,
            },
            EmbedField {
                name: "how do i send dates?".to_owned(),
                value: format!("just use the </date:{}> command", command_ids.date),
                inline: false,
            },
            EmbedField {
                name: "how do i send these times in dms or another server without the bot because \
                       that server sucks?"
                    .to_owned(),
                value: format!(
                    "on mobile you can just hold on the message and click **copy text**, on pc, \
                     use the </copy:{}> command and copy paste that magic code around",
                    command_ids.copy
                ),
                inline: false,
            },
        ],
        kind: String::new(),
        author: None,
        description: None,
        footer: None,
        image: None,
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        title: None,
        video: None,
    }
}

fn missing_permissions_embed(application_name: &str, missing_permissions: Permissions) -> Embed {
    Embed {
        color: Some(ACCENT_COLOR),
        title: Some("permissions error detected, call tech support!".to_owned()),
        fields: vec![
            EmbedField {
                name: "missing permissions".to_owned(),
                value: missing_permissions.prettify(),
                inline: false,
            },
            EmbedField {
                name: "how to fix??".to_owned(),
                value: format!(
                    "**1.** go to **server settings -> roles -> {application_name} -> \
                     permissions**\n**2.** turn on those permissions i listed\n**oops.** if \
                     they're already given, read below and have fun\n"
                ),
                inline: false,
            },
            EmbedField {
                name: "how to fix?? (extreme edition)".to_owned(),
                value: format!(
                    "**1.** right click the channel you're in right now\n**2.** click **edit \
                     channel -> permissions -> advanced permissions -> tiny little + \
                     button**\n**3.** type **{application_name}** and select the role or \
                     user\n**4.** finally put all those permissions i listed into a nice looking \
                     green tick"
                ),
                inline: false,
            },
        ],
        kind: String::new(),
        author: None,
        description: None,
        footer: None,
        image: None,
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        video: None,
    }
}

impl InteractionContext<'_> {
    pub async fn handle_help_command(&self) -> Result<()> {
        if let Some(app_permissions) = self.interaction.app_permissions {
            let missing_permissions = REQUIRED_PERMISSIONS - app_permissions;

            if !missing_permissions.is_empty() {
                self.handle
                    .reply(Reply::new().embed(missing_permissions_embed(
                        &self.ctx.bot.application.name,
                        missing_permissions,
                    )))
                    .await?;

                return Ok(());
            }
        }

        self.handle
            .reply(
                Reply::new()
                    .ephemeral()
                    .embed(help_embed(self.ctx.command_ids)),
            )
            .await?;

        Ok(())
    }
}
