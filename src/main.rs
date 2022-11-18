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
    unreachable_pub,
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

use std::{env, sync::Arc};

use futures::stream::StreamExt;
use sparkle_convenience::Bot;
use twilight_model::gateway::Intents;

struct Context {
    bot: Bot,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let (bot, mut events) = Bot::new(
        env::var("TIMEZONER_BOT_TOKEN")?,
        Intents::GUILDS
            | Intents::GUILD_WEBHOOKS
            | Intents::GUILD_MESSAGES
            | Intents::MESSAGE_CONTENT,
        Some(env::var("LOGGING_CHANNEL_ID")?.parse()?),
        Some("timezoner errors.txt".to_owned()),
    )
    .await?;

    let ctx = Arc::new(Context { bot });

    while let Some((_, event)) = events.next().await {}

    Ok(())
}
