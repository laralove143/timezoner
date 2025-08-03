defmodule Timezoner.Main do
  use Application

  def start(_, _) do
    Supervisor.start_link(
      [
        Timezoner.Consumer,
        Timezoner.StatusUpdater,
        Timezoner.DatetimeParser
      ],
      strategy: :one_for_one,
      name: __MODULE__
    )
  end
end
