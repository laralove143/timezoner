defmodule Timezoner.Consumer do
  use Nostrum.Consumer

  alias Nostrum.Api.ApplicationCommand
  alias Timezoner.Error

  defp commands do
    [Timezoner.Interactions.Help]
  end

  def handle_event({:READY, _data, _ws_state}) do
    guild_id = Application.get_env(:timezoner, :guild_id)
    commands = Enum.map(commands(), fn cmd -> cmd.command() end)

    if guild_id do
      ApplicationCommand.bulk_overwrite_guild_commands(guild_id, commands) |> Error.handle()
    end

    ApplicationCommand.bulk_overwrite_global_commands(commands) |> Error.handle()
  end

  def handle_event({:INTERACTION_CREATE, interaction, _ws_state}) do
    command = Enum.find(commands(), fn cmd -> cmd.name() == interaction.data.name end)

    command.handle(interaction)
  end
end
