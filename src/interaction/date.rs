use anyhow::Result;
use sparkle_convenience::{
    error::conversion::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "date",
    desc = "Send a date that everyone sees in their own timezone"
)]
pub struct DateCommandOptions {
    #[command(desc = "The day of the date", min_value = 0, max_value = 31)]
    day: i64,
    #[command(desc = "The month of the date", min_value = 0, max_value = 12)]
    month: i64,
    #[command(desc = "the year of the date", min_value = -262000, max_value = 262000)]
    year: i64,
    #[command(
        desc = "The hour of the date in 24-hour format",
        min_value = 0,
        max_value = 23
    )]
    hour: i64,
    #[command(desc = "The minute of the date", min_value = 0, max_value = 59)]
    min: i64,
}

impl InteractionContext<'_> {
    pub async fn handle_date_command(self) -> Result<()> {
        let author_id = self.interaction.author_id().ok()?;
        let options = DateCommandOptions::from_interaction(
            self.interaction.data.ok()?.command().ok()?.into(),
        )?;

        self.handle
            .reply(Reply::new().content(format!(
                "<t:{}:F>",
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
