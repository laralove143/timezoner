use twilight_interactions::command::{CommandModel, CreateCommand};
use twilight_model::{channel::message::MessageFlags, http::interaction::InteractionResponseData};
use twilight_util::builder::{
    embed::{EmbedBuilder, EmbedFieldBuilder},
    InteractionResponseDataBuilder,
};

/// the help command
#[derive(CommandModel, CreateCommand)]
#[command(name = "help", desc = "get help on how to use bot or contact me")]
pub struct Help;

/// run the command, returning the response data
pub fn run() -> InteractionResponseData {
    let embed = EmbedBuilder::new()
        .color(0x00EC_E6B7)
        .description(
"hii! im a discord bot that automatically makes the times you send appear in everyone's own \
timezone using discord magic!"
        )
        .field(EmbedFieldBuilder::new("how do i actually use it to send times",
"first set your timezone using `/timezone`
just mention a time in a format like `13:40` `1:40pm` `1pm` and i'll resend your message, like
> you: anyone down to watch a movie between 2pm and 3:30pm?
> me: anyone down to watch a movie between 2pm (2:00 PM) and 3:30pm (3:30 PM)
where the `2:00 PM` and `3:30 PM` are in everyone's own timezone, discord even converts formats so \
for some people it'll be `14:00 PM`"))
        .field(EmbedFieldBuilder::new("how do i use it to see the converted times",
"you dont have to do anything! the times in `()` that come after normal text (note the fancy \
gray background) appear in your own timezone"))
        .field(EmbedFieldBuilder::new("why am i a bot now",
"you can't actually edit other's messages, what i do is delete your message and resend it \
using your avatar, nickname and the new message content"))
        .field(EmbedFieldBuilder::new("how do i use this in other servers/dms",
"you can use the `/copy` command, copy the message, paste it anywhere and get rid of the \
`` `'s at the start and end, you could also ask me to be added to the server :)"))
        .field(EmbedFieldBuilder::new("how does this even work",
"the way discord works with this is timestamps, basically number of seconds since 1970, i \
just convert times into discord format and your discord client does the rest"))
        .field(EmbedFieldBuilder::new("how do i change my timezone",
"just run the `/timezone` command again, it'll re-set your timezone"))
        .field(EmbedFieldBuilder::new("it doesn't work in a channel!",
"make sure i have `view channel`, `manage webhooks` and `manage messages` permissions in the\
 channel/category, if it still doesn't work, please contact me"))
        .field(EmbedFieldBuilder::new("terms of service",
"pretty basic stuff, but here: \
https://github.com/laralove143/timezoner-discord-bot#terms-of-service"))
        .field(EmbedFieldBuilder::new("i have a bug report/feature request/feedback",
                                      "please contact my developer"))
        .field(EmbedFieldBuilder::new("contact my developer",
"join their server: https://discord.gg/6vAzfFj8xG
friend them: <@258568289746288641>
email them: laralove143@icloud.com
make a github issue: https://github.com/laralove143/timezoner-discord-bot/issues/new"))
        .field(EmbedFieldBuilder::new("i love this bot, how can i help",
"first of all thank you :) any feedback to improve the bot helps, you can tell about it to \
your friends too
also if you really want to, you can contact me to donate, a dollar is 15 liras in turkey so even 10\
 bucks is a lot here! (it'd also be the first donation i ever get)")).build();

    InteractionResponseDataBuilder::new()
        .embeds([embed])
        .flags(MessageFlags::EPHEMERAL)
        .build()
}
