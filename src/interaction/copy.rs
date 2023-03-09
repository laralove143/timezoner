use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{ApplicationCommandData, CommandModel, CreateCommand};
use twilight_util::builder::embed::EmbedFooterBuilder;

use crate::{
    database::UsageKind,
    embed,
    interaction::{date, InteractionContext},
    time,
};

pub const NAME: &str = "copy";

pub fn command() -> ApplicationCommandData {
    let mut command = date::Command::create_command();
    command.name = NAME.to_owned();
    command.description =
        "send a date that you can copy on desktop (on mobile just copy the message)".to_owned();
    command
}

impl InteractionContext<'_> {
    pub async fn handle_copy_command(self) -> Result<()> {
        let author_id = self.interaction.author_id().ok()?;
        let options =
            date::Command::from_interaction(self.interaction.data.ok()?.command().ok()?.into())?;

        let time = self.ctx.user_time(author_id, options).await?;
        self.handle
            .reply(
                Reply::new().ephemeral().embed(
                    embed()
                        .title(":clipboard: to be copied")
                        .description(format!("`{}`", time::format(time, options.style)))
                        .footer(EmbedFooterBuilder::new(
                            "if you're using this to share a time in another server, consider \
                             adding me there :)",
                        ))
                        .build(),
                ),
            )
            .await?;

        self.ctx.insert_usage(UsageKind::Copy).await?;
        Ok(())
    }
}
