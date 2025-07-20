defmodule Timezoner.Error do
  require Logger

  def handle(result, message \\ nil)

  def handle({:error, err}, message) do
    err_msg = inspect(err, pretty: true)

    if message do
      Logger.error("#{message}: #{err_msg}")
    else
      Logger.error(err_msg)
    end
  end

  def handle(_, _), do: :ok
end
