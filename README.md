[join the server]: https://discord.com/invite/KUMdnjcE97

# timezoner

[![add to your server](https://custom-icon-badges.demolab.com/badge/add_to_your_server-invite-5865F2?style=for-the-badge&logo=discord&logoColor=ffffff)](https://discord.com/api/oauth2/authorize?client_id=909820903574106203&permissions=536947776&scope=bot%20applications.commands)
[![join the server](https://custom-icon-badges.demolab.com/discord/903367565349384202?style=for-the-badge&color=5865F2&logo=comment-discussion&label=join%20the%20server)][join the server]
[![number of servers the bot is in](https://custom-icon-badges.demolab.com/badge/dynamic/json?url=https://api.jsonstorage.net/v1/json/52e7ddba-9c54-4f66-8e42-5aff2634f2fa/fd6b3135-0275-4f8a-8cfc-3e8910da1743&style=for-the-badge&color=555555&logo=graph&label=%20&prefix=used%20in%20&query=$.guild_count&suffix=%20servers)](#timezoner)
[![number of times the bot converted a time](https://custom-icon-badges.demolab.com/badge/dynamic/json?url=https://api.jsonstorage.net/v1/json/52e7ddba-9c54-4f66-8e42-5aff2634f2fa/fd6b3135-0275-4f8a-8cfc-3e8910da1743&style=for-the-badge&color=555555&logo=clock&label=%20&prefix=converted%20&query=$.usage_count&suffix=%20times)](#timezoner)

goodbye to timezone conversions!
a discord bot that converts times so that everyone sees them in their own timezone

## features

### convert a time in a message

when there's a time in a message, the bot will add a reaction to it,
just hit that reaction and everyone magically sees the time in their own timezone!

- only the person that sent the message needs to set their timezone,
the ones reading the time don't even need to do anything
- you can convert times even if you didn't send the message, it'll just dm you instead!
- supports basically all the time formats

![example](https://github.com/laralove143/timezoner/blob/main/examples/sent.gif?raw=true)

and someone 8 hours behind sees:

![example](https://github.com/laralove143/timezoner/blob/main/examples/shown.png?raw=true)

### share a date

wanna include a date? the `/date` command is in your command

- you can style it too, showing just the date for example

![example](https://github.com/laralove143/timezoner/blob/main/examples/date.gif?raw=true)

### share a date ANYwhere

you can share a date in dms, other servers, even put it in your bio! just use
the `/copy` command

- consider asking the mods to add the bot to the other server though :)

![example](https://github.com/laralove143/timezoner/blob/main/examples/copy.gif?raw=true)

### learn what time it is for someone

wanna know if your friend is asleep for example? now you can with just right
clicking/tapping on a user then pressing _apps_ and then _get current time for user_

![example](https://github.com/laralove143/timezoner/blob/main/examples/get_current_time.gif?raw=true)

## getting started

all you have to do is use the `/timezone` command to set your timezone,
it takes just a few seconds

- only the people that are sharing times need to do this,
people seeing the times don't need to do anything!

![example](https://github.com/laralove143/timezoner/blob/main/examples/timezone.gif?raw=true)

the bot can even guess your timezone from your discord language

![example](https://github.com/laralove143/timezoner/blob/main/examples/timezone_detect.png?raw=true)

## something not working? let's see

### bot doesn't react to a message

1. make sure the person that sent the message hasn't blocked the bot
2. run the `/help` command in the channel where this is happening
3. it'll probably warn you about missing permissions, if not, [join the server]

### missing permissions

1. if you unticked any permissions when adding the bot, kick the bot and invite
  it again without unticking any permissions, it actually needs them!
2. fix channel or category permissions
    1. right click on the channel or category where the bot doesn't work
    _(hold down on it on mobile)_
    2. select **edit channel/category** then press **permissions**
    _(**channel/category permissions** on mobile)_
    3. click **advanced permissions** if it's closed _(**advanced view** on mobile)_
    4. press that tiny **+ button** _(**add member** on mobile)_
    5. search **timezoner** and select the one with the avatar
    6. press **the green tick** next to permissions listed below
        - view channel
        - manage webhooks
        - send messages
        - add reactions
        - manage messages
        - read message history

### something else? how could this happen??

please [join the server] and tell me

## let's get in touch

- get announcements for updates
- help shape the future of the bots by answering my feedback questions
- peek into the future of the bots aka upcoming features
- tell your feature ideas or bug reports
- get help if you're having trouble with something
- or just have a chat

[join the server]

## check out my other bots

- [interchannel message mover](https://github.com/laralove143/interchannel-message-mover): a discord bot to move
  messages between channels

## sponsors

- [jason](https://github.com/zudsniper)
- [grace](https://github.com/Grace-xo)
- wanna see your name here? [support me with whatever amount you wish :)](https://github.com/sponsors/laralove143)

## terms of service

- _"i" refers to lara kayaalp, the developer of timezoner_
- _"you" refers to all users of the bot_

### privacy

- your timezone is only used to convert times to timestamps
- only your currently selected timezone is saved, no history is kept
- unidentifiable, anonymous usage data is collected and used
for advertisement and analytics
- no other data, including message content, is saved anywhere

### disclaimers

- the webhooks the bot executes copy your message's content, nickname and avatar
- i am not responsible for this copied data
- i am the sole owner and developer of this bot
- the hosted application is built directly from this repo

### self-hosting

these clauses override all other licenses:

- _"you" refers to the self-hoster of the bot_
- you may not advertise the self-hosted bot, such as in app directory
or on websites such as top.gg
- the self-hosted bot may not be in over 5 guilds
- you must direct the feedback/support requests to me
