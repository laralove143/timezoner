use twilight_interactions::command::CreateCommand;

use crate::{interaction::date::DateCommand, Context};

mod date;

impl Context {
    pub async fn set_commands(&self) -> Result<(), anyhow::Error> {
        self.bot
            .interaction_client()
            .set_global_commands(&[DateCommand::create_command().into()])
            .await?;

        Ok(())
    }
}
