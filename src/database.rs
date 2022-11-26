#![allow(clippy::cast_possible_wrap)]

use sqlx::query_file;
use time_tz::{TimeZone, Tz};
use twilight_model::id::{marker::UserMarker, Id};

use crate::Context;

impl Context {
    pub async fn init_db(&self) -> Result<(), anyhow::Error> {
        query_file!("sql/init.sql").execute(self.cache.pg()).await?;

        Ok(())
    }

    pub async fn insert_timezone(
        &self,
        user_id: Id<UserMarker>,
        timezone: &Tz,
    ) -> Result<(), anyhow::Error> {
        query_file!(
            "sql/insert_timezone.sql",
            user_id.get() as i64,
            timezone.name()
        )
        .execute(self.cache.pg())
        .await?;

        Ok(())
    }
}
