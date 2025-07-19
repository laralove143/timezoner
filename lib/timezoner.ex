defmodule Timezoner.Supervisor do
  use Supervisor

  def start_link(args) do
    Supervisor.start_link(__MODULE__, args, name: __MODULE__)
  end

  @impl true
  def init(_init_arg) do
    children = [Timezoner.Consumer]

    Supervisor.init(children, strategy: :one_for_one)
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
