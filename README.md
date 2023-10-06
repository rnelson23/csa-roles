# CSA Roles Bot

## About
This Discord bot is made to replace the existing role selection bot in
SUNY Oswego's Computer Science Association Discord server. It allows users
to join and leave roles by using slash commands.

## Commands
The bot has two main slash commands, `/add` and `/remove`, which allow users
to join and leave roles, respectively. The commands take one argument, which
is the role to be added. Discord handles verifying that the role is valid.
The bot verifies that the role is allowed to be added/removed by the user.
The bot also has a `/list` command, which lists all the roles that can be
added/removed by the user. When using the add/remove commands, all
roles that can be mentioned by the user are listed in the menu, but only the
roles that can be added/removed are actually added/removed.

## Usage
To run the bot, simply build the Docker image and run it with an environment
variable for the bot token and the Discord server's ID. The bot token can be
found on the [Discord developer portal](https://discord.com/developers/applications),
and the server ID can be found by enabling developer mode in Discord and
right-clicking on the server icon.

The bot knows which roles can be added/removed based on the location of the
"CSA Bot" role. All roles underneath the bot role in the role list can be
added/removed by the bot.
