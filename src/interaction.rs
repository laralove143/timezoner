use std::env;

use anyhow::anyhow;
use sparkle_convenience::{
    interaction::InteractionHandle, reply::Reply, util::Prettify, Error, IntoError,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::application::interaction::Interaction;

use crate::{
    interaction::{date::DateCommandOptions, timezone::TimezoneCommandOptions},
    Context,
};

mod date;
mod timezone;

#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UserError {
    #[error(
        "I couldn't find that timezone... If you're sure the timezone is right, please join the \
         support server"
    )]
    BadTimezone,
}

struct InteractionContext<'ctx, 'handle> {
    ctx: &'ctx Context,
    handle: &'ctx mut InteractionHandle<'handle>,
    interaction: Interaction,
}

impl Context {
    pub async fn set_commands(&self) -> Result<(), anyhow::Error> {
        let commands = &[
            DateCommandOptions::create_command().into(),
            TimezoneCommandOptions::create_command().into(),
        ];

        self.bot
            .interaction_client()
            .set_global_commands(commands)
            .await?;
        self.bot
            .interaction_client()
            .set_guild_commands(env::var("TEST_GUILD_ID")?.parse()?, commands)
            .await?;

        Ok(())
    }

    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<(), anyhow::Error> {
        let mut handle = self.bot.interaction_handle(&interaction);
        let ctx = InteractionContext {
            ctx: self,
            handle: &mut handle,
            interaction,
        };

        let command_run_result = match ctx.handle.name.as_deref().ok()? {
            "timezone" => ctx.handle_timezone_command().await,
            "timezone_paste_button" => ctx.handle_timezone_paste_button_click().await,
            "timezone_modal_submit" => ctx.handle_timezone_modal_submit().await,
            name => Err(Error::Internal(anyhow!("Unknown command: {name}"))),
        };

        if let Err(err) = command_run_result {
            let content = match &err {
                Error::User(err) => err.to_string(),
                Error::MissingPermissions(permissions) => {
                    format!(
                        "Please give me these permissions first:\n{}",
                        permissions.prettify()
                    )
                }
                Error::Internal(_) => "Something went wrong... The error has been reported to the \
                                       developer"
                    .to_owned(),
            };
            handle
                .reply(Reply::new().ephemeral().content(content))
                .await?;

            if let Error::Internal(err) = err {
                return Err(err);
            }
        }

        Ok(())
    }
}
