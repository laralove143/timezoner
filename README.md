# timezoner

[![add it to your server - invite](https://img.shields.io/badge/add_it_to_your_server-invite-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.com/api/oauth2/authorize?client_id=909820903574106203&permissions=536880128&scope=bot%20applications.commands)  
[![talk to me - join server](https://img.shields.io/badge/talk_to_me-join-5865F2?style=for-the-badge&logo=discord&logoColor=white)](https://discord.gg/6vAzfFj8xG)

goodbye to timezone conversions! just type some time in chat and everyone magically sees it in their own timezone!

![example](example.gif)

## usage
to send times, the bot needs to know your timezone first, just type `/timezone` and it’ll suggest timezones for you
- you only do this once of course
- only the people that type times need to do this, the rest reading the time don't need to do anything!

there’s also a `/copy` command that you can use to share times in dms etc.

## terms of service
the hosted application is built directly from this repo
### your data
- your selected timezone is only used to convert times to timestamps
- the messages are not saved anywhere
- your discord information (such as nicknames, avatars) is only cached and used to resend your messages with converted times
### self-hosting
- you may not advertise the self-hosted bot, such as on websites such as top.gg
- the self-hosted bot may not be in over 5 guilds
- you must direct the feedback/support requests to me
### disclaimers
- the webhooks the bot executes copy your message content, nickname and avatar, the bot is not responsible for what it copied
- if the feedback you gave is implemented without credit, please contact me to be given credit
- you may not claim you are the owner, developer, motivator or hoster of this bot, the original idea of the bot is mine alone
- i am the only person responsible for the bot, it has no support etc. team
### contact
- if you have negative criticism, please let me know so i can improve it
- discord server: https://discord.gg/6vAzfFj8xG
- discord username: laralove#7186
- email: laralove143@icloud.com
- github issues
- do not spam these sources

## nerdy stuff
made with [rust](https://www.rust-lang.org) and [twilight](https://github.com/twilight-rs/twilight) and sqlite

*made by me (laralove143), licensed MIT*
