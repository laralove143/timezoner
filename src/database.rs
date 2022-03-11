use anyhow::{anyhow, Result};
use chrono_tz::Tz;
use sqlx::{query, query_scalar, sqlite::SqliteConnectOptions, SqlitePool};
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

/// connect to the database
pub async fn new() -> Result<SqlitePool> {
    let db =
        SqlitePool::connect_with(SqliteConnectOptions::new().filename("timezoner.sqlite")).await?;

    Ok(db)
}

/// update a user's timezone info
#[allow(clippy::integer_arithmetic)]
pub async fn set_timezone(db: &SqlitePool, user_id: Id<UserMarker>, tz: Tz) -> Result<()> {
    let id: i64 = user_id.get().try_into()?;
    let tz_str = tz.to_string();

    query!("INSERT OR REPLACE INTO timezones VALUES (?, ?)", id, tz_str)
        .execute(db)
        .await?;

    Ok(())
}

/// retrieve a user's timezone info
#[allow(clippy::integer_arithmetic)]
pub async fn timezone(db: &SqlitePool, user_id: Id<UserMarker>) -> Result<Option<Tz>> {
    let id: i64 = user_id.get().try_into()?;

    let tz = match query_scalar!("SELECT timezone FROM timezones WHERE user_id = ?", id)
        .fetch_optional(db)
        .await?
    {
        Some(tz) => tz,
        None => return Ok(None),
    };

    Ok(Some(
        tz.parse()
            .map_err(|s| anyhow!("saved timezone is invalid: {s}"))?,
    ))
}

/// disable parsing for a guild
#[allow(clippy::integer_arithmetic)]
pub async fn disable_parsing(db: &SqlitePool, guild_id: Id<GuildMarker>) -> Result<()> {
    let id: i64 = guild_id.get().try_into()?;

    query!("INSERT OR IGNORE INTO parse_disabled VALUES (?)", id)
        .execute(db)
        .await?;

    Ok(())
}

/// enable parsing for a guild
#[allow(clippy::integer_arithmetic)]
pub async fn enable_parsing(db: &SqlitePool, guild_id: Id<GuildMarker>) -> Result<()> {
    let id: i64 = guild_id.get().try_into()?;

    query!("DELETE FROM parse_disabled WHERE guild_id = ?", id)
        .execute(db)
        .await?;

    Ok(())
}

/// retrieve whether parsing is disabled in a guild
#[allow(clippy::integer_arithmetic)]
pub async fn parsing_disabled(db: &SqlitePool, guild_id: Id<GuildMarker>) -> Result<bool> {
    let id: i64 = guild_id.get().try_into()?;

    Ok(
        query_scalar!("SELECT 1 FROM parse_disabled WHERE guild_id = ?", id)
            .fetch_optional(db)
            .await?
            .is_some(),
    )
}
