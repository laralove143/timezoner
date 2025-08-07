defmodule Timezoner.Interactions.Behaviour do
  @callback name() :: any()
  @callback handle(interaction :: any()) :: any()
  @callback command() :: any()
end
