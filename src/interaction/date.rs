use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::interaction::InteractionContext;

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "date",
    desc = "send a date that everyone sees in their own timezone"
)]
pub struct Command {
    #[command(desc = "the day of the date", min_value = 0, max_value = 31)]
    pub day: i64,
    #[command(desc = "the month of the date", min_value = 0, max_value = 12)]
    pub month: i64,
    #[command(desc = "the year of the date", min_value = -262000, max_value = 262000)]
    pub year: i64,
    #[command(
        desc = "the hour of the date in 24 hour format",
        min_value = 0,
        max_value = 23
    )]
    pub hour: i64,
    #[command(desc = "the minute of the date", min_value = 0, max_value = 59)]
    pub min: i64,
    #[command(desc = "the second of the date", min_value = 0, max_value = 59)]
    pub sec: i64,
}

impl InteractionContext<'_> {
    pub async fn handle_date_command(self) -> Result<()> {
        let author_id = self.interaction.author_id().ok()?;
        let options =
            Command::from_interaction(self.interaction.data.ok()?.command().ok()?.into())?;

        self.handle
            .reply(Reply::new().content(format!(
                "<t:{}:F>",
                self.ctx.user_timestamp(author_id, options).await?
            )))
            .await?;

        Ok(())
    }
}
