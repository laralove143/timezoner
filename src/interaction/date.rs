use twilight_interactions::command::CreateCommand;

#[derive(CreateCommand)]
#[command(
    name = "date",
    desc = "send a date that everyone sees in their own timezone"
)]
#[allow(clippy::module_name_repetitions)]
pub struct DateCommand {
    #[command(desc = "the day of the date", min_value = 0, max_value = 31)]
    day: i64,
    #[command(desc = "the month of the date", min_value = 0, max_value = 12)]
    month: i64,
    #[command(desc = "the year of the date", min_value = -9999, max_value = 9999)]
    year: i64,
    #[command(
        desc = "the hour of the date in 24-hour format",
        min_value = 0,
        max_value = 23
    )]
    hour: i64,
    #[command(desc = "the minute of the date", min_value = 0, max_value = 59)]
    min: i64,
}
