defmodule Timezoner.DatetimeParser do
  @behaviour NimblePool

  def start_link do
    NimblePool.start_link(worker: {__MODULE__, []}, name: __MODULE__)
  end

  def child_spec(_) do
    %{
      id: __MODULE__,
      start: {__MODULE__, :start_link, []}
    }
  end

  def parse(content, tz) do
    payload = Jason.encode!(%{content: content, tz: tz})

    NimblePool.checkout!(__MODULE__, :checkout, fn _, port ->
      send_command(port, payload)
    end)
  end

  defp send_command(port, payload) do
    send(port, {self(), {:command, payload}})

    receive do
      {^port, {:data, data}} ->
        data = Jason.decode(data)

        Process.unlink(port)
        {data, :ok}

      {^port, {:exit_status, status}} ->
        {{:error, "datetime parse process exited with status #{status}"}, :close}
    after
      5000 ->
        {{:error, "datetime parse timeout"}, :close}
    end
  end

  @impl NimblePool
  def init_worker(pool_state) do
    priv_dir = Application.app_dir(:timezoner, "priv")

    port =
      Port.open({:spawn, "#{priv_dir}/.venv/bin/python #{priv_dir}/datetime_parse.py"}, [
        {:packet, 4},
        :binary,
        :exit_status,
        :nouse_stdio
      ])

    {:ok, port, pool_state}
  end

  @impl NimblePool
  def handle_checkout(:checkout, {pid, _}, port, pool_state) do
    Port.connect(port, pid)
    {:ok, port, port, pool_state}
  end

  @impl NimblePool
  def handle_checkin(:ok, _, port, pool_state) do
    {:ok, port, pool_state}
  end

  @impl NimblePool
  def handle_checkin(:close, _, _, pool_state) do
    {:remove, :closed, pool_state}
  end
end
