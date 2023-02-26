use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{ApplicationCommandData, CommandModel, CreateCommand};

use crate::interaction::{date, InteractionContext};

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

        self.handle
            .reply(Reply::new().content(format!(
                "`{}`",
                self.ctx.user_timestamp(author_id, options).await?
            )))
            .await?;

        Ok(())
    }
}
