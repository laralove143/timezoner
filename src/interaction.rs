use anyhow::{anyhow, Result};
use sparkle_convenience::{
    error::conversion::IntoError,
    interaction::{extract::InteractionExt, InteractionHandle},
    Bot,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    id::{marker::CommandMarker, Id},
};

use crate::{log, Context, HandleExitResult, TEST_GUILD_ID};

mod copy;
mod date;
mod help;
mod timezone;

#[derive(Clone, Copy, Debug)]
pub struct CommandIds {
    pub timezone: Id<CommandMarker>,
    pub date: Id<CommandMarker>,
    pub copy: Id<CommandMarker>,
}

impl CommandIds {
    fn new(commands: &[Command]) -> Result<Self> {
        Ok(Self {
            timezone: Self::command_id("timezone", commands)?,
            date: Self::command_id("date", commands)?,
            copy: Self::command_id("copy", commands)?,
        })
    }

    fn command_id(command_name: &str, commands: &[Command]) -> Result<Id<CommandMarker>> {
        commands
            .iter()
            .find_map(|command| (command.name == command_name).then_some(command.id?))
            .ok()
    }
}

#[derive(Debug)]
struct InteractionContext<'ctx> {
    ctx: &'ctx Context,
    handle: InteractionHandle<'ctx>,
    interaction: Interaction,
}

pub async fn set_commands(bot: &Bot) -> Result<CommandIds> {
    let commands = &[
        timezone::CommandOptions::create_command().into(),
        date::CommandOptions::create_command().into(),
        copy::CommandOptions::create_command().into(),
        help::CommandOptions::create_command().into(),
    ];

    let commands_response = bot
        .interaction_client()
        .set_global_commands(commands)
        .await?
        .models()
        .await?;
    bot.interaction_client()
        .set_guild_commands(TEST_GUILD_ID, commands)
        .await?;

    CommandIds::new(&commands_response)
}

impl Context {
    pub async fn handle_interaction(&self, interaction: Interaction) -> Result<()> {
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
            "date" => ctx.handle_date_command().await,
            "copy" => ctx.handle_copy_command().await,
            "help" => ctx.handle_help_command().await,
            name => Err(anyhow!("unknown command: {name}")),
        };

        if let Some((reply, internal_err)) = command_run_result.handle() {
            if let Some((_, Some(err))) = handle.reply(reply).await.handle() {
                log(&self.bot, &err).await;
            }

            if let Some(err) = internal_err {
                log(&self.bot, &err).await;
            }
        }

        Ok(())
    }
}
