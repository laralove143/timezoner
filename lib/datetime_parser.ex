defmodule Timezoner.DatetimeParser do
  use GenServer

  alias Timezoner.Error
  require Logger

  def start_link(_) do
    GenServer.start_link(__MODULE__, %{}, name: __MODULE__)
  end

  defp start_port do
    priv_dir = Application.app_dir(:timezoner, "priv")

    Port.open({:spawn, "#{priv_dir}/.venv/bin/python #{priv_dir}/datetime_parse.py"}, [
      {:packet, 4},
      :binary,
      :exit_status,
      :nouse_stdio
    ])
  end

  @impl GenServer
  def init(_) do
    {:ok, %{port: start_port()}}
  end

  def parse(content, tz) do
    __MODULE__
    |> GenServer.call({:parse, content, tz})
    |> Error.handle("parsing datetime failed")
  end

  @impl GenServer
  def handle_call({:parse, content, tz}, _, state) do
    payload = Jason.encode!(%{content: content, tz: tz})
    Port.command(state.port, payload)

    receive do
      {_, {:data, data}} ->
        {:reply, Jason.decode(data), state}

      {:error, err} ->
        {:reply, {:error, err}, state}
    end
  end

  @impl GenServer
  def handle_info({port, {:exit_status, status}}, state) when port == state.port do
    Logger.warning("datetime parse process exited with status #{status}, restarting")

    {:noreply, %{state | port: start_port()}}
  end
end
