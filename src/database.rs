use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use twilight_model::id::{marker::UserMarker, Id};

use crate::Context;

#[derive(Serialize, Deserialize)]
struct Timezone {
    id: Id<UserMarker>,
    timezone: String,
}

impl Context {
    pub fn insert_timezone(&self, key: Id<UserMarker>, timezone: Tz) -> Result<(), anyhow::Error> {
        self.timezones()?.insert(key.id(), timezone.name())?;

        Ok(())
    }

    pub fn timezone(&self, key: Id<UserMarker>) -> Result<Option<Tz>, anyhow::Error> {
        match self.timezones()?.get(key.id())? {
            Some(value) => Ok(Some(value.timezone()?)),
            None => Ok(None),
        }
    }
}
