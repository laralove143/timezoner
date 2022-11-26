use sparkle_convenience::{interaction::InteractionHandle, reply::Reply, Error, IntoError};
use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{
    application::interaction::modal::ModalInteractionData,
    channel::message::{
        component::{ActionRow, Button, ButtonStyle, TextInput, TextInputStyle},
        embed::{EmbedField, EmbedImage},
        Component, Embed, ReactionType,
    },
    id::{marker::UserMarker, Id},
};

use crate::{interaction::UserError, Context};

#[derive(CommandModel, CreateCommand)]
#[command(
    name = "timezone",
    desc = "Set your timezone so you can start sharing magical times"
)]
pub struct TimezoneCommandOptions {}

pub struct TimezoneCommand<'bot> {
    handle: InteractionHandle<'bot>,
}

impl<'bot> TimezoneCommand<'bot> {
    pub const fn new(handle: InteractionHandle<'bot>) -> Self {
        Self { handle }
    }

    pub async fn run(self) -> Result<(), Error<UserError>> {
        self.handle
            .reply(
                Reply::new()
                    .component(Component::ActionRow(ActionRow {
                        components: vec![
                            Component::Button(Button {
                                emoji: Some(ReactionType::Unicode {
                                    name: "📋".to_owned(),
                                }),
                                label: Some("Copy your timezone".to_owned()),
                                url: Some(
                                    "https://kevinnovak.github.io/Time-Zone-Picker/".to_owned(),
                                ),
                                style: ButtonStyle::Link,
                                disabled: false,
                                custom_id: None,
                            }),
                            Component::Button(Button {
                                custom_id: Some("timezone_submit_button_click".to_owned()),
                                emoji: Some(ReactionType::Unicode {
                                    name: "✍️".to_owned(),
                                }),
                                label: Some("Paste it here".to_owned()),
                                style: ButtonStyle::Primary,
                                disabled: false,
                                url: None,
                            }),
                        ],
                    }))
                    .embed(Embed {
                        title: Some("Timezone Picking 101".to_owned()),
                        fields: vec![
                            EmbedField {
                                name: ":one:".to_owned(),
                                value: "Press the `Copy your timezone` button".to_owned(),
                                inline: false,
                            },
                            EmbedField {
                                name: ":two:".to_owned(),
                                value: "In the website that opens, press the `Copy` button (If \
                                        your timezone isn't detected, select it on the map and \
                                        then press the `Copy` button)"
                                    .to_owned(),
                                inline: false,
                            },
                            EmbedField {
                                name: ":three:".to_owned(),
                                value: "Come back to Discord and press the `Paste it here` button"
                                    .to_owned(),
                                inline: false,
                            },
                            EmbedField {
                                name: ":four:".to_owned(),
                                value: "Paste the timezone you copied to the text field and press \
                                        the `Submit` button"
                                    .to_owned(),
                                inline: false,
                            },
                        ],
                        image: Some(EmbedImage {
                            url: "https://github.com/laralove143/timezoner/blob/main/examples\
                            /timezone.gif".to_owned(),
                            proxy_url: None,
                            height: None,
                            width: None,
                        }),
                        kind: String::new(),
                        description: None,
                        author: None,
                        footer: None,
                        timestamp: None,
                        thumbnail: None,
                        url: None,
                        color: None,
                        video: None,
                        provider: None,
                    }),
            )
            .await?;

        Ok(())
    }
}

pub struct TimezoneSubmitButtonClick<'bot> {
    pub handle: InteractionHandle<'bot>,
}

impl<'bot> TimezoneSubmitButtonClick<'bot> {
    pub const fn new(handle: InteractionHandle<'bot>) -> Self {
        Self { handle }
    }

    pub async fn run(self) -> Result<(), Error<UserError>> {
        self.handle
            .modal(
                "timezone_submit".to_owned(),
                "Timezone Postal Service".to_owned(),
                vec![TextInput {
                    custom_id: "timezone".to_owned(),
                    label: "Paste your timezone here please".to_owned(),
                    placeholder: Some("America/Chicago".to_owned()),
                    required: Some(true),
                    style: TextInputStyle::Short,
                    max_length: None,
                    min_length: None,
                    value: None,
                }],
            )
            .await?;

        Ok(())
    }
}

pub struct TimezoneSubmit<'bot> {
    pub handle: InteractionHandle<'bot>,
    pub ctx: &'bot Context,
    pub user_id: Id<UserMarker>,
    pub timezone: String,
}

impl<'bot> TimezoneSubmit<'bot> {
    pub fn new(
        handle: InteractionHandle<'bot>,
        ctx: &'bot Context,
        user_id: Id<UserMarker>,
        data: ModalInteractionData,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            handle,
            ctx,
            user_id,
            timezone: data
                .components
                .into_iter()
                .next()
                .ok()?
                .components
                .into_iter()
                .next()
                .ok()?
                .value
                .ok()?,
        })
    }

    pub async fn run(self) -> Result<(), Error<UserError>> {
        let tz = time_tz::timezones::get_by_name(&self.timezone)
            .ok_or(Error::User(UserError::BadTimezone))?;

        self.ctx.insert_timezone(self.user_id, tz).await?;

        self.handle
            .reply(
                Reply::new()
                    .ephemeral()
                    .content("Done! Now you can use me to show magical times".to_owned()),
            )
            .await?;

        Ok(())
    }
}
