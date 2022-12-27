use anyhow::Result;
use sparkle_convenience::reply::Reply;
use twilight_interactions::command::CreateCommand;
use twilight_model::channel::message::{embed::EmbedField, Embed};

use crate::{
    interaction::{CommandIds, InteractionContext},
    ACCENT_COLOR, BOT_INVITE, SUPPORT_SERVER_INVITE,
};

#[derive(CreateCommand)]
#[command(
    name = "help",
    desc = "Get info about the bot and learn why it might not be working"
)]
pub struct HelpCommandOptions {}

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
                name: "Get support".to_owned(),
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
                    "On mobile you can just hold on the message and click `Copy Text`, on PC, use \
                     the </copy:{}> command and copy-paste that magic code around",
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

impl InteractionContext<'_> {
    pub async fn handle_help_command(&self) -> Result<()> {
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
