# timezoner

[<img src="add_to_server_badge.png" height="32"/>]
[<img src="join_support_server_badge.png" height="32"/>]

[<img src="add_to_server_badge.png" height="32"/>]: https://discord.com/api/oauth2/authorize?client_id=909820903574106203&permissions=536880128&scope=bot%20applications.commands
[<img src="join_support_server_badge.png" height="32"/>]: https://discord.gg/6vAzfFj8xG

goodbye to timezone conversions!

just type some time in chat and everyone magically sees it in their own timezone

![example](example.gif)

and someone 10 hours ahead sees:

![example](example.png)

setting your timezone couldn't be easier:

![example](example_timezone.gif)

*thank you to [jason](https://github.com/zudsniper) for funding the hosting!*

## features

- does all the conversion automatically, no commands or dms or annoyances
- the ones reading the times don't need to do anything
- supports all the sane formats: `1pm` `1:30 Pm` `13:30`...
- has a `/copy` command that you can use to share times in dms etc
- setting up your timezone couldn't be easier, scroll down for an example

## contact

- i am the only person responsible for the bot, it has no team
- discord server: <https://discord.gg/6vAzfFj8xG>
- discord username: laralove#7186
- email: laralove143@icloud.com
- github issues

## terms of service

- *"i" refers to laralove143, the developer of timezoner*
- *"you" refers to all users of the bot*

### privacy

- your timezone is only used to convert times to timestamps
- only your currently selected timezone is saved, no history is kept
- all saved data is encrypted
- no other data, including message content is saved anywhere

### disclaimers

- the webhooks the bot executes copy your message's content, nickname and avatar
- i am not responsible for this copied data
- i am the sole owner and developer of this bot
- the hosted application is built directly from this repo

### self-hosting

these clauses override all other licenses:

- *"you" refers to the self-hoster of the bot*
- you may not advertise the self-hosted bot, such as on websites such as top.gg
- the self-hosted bot may not be in over 5 guilds
- you must direct the feedback/support requests to me

## nerdy stuff

made with [rust] and [twilight] and sqlite

[rust]: https://www.rust-lang.org
[twilight]: https://github.com/twilight-rs/twilight
