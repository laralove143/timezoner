use std::str::FromStr;

use anyhow::{bail, Context as _, Result};
use chrono_tz::Tz;
use sqlx::SqlitePool;
use tantivy::{
    collector::TopDocs,
    query::QueryParser,
    schema::{Field, Value},
};
use twilight_model::{
    application::{
        command::{Command, CommandOptionChoice, CommandType},
        interaction::{
            application_command::{CommandData, CommandDataOption, CommandOptionValue},
            application_command_autocomplete::ApplicationCommandAutocompleteDataOption,
        },
    },
    channel::message::MessageFlags,
    http::interaction::InteractionResponseData,
    id::{marker::UserMarker, Id},
};
use twilight_util::builder::{
    command::{CommandBuilder, StringBuilder},
    InteractionResponseDataBuilder,
};

use crate::{database, Context};

/// the timezone option of the `timezone` command
#[derive(Debug)]
enum TimezoneOption {
    /// the interaction is sent completely
    Complete(String),
    /// the value is to be autocompleted
    Partial(String),
}

/// the `timezone` command's options
#[derive(Debug)]
pub struct Timezone {
    /// the timezone option
    timezone: TimezoneOption,
}

impl Timezone {
    /// return the command to register
    pub fn build() -> Command {
        CommandBuilder::new(
            "timezone".to_owned(),
            "set your time zone so that you can actually use me".to_owned(),
            CommandType::ChatInput,
        )
        .option(
            StringBuilder::new(
                "timezone".to_owned(),
                "try typing your city, country or its capital".to_owned(),
            )
            .required(true)
            .autocomplete(true),
        )
        .build()
    }
}

impl TryFrom<Vec<CommandDataOption>> for Timezone {
    type Error = anyhow::Error;

    fn try_from(mut options: Vec<CommandDataOption>) -> Result<Self> {
        Ok(Self {
            timezone: TimezoneOption::Complete(
                if let CommandOptionValue::String(tz) = options
                    .pop()
                    .context("timezone command has no options")?
                    .value
                {
                    tz
                } else {
                    bail!("timezone command's first option is not string: {options:#?}")
                },
            ),
        })
    }
}

impl From<Vec<ApplicationCommandAutocompleteDataOption>> for Timezone {
    fn from(mut options: Vec<ApplicationCommandAutocompleteDataOption>) -> Self {
        Self {
            timezone: TimezoneOption::Partial(
                options
                    .pop()
                    .map_or("".to_owned(), |o| o.value.unwrap_or_default()),
            ),
        }
    }
}

/// run the command, returning the response data
pub async fn run(
    db: &SqlitePool,
    user_id: Id<UserMarker>,
    command_data: CommandData,
) -> Result<InteractionResponseData> {
    let reply = _run(db, user_id, command_data.options.try_into()?).await?;

    Ok(InteractionResponseDataBuilder::new()
        .flags(MessageFlags::EPHEMERAL)
        .content(reply.to_owned())
        .build())
}

/// run the command, returning the success or error message
async fn _run(db: &SqlitePool, user_id: Id<UserMarker>, options: Timezone) -> Result<&'static str> {
    let tz_option = if let TimezoneOption::Complete(tz) = options.timezone {
        tz
    } else {
        bail!("tried to run timezone command with partial option: {options:#?}")
    };

    let tz = if let Ok(tz) = Tz::from_str(&tz_option) {
        tz
    } else {
        return Ok(
            "i couldn't find that timezone :( you can use this website to copy-paste instead:\n\
            https://kevinnovak.github.io/Time-Zone-Picker",
        );
    };

    database::set_timezone(db, user_id, tz).await?;

    Ok("tada! now you can send times ^^")
}

/// return the interaction response data with the suggestions based on the
/// partial input
pub fn run_autocomplete(ctx: &Context, options: Timezone) -> Result<InteractionResponseData> {
    let suggestions = _run_autocomplete(ctx, options)?;

    Ok(InteractionResponseDataBuilder::new()
        .choices(
            suggestions
                .into_iter()
                .map(|s| CommandOptionChoice::String {
                    name: s.clone(),
                    value: s,
                }),
        )
        .build())
}

/// return suggestions based on the partial input
fn _run_autocomplete(ctx: &Context, options: Timezone) -> Result<Vec<String>> {
    let mut suggestions = Vec::with_capacity(10);

    let partial = if let TimezoneOption::Partial(option) = options.timezone {
        option
    } else {
        bail!("tried to run timezone autocomplete with a complete option: {options:#?}")
    };

    if partial.len() < 3 {
        return Ok(suggestions);
    }

    let query = match QueryParser::for_index(&ctx.searcher.0, vec![Field::from_field_id(0)])
        .parse_query(&partial)
    {
        Ok(query) => query,
        Err(_) => return Ok(suggestions),
    };
    let searcher = ctx.searcher.1.searcher();
    let docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_, address) in docs {
        let doc = searcher.doc(address)?;
        for field in doc {
            if let Value::Str(val) = field.value {
                suggestions.push(val);
            };
        }
    }

    Ok(suggestions)
}
