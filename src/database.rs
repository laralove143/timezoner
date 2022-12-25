use anyhow::{anyhow, Result};
use chrono_tz::{ParseError, Tz};
use sqlx::{query, query_scalar, Postgres};
use twilight_model::id::{marker::UserMarker, Id};

use crate::Context;

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
        self.parse().map_err(|err: ParseError| anyhow!(err))
    }
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
}
