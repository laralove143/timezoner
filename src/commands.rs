mod date;
mod set_timezone;
mod time;

use std::num::NonZeroU64;

use anyhow::{bail, Result};
use twilight_http::Client;
use twilight_model::{
    application::{
        callback::InteractionResponse,
        interaction::{ApplicationCommand, Interaction},
    },
    id::{GuildId, UserId},
};
use twilight_util::builder::CallbackDataBuilder;

use crate::Context;

pub async fn handle(ctx: Context, interaction: Interaction) -> Result<()> {
    let command = if let Interaction::ApplicationCommand(command) = interaction {
        command
    } else {
        bail!("unknown interaction: {:?}", interaction);
    };
    let user_id = get_user_id(&command).unwrap();

    let reply = match command.data.name.as_str() {
        "time" => time::run(&ctx.db, user_id, command.data.options).await?,
        "date" => date::run(&ctx.db, user_id, command.data.options).await?,
        "set_timezone" => set_timezone::run(&ctx.db, user_id, command.data.options).await?,
        _ => bail!("unknown command: {:?}", command),
    };

    let callback = CallbackDataBuilder::new().content(reply).build();

    ctx.http
        .interaction_callback(
            command.id,
            &command.token,
            &InteractionResponse::ChannelMessageWithSource(callback),
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn create(http: &Client) -> Result<()> {
    let guild_id = GuildId(NonZeroU64::new(903367565349384202).unwrap());

    http.set_guild_commands(
        guild_id,
        &[time::build(), date::build(), set_timezone::build()],
    )?
    .exec()
    .await?;

    Ok(())
}

fn get_user_id(command: &ApplicationCommand) -> Option<UserId> {
    if let Some(member) = &command.member {
        Some(member.user.as_ref()?.id)
    } else if let Some(user) = &command.user {
        Some(user.id)
    } else {
        None
    }
}
