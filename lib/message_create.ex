defmodule Timezoner.MessageCreate do
  alias Timezoner.DatetimeParser
  alias Timezoner.Error

  def handle(message) do
    message.content
    |> DatetimeParser.parse("UTC")
    |> Error.handle()
  end
end
