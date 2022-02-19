use std::{num::NonZeroU64, str::FromStr};

use anyhow::{bail, Result};
use chrono_tz::Tz;
use sqlx::{query, query_scalar, sqlite::SqliteConnectOptions, SqlitePool};
use twilight_model::id::marker::UserMarker;
use twilight_model::id::Id;

trait AsI64 {
    fn as_i64(&self) -> i64;
}

impl AsI64 for NonZeroU64 {
    fn as_i64(&self) -> i64 {
        self.get() as i64
    }
}

pub async fn new() -> Result<SqlitePool> {
    let db =
        SqlitePool::connect_with(SqliteConnectOptions::new().filename("timezoner.sqlite")).await?;

    Ok(db)
}

pub async fn set_timezone(db: &SqlitePool, user_id: Id<UserMarker>, tz: &Tz) -> Result<()> {
    let user_id: i64 = user_id.get().try_into()?;
    let tz = tz.to_string();

    query!(
        "INSERT OR REPLACE INTO timezones VALUES (?, ?)",
        user_id,
        tz
    )
    .execute(db)
    .await?;

    Ok(())
}

pub async fn timezone(db: &SqlitePool, user_id: Id<UserMarker>) -> Result<Option<Tz>> {
    let user_id: i64 = user_id.get().try_into()?;

    let tz = query_scalar!("SELECT timezone FROM timezones WHERE user_id = ?", user_id)
        .fetch_optional(db)
        .await?;

    match tz {
        Some(tz) => match Tz::from_str(&tz) {
            Ok(tz) => Ok(Some(tz)),
            Err(string) => bail!("couldn't parse timezone from string: {}", string),
        },
        None => Ok(None),
    }
}
