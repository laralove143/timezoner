use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    database::UsageKind,
    interaction::{date, InteractionContext},
    CustomError,
};

pub const NAME: &str = "get current time for user";

pub fn command() -> Command {
    CommandBuilder::new(NAME, "", CommandType::User).build()
}

impl InteractionContext<'_> {
    pub async fn handle_current_time_command(self) -> Result<()> {
        let user_id = self.interaction.data.ok()?.command().ok()?.target_id.ok()?;

        self.handle
            .reply(
                Reply::new().ephemeral().content(
                    self.ctx
                        .user_time(user_id.cast(), date::Command::default())
                        .await
                        .map_err(|err| {
                            if let Some(CustomError::MissingTimezone(command_id)) =
                                err.downcast_ref()
                            {
                                CustomError::OtherUserMissingTimezone(*command_id).into()
                            } else {
                                err
                            }
                        })?
                        .format("%A, %B %-d, %-Y %-I:%M %p")
                        .to_string(),
                ),
            )
            .await?;

        self.ctx.insert_usage(UsageKind::CurrentTime).await?;
        Ok(())
    }
}
