use sparkle_convenience::interaction::InteractionHandle;
use twilight_interactions::command::CreateCommand;

#[derive(CreateCommand)]
#[command(
    name = "date",
    desc = "Send a date that everyone sees in their own timezone"
)]
pub struct DateCommandOptions {
    #[command(desc = "The day of the date", min_value = 0, max_value = 31)]
    day: i64,
    #[command(desc = "The month of the date", min_value = 0, max_value = 12)]
    month: i64,
    #[command(desc = "the year of the date", min_value = -9999, max_value = 9999)]
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

pub struct DateCommand<'bot> {
    handle: InteractionHandle<'bot>,
    options: DateCommandOptions,
}
