defmodule Timezoner.Interactions do
  alias Nostrum.Api.ApplicationCommand
  alias Timezoner.Error

  def commands do
    [Timezoner.Interactions.Help, Timezoner.Interactions.Timezone]
  end

  def register do
    guild_id = Application.get_env(:timezoner, :guild_id)
    commands = Enum.map(commands(), fn cmd -> cmd.command() end)

    if guild_id do
      guild_id
      |> ApplicationCommand.bulk_overwrite_guild_commands(commands)
      |> Error.handle()
    end

    commands
    |> ApplicationCommand.bulk_overwrite_global_commands()
    |> Error.handle()
  end

  def handle(interaction) do
    command =
      Enum.find(commands(), fn cmd -> cmd.name() == interaction.data.name end)

    command.handle(interaction)
  end
end
