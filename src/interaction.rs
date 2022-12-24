use std::env;

use anyhow::anyhow;
use sparkle_convenience::{
    error::{conversion::IntoError, ErrorExt},
    interaction::{extract::InteractionExt, InteractionHandle},
    Bot,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    id::{marker::CommandMarker, Id},
};

use crate::{
    err_reply,
    interaction::{date::DateCommandOptions, timezone::TimezoneCommandOptions},
    Context, CustomError,
};

mod date;
mod timezone;

struct InteractionContext<'ctx> {
    ctx: &'ctx Context,
    handle: InteractionHandle<'ctx>,
    interaction: Interaction,
}

pub async fn set_commands(bot: &Bot) -> Result<Vec<Command>, anyhow::Error> {
    let commands = &[
        DateCommandOptions::create_command().into(),
        TimezoneCommandOptions::create_command().into(),
    ];

    let commands_response = bot
        .interaction_client()
        .set_global_commands(commands)
        .await?
        .models()
        .await?;
    bot.interaction_client()
        .set_guild_commands(env::var("TEST_GUILD_ID")?.parse()?, commands)
        .await?;

    Ok(commands_response)
}

impl Context {
    pub fn timezone_command_id(&self) -> Result<Id<CommandMarker>, anyhow::Error> {
        self.commands
            .iter()
            .find_map(|command| (command.name == "timezone").then_some(command.id?))
            .ok()
    }

    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<(), anyhow::Error> {
        let handle = self.bot.interaction_handle(&interaction);
        let ctx = InteractionContext {
            ctx: self,
            handle: handle.clone(),
            interaction,
        };

        let command_run_result = match ctx.interaction.name().ok()? {
            "timezone" => ctx.handle_timezone_command().await,
            "timezone_paste_button" => ctx.handle_timezone_paste_button_click().await,
            "timezone_modal_submit" => ctx.handle_timezone_modal_submit().await,
            name => Err(anyhow!("unknown command: {name}")),
        };

        if let Err(err) = command_run_result {
            if err.ignore() {
                return Ok(());
            }

            handle.reply(err_reply(&err)?).await?;

            if let Some(err) = err.internal::<CustomError>() {
                return Err(err);
            }
        }

        Ok(())
    }
}
