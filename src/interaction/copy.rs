use anyhow::Result;
use sparkle_convenience::{
    error::conversion::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "copy",
    desc = "send a date that you can copy on desktop (on mobile just copy the message)"
)]
pub struct CommandOptions {
    #[command(desc = "the day of the date", min_value = 0, max_value = 31)]
    day: i64,
    #[command(desc = "the month of the date", min_value = 0, max_value = 12)]
    month: i64,
    #[command(desc = "the year of the date", min_value = -262000, max_value = 262000)]
    year: i64,
    #[command(
        desc = "the hour of the date in 24 hour format",
        min_value = 0,
        max_value = 23
    )]
    hour: i64,
    #[command(desc = "the minute of the date", min_value = 0, max_value = 59)]
    min: i64,
}

impl InteractionContext<'_> {
    pub async fn handle_copy_command(self) -> Result<()> {
        let author_id = self.interaction.author_id().ok()?;
        let options =
            CommandOptions::from_interaction(self.interaction.data.ok()?.command().ok()?.into())?;

        self.handle
            .reply(Reply::new().content(format!(
                    "`<t:{}:F>`",
                    self.ctx
                        .user_timestamp(
                            author_id,
                            options.year.try_into()?,
                            options.month.try_into()?,
                            options.day.try_into()?,
                            options.hour.try_into()?,
                            options.min.try_into()?
                        )
                        .await?
                )))
            .await?;

        Ok(())
    }
}
