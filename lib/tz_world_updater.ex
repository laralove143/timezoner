defmodule Timezoner.TzWorldUpdater do
  use GenServer

  alias TzWorld.Backend.DetsWithIndexCache

  def start_link(_) do
    GenServer.start_link(__MODULE__, nil, name: __MODULE__)
  end

  @impl GenServer
  def init(_) do
    send(self(), :update_data)
    {:ok, nil}
  end

  @impl GenServer
  def handle_info(:update_data, _) do
    TzWorld.Downloader.update_release(include_oceans: true)
    DetsWithIndexCache.reload_timezone_data()

    Process.send_after(self(), :update_data, :timer.hours(1))

    {:noreply, nil}
  end
end
