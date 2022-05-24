use aes_gcm_siv::{aead::Aead, Nonce};
use anyhow::{anyhow, Result};
use chrono_tz::Tz;
use rand::random;
use sqlx::{query, query_scalar, sqlite::SqliteConnectOptions, SqlitePool};
use twilight_model::id::{marker::UserMarker, Id};

use crate::Context;

/// connect to the database
pub async fn new() -> Result<SqlitePool> {
    let db =
        SqlitePool::connect_with(SqliteConnectOptions::new().filename("timezoner_origin.sqlite"))
            .await?;

    Ok(db)
}

/// update a user's timezone info
#[allow(clippy::integer_arithmetic)]
pub async fn set_timezone(ctx: &Context, user_id: Id<UserMarker>, tz: Tz) -> Result<()> {
    let id: i64 = user_id.get().try_into()?;

    let arr = random::<[u8; 12]>();
    let slice = arr.as_slice();
    let nonce = Nonce::from_slice(slice);

    let encrypted = ctx.cipher.encrypt(nonce, tz.to_string().as_bytes())?;

    query!(
        "INSERT OR REPLACE INTO timezones VALUES (?, ?, ?)",
        id,
        encrypted,
        slice
    )
    .execute(&ctx.db)
    .await?;

    Ok(())
}

/// retrieve a user's timezone info
#[allow(clippy::integer_arithmetic)]
pub async fn timezone(ctx: &Context, user_id: Id<UserMarker>) -> Result<Option<Tz>> {
    let id: i64 = user_id.get().try_into()?;

    let tz = match query!(
        "SELECT timezone, nonce FROM timezones WHERE user_id = ?",
        id
    )
    .fetch_optional(&ctx.db)
    .await?
    {
        Some(record) => String::from_utf8(
            ctx.cipher
                .decrypt(Nonce::from_slice(&record.nonce), record.timezone.as_slice())?,
        )?,
        None => return Ok(None),
    };

    Ok(Some(
        tz.parse()
            .map_err(|s| anyhow!("saved timezone is invalid: {s}"))?,
    ))
}

/// set a user as dmed
#[allow(clippy::integer_arithmetic)]
pub async fn set_dmed(db: &SqlitePool, user_id: Id<UserMarker>) -> Result<()> {
    let id: i64 = user_id.get().try_into()?;

    query!("INSERT OR REPLACE INTO dmed VALUES (?)", id)
        .execute(db)
        .await?;

    Ok(())
}

/// get whether a user is already dmed
#[allow(clippy::integer_arithmetic)]
pub async fn dmed(db: &SqlitePool, user_id: Id<UserMarker>) -> Result<bool> {
    let id: i64 = user_id.get().try_into()?;

    Ok(
        query_scalar!("SELECT user_id FROM dmed WHERE user_id = ?", id)
            .fetch_optional(db)
            .await?
            .is_some(),
    )
}
