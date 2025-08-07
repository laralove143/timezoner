defmodule Timezoner.MessageCreate do
  alias Timezoner.DatetimeParser
  alias Timezoner.Error

  def handle(message) do
    {:ok, parsed} =
      message.content
      |> DatetimeParser.parse("UTC")
      |> Error.handle()
  end
end
