use anyhow::{anyhow, Result};
use chrono_tz::Tz;
use sqlx::{query, query_scalar, sqlite::SqliteConnectOptions, SqlitePool};
use twilight_model::id::{marker::UserMarker, Id};

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
