use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::{
    command::CommandBuilder,
    embed::{EmbedAuthorBuilder, ImageSource},
};

use crate::{
    database::UsageKind,
    embed,
    interaction::{date, InteractionContext},
    message::avatar_url,
    CustomError,
};

pub const NAME: &str = "get current time for user";

pub fn command() -> Command {
    CommandBuilder::new(NAME, "", CommandType::User)
        .dm_permission(false)
        .build()
}

impl InteractionContext<'_> {
    pub async fn handle_current_time_command(self) -> Result<()> {
        let resolved = self.interaction.data.ok()?.command().ok()?.resolved.ok()?;
        let member = resolved.members.into_iter().next().ok()?.1;
        let user = resolved.users.into_iter().next().ok()?.1;

        self.handle
            .reply(
                Reply::new().ephemeral().embed(
                    embed()
                        .author(
                            EmbedAuthorBuilder::new(member.nick.unwrap_or(user.name)).icon_url(
                                ImageSource::url(avatar_url(
                                    member.avatar,
                                    user.avatar,
                                    user.id,
                                    self.interaction.guild_id,
                                    user.discriminator,
                                ))?,
                            ),
                        )
                        .description(
                            self.ctx
                                .user_time(user.id, date::Command::default())
                                .await
                                .map_err(|err| {
                                    if let Some(CustomError::MissingTimezone(command_id)) =
                                        err.downcast_ref()
                                    {
                                        CustomError::OtherUserMissingTimezone(*command_id).into()
                                    } else {
                                        err
                                    }
                                })?
                                .format("%A, %B %-d, %-Y %-I:%M %p")
                                .to_string(),
                        )
                        .build(),
                ),
            )
            .await?;

        self.ctx.insert_usage(UsageKind::CurrentTime).await?;
        Ok(())
    }
}
