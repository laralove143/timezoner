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
    desc = "Get info about the bot or learn why it might not be working"
)]
pub struct CommandOptions {}

fn help_embed(command_ids: CommandIds) -> Embed {
    Embed {
        color: Some(ACCENT_COLOR),
        fields: vec![
            EmbedField {
                name: "Add to your server (ugly link below)".to_owned(),
                value: BOT_INVITE.to_owned(),
                inline: false,
            },
            EmbedField {
                name: "Get support/Give feedback/Get updates/Meet Lara (the dev)/Join please \
                       :pleading_face:"
                    .to_owned(),
                value: SUPPORT_SERVER_INVITE.to_owned(),
                inline: false,
            },
            EmbedField {
                name: "How do I use this?".to_owned(),
                value: "Just type a time in the chat and hit that reaction I send to your message"
                    .to_owned(),
                inline: false,
            },
            EmbedField {
                name: "How do I send dates?".to_owned(),
                value: format!("Just use the </date:{}> command", command_ids.date),
                inline: false,
            },
            EmbedField {
                name: "How do I send these times in DMs or another server without the bot because \
                       that server sucks?"
                    .to_owned(),
                value: format!(
                    "On mobile you can just hold on the message and click **Copy Text**, on PC, \
                     use the </copy:{}> command and copy-paste that magic code around",
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
        title: Some("Permissions error detected, call tech support!".to_owned()),
        fields: vec![
            EmbedField {
                name: "Missing permissions".to_owned(),
                value: missing_permissions.prettify(),
                inline: false,
            },
            EmbedField {
                name: "How to fix??".to_owned(),
                value: format!(
                    "**1.** Go to **Server Settings -> Roles -> {application_name} -> \
                     Permissions**\n**2.** Turn on those permissions I listed\n**Oops.** If \
                     they're already given, read below and have fun\n"
                ),
                inline: false,
            },
            EmbedField {
                name: "How to fix?? (extreme edition)".to_owned(),
                value: format!(
                    "**1.** Right click the channel you're in right now\n**2.** Click **Edit \
                     Channel -> Permissions -> Advanced Permissions -> Tiny little + \
                     button**\n**3.** Type **{application_name}** and select the role or \
                     user\n**4.** Finally put all those permissions I listed into a nice looking \
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
