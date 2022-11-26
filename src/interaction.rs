use std::{env, str::FromStr};

use anyhow::anyhow;
use sparkle_convenience::{
    reply::Reply,
    util::{InteractionDataExt, InteractionExt, Prettify},
    Error, IntoError,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::application::interaction::Interaction;

use crate::{
    interaction::{
        date::DateCommandOptions,
        timezone::{
            TimezoneCommand, TimezoneCommandOptions, TimezoneSubmit, TimezoneSubmitButtonClick,
        },
    },
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

impl From<UserError> for Reply {
    fn from(err: UserError) -> Self {
        Self::new().ephemeral().content(err.to_string())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandKind {
    Timezone,
    TimezoneSubmitButtonClick,
    TimezoneSubmit,
}

impl CommandKind {
    const fn deferred(self) -> bool {
        match self {
            Self::TimezoneSubmit | Self::Timezone => true,
            Self::TimezoneSubmitButtonClick => false,
        }
    }

    const fn is_ephemeral(self) -> bool {
        match self {
            Self::TimezoneSubmit | Self::Timezone => true,
            Self::TimezoneSubmitButtonClick => false,
        }
    }
}

impl FromStr for CommandKind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "timezone" => Ok(Self::Timezone),
            "timezone_submit_button_click" => Ok(Self::TimezoneSubmitButtonClick),
            "timezone_submit" => Ok(Self::TimezoneSubmit),
            _ => Err(anyhow!("Unknown command: {s}")),
        }
    }
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
        let command_kind: CommandKind = interaction.name().ok()?.parse()?;
        let handle = self.bot.interaction_handle(&interaction);
        let err_handle = handle.clone();

        if command_kind.deferred() {
            handle.defer(command_kind.is_ephemeral()).await?;
        }

        if let Err(err) = match command_kind {
            CommandKind::Timezone => TimezoneCommand::new(handle).run().await,
            CommandKind::TimezoneSubmitButtonClick => {
                TimezoneSubmitButtonClick::new(handle).run().await
            }
            CommandKind::TimezoneSubmit => {
                TimezoneSubmit::new(
                    handle,
                    self,
                    interaction.author_id().ok()?,
                    interaction.data.ok()?.modal().ok()?,
                )?
                .run()
                .await
            }
        } {
            if command_kind.deferred() {
                let reply = match &err {
                    Error::User(err) => (*err).into(),
                    Error::MissingPermissions(permissions) => {
                        Reply::new().ephemeral().content(format!(
                            "Please give me these permissions first:\n{}",
                            permissions.prettify()
                        ))
                    }
                    Error::Internal(_) => Reply::new().ephemeral().content(
                        "Something went wrong... The error has been reported to the developer"
                            .to_owned(),
                    ),
                };
                err_handle.reply(reply).await?;
            }

            if let Error::Internal(err) = err {
                return Err(err);
            }
        }

        Ok(())
    }
}
