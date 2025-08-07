defmodule Timezoner.StatusUpdater do
  use GenServer

  alias Nostrum.Api.Self
  alias Nostrum.Cache.GuildCache

  def start_link(_) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  @impl GenServer
  def init(_) do
    {:ok, nil}
  end

  def start_scheduling do
    GenServer.cast(__MODULE__, :start_scheduling)
  end

  @impl GenServer
  def handle_cast(:start_scheduling, _) do
    send(self(), :update_status)

    {:noreply, nil}
  end

  @impl GenServer
  def handle_info(:update_status, _) do
    guild_count = Enum.count(GuildCache.all())
    Self.update_status(:online, "for times in #{guild_count} servers!", 3)

    Process.send_after(self(), :update_status, :timer.hours(1))

    {:noreply, nil}
  end
end
