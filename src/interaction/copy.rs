use anyhow::Result;
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_mention::Mention;
use twilight_model::{
    application::interaction::application_command::CommandData,
    channel::message::MessageFlags,
    http::interaction::InteractionResponseData,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{database, parse, parse::Format, Context};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "copy",
    desc = "copy the time so you can paste it in dms and all"
)]
/// the time command's options
pub struct Copy {
    #[command(desc = "the time you want to share")]
    /// the time to share
    time: String,
}

/// run the command, returning the response data
pub async fn run(
    ctx: &Context,
    user_id: Id<UserMarker>,
    command_data: CommandData,
) -> Result<InteractionResponseData> {
    let reply = _run(ctx, user_id, Copy::from_interaction(command_data.into())?).await?;

    Ok(InteractionResponseDataBuilder::new()
        .content(reply)
        .flags(MessageFlags::EPHEMERAL)
        .build())
}

/// run the command, returning the formatted string or the error message
async fn _run(ctx: &Context, user_id: Id<UserMarker>, options: Copy) -> Result<String> {
    let tz = match database::timezone(&ctx.db, user_id).await? {
        Some(tz) => tz,
        None => {
            return Ok(
                "i don't know your timezone yet, tell me using `/timezone` please".to_owned(),
            )
        }
    };

    let captures = if let Some(captures) = ctx
        .regex_12_hour
        .captures_iter(&options.time)
        .next()
        .map_or_else(
            || {
                ctx.regex_24_hour
                    .captures_iter(&options.time)
                    .next()
                    .map(|captures| (captures, Format::Hour24))
            },
            |captures| Some((captures, Format::Hour12)),
        ) {
        captures
    } else {
        return Ok("i couldn't find a time there, sorry :(".to_owned());
    };

    Ok(format!(
        "`{}`",
        parse::time(&captures.0, captures.1, tz)?.mention()
    ))
}
