# credo:disable-for-next-line Credo.Check.Refactor.ModuleDependencies
defmodule Timezoner.Interactions.Timezone do
  @behaviour Timezoner.Interactions.Behaviour

  alias Nostrum.Api.Interaction
  alias Nostrum.Constants.ApplicationCommandOptionType
  alias Nostrum.Constants.ApplicationCommandType
  alias Timezoner.Component
  alias Timezoner.Error
  alias Timezoner.InteractionResponse

  @impl Timezoner.Interactions.Behaviour
  def name, do: "timezone"

  @impl Timezoner.Interactions.Behaviour
  def command do
    option = %{
      name: "city",
      description: "Your city",
      type: ApplicationCommandOptionType.string(),
      required: true
    }

    %{
      name: name(),
      description: "Set your timezone to send times",
      type: ApplicationCommandType.chat_input(),
      options: [option]
    }
  end

  @impl Timezoner.Interactions.Behaviour
  def handle(interaction) do
    city = List.first(interaction.data.options).value

    response =
      city
      |> Geocoder.call()
      |> response()

    interaction
    |> Interaction.create_response(response)
    |> Error.handle()
  end

  def response({:ok, %Geocoder.Coords{lat: lat, lon: lon}}) do
    {:ok, tz} = TzWorld.timezone_at({lon, lat})

    InteractionResponse.channel_message_with_source([
      Component.section("https://cdn.lara.lv/emoji/partying-face.webp", [
        Component.text("# Timezone set"),
        Component.text(
          "I set your timezone to **#{tz}**. Now, you can convert times however you want!"
        )
      ])
    ])
  end

  def response({:error, _}) do
    InteractionResponse.channel_message_with_source([
      Component.section("https://cdn.lara.lv/emoji/pensive.webp", [
        Component.text("# City not found"),
        Component.text("I couldn't find that city, please make sure you spelled it correctly."),
        Component.text(
          "-# If you spelled it right, make sure the city that you're living in actually exists."
        )
      ])
    ])
  end
end
