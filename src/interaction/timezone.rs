use anyhow::Result;
use sparkle_convenience::{
    error::conversion::IntoError, interaction::extract::InteractionDataExt, reply::Reply,
};
use twilight_interactions::command::CreateCommand;
use twilight_model::channel::message::{
    component::{ActionRow, Button, ButtonStyle, TextInput, TextInputStyle},
    embed::EmbedImage,
    Component, Embed, ReactionType,
};

use crate::{interaction::InteractionContext, CustomError, ACCENT_COLOR};

const COPY_BUTTON_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/copy_button.png?raw=true";
const COPY_TIMEZONE_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/copy_timezone.png?raw=true";
const PASTE_BUTTON_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/paste_button.png?raw=true";
const SUBMIT_TIMEZONE_EXAMPLE_URL: &str =
    "https://github.com/laralove143/timezoner/blob/main/examples/submit_timezone.png?raw=true";

#[derive(CreateCommand)]
#[command(
    name = "timezone",
    desc = "set your timezone so you can start sharing magical times"
)]
pub struct CommandOptions {}

fn copy_button() -> Component {
    Component::Button(Button {
        style: ButtonStyle::Link,
        emoji: Some(ReactionType::Unicode {
            name: "📋".to_owned(),
        }),
        label: Some("copy your timezone".to_owned()),
        url: Some("https://kevinnovak.github.io/Time-Zone-Picker/".to_owned()),
        disabled: false,
        custom_id: None,
    })
}

fn paste_button() -> Component {
    Component::Button(Button {
        custom_id: Some("timezone_paste_button".to_owned()),
        style: ButtonStyle::Primary,
        emoji: Some(ReactionType::Unicode {
            name: "✍️".to_owned(),
        }),
        label: Some("paste it".to_owned()),
        disabled: false,
        url: None,
    })
}

fn copy_button_example_embed() -> Embed {
    Embed {
        title: Some("press the `copy your timezone` button".to_owned()),
        color: Some(ACCENT_COLOR),
        image: Some(EmbedImage {
            url: COPY_BUTTON_EXAMPLE_URL.to_owned(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        fields: vec![],
        kind: String::new(),
        author: None,
        description: None,
        footer: None,
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        video: None,
    }
}

fn copy_timezone_example_embed() -> Embed {
    Embed {
        title: Some("in the website that opens, press the `copy` button".to_owned()),
        description: Some(
            "if the detected timezone is wrong, select where you live on the map and then press \
             the `copy` button"
                .to_owned(),
        ),
        color: Some(ACCENT_COLOR),
        image: Some(EmbedImage {
            url: COPY_TIMEZONE_EXAMPLE_URL.to_owned(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        fields: vec![],
        kind: String::new(),
        author: None,
        footer: None,
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        video: None,
    }
}

fn paste_button_example_embed() -> Embed {
    Embed {
        title: Some(
            "come back to discord and press the `paste it` button".to_owned(),
        ),
        color: Some(ACCENT_COLOR),
        image: Some(EmbedImage {
            url: PASTE_BUTTON_EXAMPLE_URL.to_owned(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        fields: vec![],
        kind: String::new(),
        author: None,
        description: None,
        footer: None,
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        video: None,
    }
}

fn submit_timezone_example_embed() -> Embed {
    Embed {
        title: Some(
            "paste the timezone you copied to the text field and press the `submit` button"
                .to_owned(),
        ),
        color: Some(ACCENT_COLOR),
        image: Some(EmbedImage {
            url: SUBMIT_TIMEZONE_EXAMPLE_URL.to_owned(),
            proxy_url: None,
            height: None,
            width: None,
        }),
        fields: vec![],
        kind: String::new(),
        author: None,
        description: None,
        footer: None,
        provider: None,
        thumbnail: None,
        timestamp: None,
        url: None,
        video: None,
    }
}

impl InteractionContext<'_> {
    pub async fn handle_timezone_command(self) -> Result<()> {
        self.handle
            .reply(
                Reply::new()
                    .ephemeral()
                    .embed(copy_button_example_embed())
                    .embed(copy_timezone_example_embed())
                    .embed(paste_button_example_embed())
                    .embed(submit_timezone_example_embed())
                    .component(Component::ActionRow(ActionRow {
                        components: vec![copy_button(), paste_button()],
                    })),
            )
            .await?;

        Ok(())
    }

    pub async fn handle_timezone_paste_button_click(self) -> Result<()> {
        self.handle
            .modal(
                "timezone_modal_submit".to_owned(),
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
                Reply::new()
                    .ephemeral()
                    .content("done! now you can use me to show magical times".to_owned()),
            )
            .await?;

        Ok(())
    }
}
