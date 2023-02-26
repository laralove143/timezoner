use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

use crate::interaction::InteractionContext;

#[derive(CommandOption, CreateOption)]
pub enum Style {
    #[option(name = "4:20 PM", value = "t")]
    ShortTime,
    #[option(name = "4:20:30 PM", value = "T")]
    LongTime,
    #[option(name = "20/04/2021", value = "d")]
    ShortDate,
    #[option(name = "20 April 2021", value = "D")]
    LongDate,
    #[option(name = "20 April 2021 4:20 PM", value = "f")]
    ShortDateTime,
    #[option(name = "Tuesday, 20 April 2021 4:20 PM", value = "F")]
    LongDateTime,
    #[option(name = "in an hour", value = "R")]
    Relative,
}

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
    pub minute: i64,
    #[command(desc = "the second of the date", min_value = 0, max_value = 59)]
    pub second: i64,
    #[command(desc = "the style of the date, by default like Tuesday, 20 April 2021 4:20 PM")]
    pub style: Option<Style>,
}

impl InteractionContext<'_> {
    pub async fn handle_date_command(self) -> Result<()> {
        let author_id = self.interaction.author_id().ok()?;
        let options =
            Command::from_interaction(self.interaction.data.ok()?.command().ok()?.into())?;

        self.handle
            .reply(Reply::new().content(self.ctx.user_timestamp(author_id, options).await?))
            .await?;

        Ok(())
    }
}
