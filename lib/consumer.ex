defmodule Timezoner.Consumer do
  use Nostrum.Consumer

  alias Timezoner.Interactions

  def handle_event({:READY, _, _}) do
    Interactions.register()

    Timezoner.StatusUpdater.start_scheduling()
  end

  def handle_event({:INTERACTION_CREATE, interaction, _}) do
    Interactions.handle(interaction)
  end
end
