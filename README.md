[join the server]: https://discord.gg/KUMdnjcE97

# timezoner

[![](https://custom-icon-badges.demolab.com/badge/add_to_your_server-invite-5865F2?logo=discord&logoColor=ffffff)](https://discord.com/api/oauth2/authorize?client_id=909820903574106203&permissions=536947776&scope=bot%20applications.commands) [![](https://custom-icon-badges.demolab.com/discord/903367565349384202?color=5865F2&logo=comment-discussion&label=join%20the%20server)](https://discord.gg/KUMdnjcE97) ![](https://custom-icon-badges.demolab.com/badge/dynamic/json?url=https://jsonblob.com/api/jsonBlob/1081686538592731136&logo=graph&label=%20&query=$.server_count&prefix=used%20in%20&suffix=%20servers&color=555555)

goodbye to timezone conversions! a discord bot that converts times so that everyone sees it in their own timezone

## features

### convert a time in a message

when there's a time in a message, the bot will add a reaction to it, just hit that reaction and everyone magically sees the time in their own timezone!

- only the person that sent the message needs to set their timezone, the ones reading the time don't even need to do anything
- supports basically all the time formats

![example](examples/sent.gif)

and someone 8 hours behind sees:

![example](examples/shown.png)

### share a date

wanna include a date? the `/date` command is in your command

- you can style it too, showing just the date for example

![example](examples/date.gif)

### share a date in dms

you can even share a date in dms or other servers, just use the `/copy` command

- consider asking the mods to add the bot to the other server though :)

![example](examples/copy.gif)

### learn what time it is for someone

wanna know if your friend is asleep for example? now you can with just right clicking/tapping on a user then pressing _apps_ and then _get current time for user_

![example](examples/get_current_time.gif)

## getting started

all you have to do is use the `/timezone` command to set your timezone, it takes just a few seconds

- only the people that are sharing times need to do this, people seeing the times don't need to do anything!

![example](examples/timezone.gif)

## something not working? let's see

### bot doesn't react to a message

- make sure the person that sent the message hasn't blocked the bot
- run the `/help` command in the channel where this is happening
- it'll probably warn you about missing permissions, if not, [join the server]

### missing permissions

- if you unticked any permissions when adding the bot, kick the bot and invite it again without unticking any permissions, it actually needs them!
- remove the bot from channel or category permissions
  - **on desktop:**
    1. right click on a channel or category
    2. click _edit channel/category_
    3. click _permissions_
    4. click _advanced permissions_ if its closed
    5. select _timezoner_ or any roles the bot has
    6. scroll all the way down and click _remove ..._ or change all the _X_ to _/_
  - **on mobile:**
    1. hold down on a channel or category
    2. tap _edit channel/category_
    3. tap _channel/category permissions_
    4. tap _advanced view_ if its not where you are
    5. tap _edit_ then _-_ on any roles the bot has or tap the role and change all the _X_ to _/_

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

- [interchannel message mover](https://github.com/laralove143/interchannel-message-mover): a discord bot to move messages between channels

## sponsors

- [jason](https://github.com/zudsniper): thank you for funding the hosting!
- wanna see your name here? [support me with whatever amount you wish :)](https://github.com/sponsors/laralove143)

## terms of service

- *"i" refers to lara kayaalp, the developer of timezoner*
- *"you" refers to all users of the bot*

### privacy

- your timezone is only used to convert times to timestamps
- only your currently selected timezone is saved, no history is kept
- no other data, including message content, is saved anywhere

### disclaimers

- the webhooks the bot executes copy your message's content, nickname and avatar
- i am not responsible for this copied data
- i am the sole owner and developer of this bot
- the hosted application is built directly from this repo

### self-hosting

these clauses override all other licenses:

- *"you" refers to the self-hoster of the bot*
- you may not advertise the self-hosted bot, such as in app directory or on websites such as top.gg
- the self-hosted bot may not be in over 5 guilds
- you must direct the feedback/support requests to me
