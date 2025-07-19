defmodule Timezoner.Application do
  use Application

  def start(_type, _args) do
    children = [
      Timezoner.Consumer
    ]

    opts = [strategy: :one_for_one, name: Timezoner.Supervisor]
    Supervisor.start_link(children, opts)
  end
end

defmodule Timezoner.Consumer do
  use Nostrum.Consumer

  alias Nostrum.Api.Message

  def handle_event({:MESSAGE_CREATE, msg, _ws_state}) do
    case msg.content do
      "!ping" ->
        Message.create(msg.channel_id, "pong!")

      _ ->
        :ignore
    end
  end
end
