use anyhow::Result;
use sparkle_convenience::{
    error::IntoError,
    interaction::{extract::InteractionExt, InteractionHandle},
    Bot,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{command::Command, interaction::Interaction},
    id::{marker::CommandMarker, Id},
};

use crate::{err_reply, Context, CustomError, Error, TEST_GUILD_ID};

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
            timezone: Self::command_id(timezone::CommandOptions::NAME, commands)?,
            date: Self::command_id(date::CommandOptions::NAME, commands)?,
            copy: Self::command_id(copy::CommandOptions::NAME, commands)?,
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

impl<'ctx> InteractionContext<'ctx> {
    async fn handle(self) -> Result<()> {
        match self.interaction.name().ok()? {
            timezone::CommandOptions::NAME => self.handle_timezone_command().await,
            timezone::PASTE_BUTTON_CUSTOM_ID => self.handle_timezone_paste_button_click().await,
            timezone::MODAL_SUBMIT_ID => self.handle_timezone_modal_submit().await,
            date::CommandOptions::NAME => self.handle_date_command().await,
            copy::CommandOptions::NAME => self.handle_copy_command().await,
            help::CommandOptions::NAME => self.handle_help_command().await,
            name => Err(Error::UnknownCommand(name.to_owned()).into()),
        }
    }
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
    pub async fn handle_interaction(&self, interaction: Interaction) {
        let handle = self.bot.interaction_handle(&interaction);
        let ctx = InteractionContext {
            ctx: self,
            handle: handle.clone(),
            interaction,
        };

        if let Err(err) = ctx.handle().await {
            handle
                .handle_error::<CustomError>(err_reply(&err), err)
                .await;
        }
    }
}
