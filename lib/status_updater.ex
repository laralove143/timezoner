defmodule Timezoner.StatusUpdater do
  use GenServer

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  @impl true
  def init(_) do
    {:ok, nil}
  end

  def start_scheduling do
    GenServer.cast(__MODULE__, :start_scheduling)
  end

  @impl true
  def handle_cast(:start_scheduling, _) do
    send(self(), :update_status)

    {:noreply, nil}
  end

  @impl true
  def handle_info(:update_status, _) do
    guild_count = Nostrum.Cache.GuildCache.all() |> Enum.count()
    Nostrum.Api.Self.update_status(:online, "for times in #{guild_count} servers!", 3)

    Process.send_after(self(), :update_status, :timer.hours(1))

    {:noreply, nil}
  end
end
