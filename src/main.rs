#![warn(
    clippy::nursery,
    clippy::pedantic,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links,
    rustdoc::missing_crate_level_docs,
    rustdoc::private_doc_tests,
    rustdoc::invalid_codeblock_attributes,
    rustdoc::invalid_html_tags,
    rustdoc::invalid_rust_codeblocks,
    rustdoc::bare_urls,
    warnings,
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    keyword_idents,
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    non_ascii_idents,
    noop_method_call,
    pointer_structural_match,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc
)]

use std::{env, sync::Arc};

use futures::stream::StreamExt;
use sparkle_cache_postgres::Cache;
use sparkle_convenience::Bot;
use twilight_gateway::EventTypeFlags;
use twilight_model::gateway::{event::Event, Intents};

mod database;
mod interaction;

#[derive(Debug)]
pub struct Context {
    bot: Bot,
    cache: Cache,
}

impl Context {
    async fn handle_event(&self, event: Event) -> Result<(), anyhow::Error> {
        if let Event::InteractionCreate(interaction) = event {
            self.handle_interaction(interaction.0).await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (mut bot, mut events) = Bot::new(
        env::var("TIMEZONER_BOT_TOKEN")?,
        Intents::empty(),
        EventTypeFlags::all(),
    )
    .await?;
    bot.set_logging_channel(env::var("LOGGING_CHANNEL_ID")?.parse()?)
        .await?;
    bot.set_logging_file("timezoner_errors.txt".to_owned());

    let ctx = Arc::new(Context {
        bot,
        cache: Cache::new(&env::var("TIMEZONER_DATABASE_URL")?).await?,
    });
    ctx.set_commands().await?;
    ctx.init_db().await?;

    while let Some((_, event)) = events.next().await {
        let ctx_ref = Arc::clone(&ctx);
        tokio::spawn(async move {
            if let Err(err) = ctx_ref.handle_event(event).await {
                ctx_ref.bot.log(format!("{err:?}")).await;
            };
        });
    }

    Ok(())
}
