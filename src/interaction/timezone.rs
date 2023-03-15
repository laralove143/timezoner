use anyhow::Result;
use sparkle_convenience::{
    error::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle, TextInput, TextInputStyle},
    Component, Embed, ReactionType,
};
use twilight_util::builder::embed::{EmbedFieldBuilder, ImageSource};

use crate::{database::UsageKind, embed, interaction::InteractionContext, CustomError};

const COPY_BUTTON_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/copy_button.png?raw=true";
const COPY_TIMEZONE_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/copy_timezone.png?raw=true";
const PASTE_BUTTON_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/paste_button.png?raw=true";
const SUBMIT_TIMEZONE_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/submit_timezone.png?raw=true";
const TIMEZONE_GIF_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/timezone.gif?raw=true";

const TIMEZONE_PICKER_URL: &str = "https://kevinnovak.github.io/Time-Zone-Picker/";

const COPY_BUTTON_LABEL: &str = "copy your timezone";
const PASTE_BUTTON_LABEL: &str = "paste it";

pub const PASTE_BUTTON_CUSTOM_ID: &str = "timezone_paste_button";
pub const MODAL_SUBMIT_ID: &str = "timezone_modal_submit";

#[derive(CreateCommand)]
#[command(
    name = "timezone",
    desc = "set your timezone so you can start sharing magical times"
)]
pub struct Command {}

fn copy_button() -> Component {
    Component::Button(Button {
        style: ButtonStyle::Link,
        emoji: Some(ReactionType::Unicode {
            name: "📋".to_owned(),
        }),
        label: Some(COPY_BUTTON_LABEL.to_owned()),
        url: Some(TIMEZONE_PICKER_URL.to_owned()),
        disabled: false,
        custom_id: None,
    })
}

fn paste_button() -> Component {
    Component::Button(Button {
        custom_id: Some(PASTE_BUTTON_CUSTOM_ID.to_owned()),
        style: ButtonStyle::Primary,
        emoji: Some(ReactionType::Unicode {
            name: "✍️".to_owned(),
        }),
        label: Some(PASTE_BUTTON_LABEL.to_owned()),
        disabled: false,
        url: None,
    })
}

fn copy_button_example_embed() -> Result<Embed> {
    Ok(embed()
        .title(":one:")
        .description(format!("press the *{COPY_BUTTON_LABEL}* button"))
        .image(ImageSource::url(COPY_BUTTON_EXAMPLE_URL)?)
        .build())
}

fn copy_timezone_example_embed() -> Result<Embed> {
    Ok(embed()
        .title(":two:")
        .description("in the website that opens, press the *copy* button")
        .field(EmbedFieldBuilder::new(
            "if the detected timezone is wrong",
            "select where you live on the map and then press the *copy* button",
        ))
        .image(ImageSource::url(COPY_TIMEZONE_EXAMPLE_URL)?)
        .build())
}

fn paste_button_example_embed() -> Result<Embed> {
    Ok(embed()
        .title(":three:")
        .description(format!(
            "come back to discord and press the *{PASTE_BUTTON_LABEL}* button"
        ))
        .image(ImageSource::url(PASTE_BUTTON_EXAMPLE_URL)?)
        .build())
}

fn submit_timezone_example_embed() -> Result<Embed> {
    Ok(embed()
        .title(":four:")
        .description(
            "paste the timezone you copied to the text field and press the *submit* button",
        )
        .image(ImageSource::url(SUBMIT_TIMEZONE_EXAMPLE_URL)?)
        .build())
}

fn timezone_example_gif_embed() -> Result<Embed> {
    Ok(embed()
        .title(":frame_photo:")
        .description("here's a gif if you prefer that over steps :)")
        .image(ImageSource::url(TIMEZONE_GIF_EXAMPLE_URL)?)
        .build())
}

impl InteractionContext<'_> {
    pub async fn handle_timezone_command(self) -> Result<()> {
        self.handle
            .reply(
                Reply::new()
                    .ephemeral()
                    .embed(copy_button_example_embed()?)
                    .embed(copy_timezone_example_embed()?)
                    .embed(paste_button_example_embed()?)
                    .embed(submit_timezone_example_embed()?)
                    .embed(timezone_example_gif_embed()?)
                    .component(Component::ActionRow(ActionRow {
                        components: vec![copy_button(), paste_button()],
                    })),
            )
            .await?;

        self.ctx.insert_usage(UsageKind::TimezoneCalled).await?;
        Ok(())
    }

    pub async fn handle_timezone_paste_button_click(self) -> Result<()> {
        self.handle
            .modal(
                MODAL_SUBMIT_ID.to_owned(),
                "timezone postal service".to_owned(),
                vec![TextInput {
                    custom_id: "timezone".to_owned(),
                    style: TextInputStyle::Short,
                    label: "paste your timezone here please".to_owned(),
                    placeholder: Some("America/Chicago".to_owned()),
                    required: Some(true),
                    max_length: None,
                    min_length: None,
                    value: None,
                }],
            )
            .await?;

        Ok(())
    }

    pub async fn handle_timezone_modal_submit(self) -> Result<(), anyhow::Error> {
        let user_id = self.interaction.author_id().ok()?;
        let input = self
            .interaction
            .data
            .ok()?
            .modal()
            .ok()?
            .components
            .into_iter()
            .next()
            .ok()?
            .components
            .into_iter()
            .next()
            .ok()?
            .value
            .ok()?;

        let tz = input.parse().map_err(|_| CustomError::BadTimezone)?;

        self.ctx.insert_timezone(user_id, tz).await?;

        self.handle
            .reply(
                Reply::new().ephemeral().embed(
                    embed()
                        .title(":partying_face: welcome onboard")
                        .description("now you can use me to show magical times")
                        .build(),
                ),
            )
            .await?;

        self.ctx.insert_usage(UsageKind::TimezoneSet).await?;
        Ok(())
    }
}
