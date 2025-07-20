defmodule Timezoner.Main do
  use Application

  def start(_type, _args) do
    Supervisor.start_link([Timezoner.Consumer],
      strategy: :one_for_one,
      name: Timezoner.Supervisor
    )
  end
end
