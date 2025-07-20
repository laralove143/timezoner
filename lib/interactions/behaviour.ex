defmodule Timezoner.Interactions.Behaviour do
  @callback name() :: String.t()
  @callback handle(interaction :: Nostrum.Struct.Interaction.t()) :: :ok
  @callback command() :: Nostrum.Struct.ApplicationCommand.application_command_map()
end
