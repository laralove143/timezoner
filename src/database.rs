#![allow(clippy::use_self)]

use anyhow::Result;
use chrono_tz::Tz;
use sparkle_convenience::error::IntoError;
use sqlx::{query, query_scalar, Postgres};
use twilight_model::id::{marker::UserMarker, Id};

use crate::{Context, Error};

trait Encode<'a, T: sqlx::Encode<'a, Postgres>> {
    fn encode(&self) -> T;
}

impl<T> Encode<'_, i64> for Id<T> {
    #[allow(clippy::cast_possible_wrap)]
    fn encode(&self) -> i64 {
        self.get() as i64
    }
}

impl Encode<'_, &str> for Tz {
    fn encode(&self) -> &'static str {
        self.name()
    }
}

trait Decode<T> {
    fn decode(&self) -> T;
}

impl Decode<Result<Tz>> for String {
    #[allow(clippy::use_self)]
    fn decode(&self) -> Result<Tz> {
        Ok(self.parse().map_err(Error::TimezoneParseDatabase)?)
    }
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "usage_kind")]
pub enum UsageKind {
    TimeDetect,
    TimeConvertByAuthor,
    TimeConvertByNonAuthor,
    Help,
    TimezoneCalledDetected,
    TimezoneCalledUndetected,
    TimezoneSetDetected,
    TimezoneSetUndetected,
    Date,
    Copy,
    CurrentTime,
}

impl Context {
    pub async fn insert_timezone(&self, user_id: Id<UserMarker>, timezone: Tz) -> Result<()> {
        query!(
            "INSERT INTO timezones (user_id, timezone) VALUES ($1, $2) ON CONFLICT (user_id) DO \
             UPDATE SET timezone = $2",
            user_id.encode(),
            timezone.encode()
        )
        .execute(&self.db)
        .await?;

        Ok(())
    }

    pub async fn timezone(&self, user_id: Id<UserMarker>) -> Result<Option<Tz>> {
        match query_scalar!(
            "SELECT timezone FROM timezones WHERE user_id = $1",
            user_id.encode()
        )
        .fetch_optional(&self.db)
        .await?
        {
            Some(timezone) => Ok(Some(timezone.decode()?)),
            None => Ok(None),
        }
    }

    pub async fn insert_guild_count(&self, count: i32) -> Result<()> {
        query!("INSERT INTO guild_count (count) VALUES ($1)", count)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn insert_usage(&self, kind: UsageKind) -> Result<()> {
        query!("INSERT INTO usage (kind) VALUES ($1)", kind as _)
            .execute(&self.db)
            .await?;

        Ok(())
    }

    pub async fn usage_count(&self) -> Result<i64> {
        query_scalar!(
            "
            SELECT count(*)
            FROM usage
            WHERE kind IN
                ('TimeConvertByAuthor', 'TimeConvertByNonAuthor', 'Date', 'Copy', 'CurrentTime')"
        )
        .fetch_one(&self.db)
        .await?
        .ok()
    }
}
