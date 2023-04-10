use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::{CommandModel, CommandOption, CreateCommand, CreateOption};

use crate::{database::UsageKind, interaction::InteractionContext, time};

#[derive(Clone, Copy, CommandOption, CreateOption)]
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

#[derive(Clone, Copy, CommandOption, CreateOption)]
pub enum Month {
    #[option(name = "january", value = 1)]
    January,
    #[option(name = "february", value = 2)]
    February,
    #[option(name = "march", value = 3)]
    March,
    #[option(name = "april", value = 4)]
    April,
    #[option(name = "may", value = 5)]
    May,
    #[option(name = "june", value = 6)]
    June,
    #[option(name = "july", value = 7)]
    July,
    #[option(name = "august", value = 8)]
    August,
    #[option(name = "september", value = 9)]
    September,
    #[option(name = "october", value = 10)]
    October,
    #[option(name = "november", value = 11)]
    November,
    #[option(name = "december", value = 12)]
    December,
}

#[derive(Clone, Copy, Default, CommandModel, CreateCommand)]
#[command(
    name = "date",
    desc = "send a date that everyone sees in their own timezone"
)]
pub struct Command {
    #[command(
        desc = "the day of the date, today by default",
        min_value = 0,
        max_value = 31
    )]
    pub day: Option<i64>,
    #[command(
        desc = "the month of the date, this month by default",
        min_value = 0,
        max_value = 12
    )]
    pub month: Option<Month>,
    #[command(
        desc = "the year of the date, this year by default",
        min_value = -262000,
        max_value = 262000
    )]
    pub year: Option<i64>,
    #[command(
        desc = "the hour of the date in 24 hour format, current hour by default",
        min_value = 0,
        max_value = 23
    )]
    pub hour: Option<i64>,
    #[command(
        desc = "the minute of the date, current minute by default",
        min_value = 0,
        max_value = 59
    )]
    pub minute: Option<i64>,
    #[command(
        desc = "the second of the date, current second by default",
        min_value = 0,
        max_value = 59
    )]
    pub second: Option<i64>,
    #[command(desc = "the style of the date, by default like Tuesday, 20 April 2021 4:20 PM")]
    pub style: Option<Style>,
}

impl InteractionContext<'_> {
    pub async fn handle_date_command(self) -> Result<()> {
        let author_id = self.interaction.author_id().ok()?;
        let options =
            Command::from_interaction(self.interaction.data.ok()?.command().ok()?.into())?;

        let time = self.ctx.user_time(author_id, options).await?;
        self.handle
            .reply(Reply::new().content(time::format(time, options.style)))
            .await?;

        self.ctx.insert_usage(UsageKind::Date).await?;
        Ok(())
    }
}
