mod time;
mod timezone;

use anyhow::{bail, Result};
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;
use twilight_model::{
    application::{callback::InteractionResponse, interaction::Interaction},
    id::{marker::ApplicationMarker, Id},
};
use twilight_util::builder::CallbackDataBuilder;

use crate::{
    commands::{time::Time, timezone::Timezone},
    Context,
};

pub async fn handle(ctx: Context, interaction: Interaction) -> Result<()> {
    let command = if let Interaction::ApplicationCommand(command) = interaction {
        command
    } else {
        bail!("unknown interaction: {:?}", interaction);
    };
    let client = ctx.http.interaction(ctx.application_id);
    let user_id = command.as_ref().member.as_ref().unwrap().user.as_ref().unwrap().id;

    let reply = match command.data.name.as_str() {
        "time" => time::run(&ctx.db, user_id, command.data.options).await?,
        "timezone" => timezone::run(&ctx.db, user_id, command.data.options).await?,
        _ => bail!("unknown command: {:?}", command),
    };

    let callback = CallbackDataBuilder::new().content(reply).build();

    client
        .interaction_callback(
            command.id,
            &command.token,
            &InteractionResponse::ChannelMessageWithSource(callback),
        )
        .exec()
        .await?;

    Ok(())
}

pub async fn create(http: &Client, application_id: Id<ApplicationMarker>) -> Result<()> {
    http.interaction(application_id).set_global_commands(&[time::build(), timezone::build()])
        .exec()
        .await?;

    Ok(())
}