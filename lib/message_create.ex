defmodule Timezoner.MessageCreate do
  alias Timezoner.DatetimeParser

  def handle(message) do
    DatetimeParser.parse(message.content, "UTC") |> dbg()
  end
end
