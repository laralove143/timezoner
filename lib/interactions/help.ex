defmodule Timezoner.Interactions.Help do
  @behaviour Timezoner.Interactions.Behaviour

  alias Nostrum.Api.Interaction
  alias Nostrum.Constants.ApplicationCommandType
  alias Nostrum.Struct.Component.ActionRow
  alias Nostrum.Struct.Component.Button
  alias Timezoner.Component
  alias Timezoner.Error
  alias Timezoner.InteractionResponse

  @impl Timezoner.Interactions.Behaviour
  def name, do: "help"

  @impl Timezoner.Interactions.Behaviour
  def command do
    %{
      name: name(),
      description: "Get information about the bot",
      type: ApplicationCommandType.chat_input()
    }
  end

  @impl Timezoner.Interactions.Behaviour
  def handle(interaction) do
    response =
      InteractionResponse.channel_message_with_source([
        title_section(),
        convert_container(),
        date_container(),
        copy_container(),
        user_time_container(),
        footer_section(),
        action_row()
      ])

    interaction
    |> Interaction.create_response(response)
    |> Error.handle()
  end

  defp title_section do
    Component.section("https://cdn.lara.lv/emoji/sos.webp", [
      Component.text("# Timezoner"),
      Component.text("I let you send times and dates that everyone sees in their own timezone.")
    ])
  end

  defp convert_container do
    Component.container([
      Component.text("### Convert a time or date in a message"),
      Component.text(
        "When there's a time in a message, the bot will add a reaction to it. Simply hit that reaction and everyone magically sees the time in their own timezone."
      ),
      Component.text(
        "-# Only the person that sent the message needs to set their timezone, the ones reading the time don't even need to do anything."
      ),
      Component.media_gallery([
        "https://cdn.lara.lv/timezoner/help/placeholder-example.png"
      ])
    ])
  end

  defp date_container do
    Component.container([
      Component.text("### Send a time or date"),
      Component.text("You can also send a time or date directly by using the command `/date`."),
      Component.text("-# You can style it too, showing just the date for example."),
      Component.media_gallery([
        "https://cdn.lara.lv/timezoner/help/placeholder-example.png"
      ])
    ])
  end

  defp copy_container do
    Component.container([
      Component.text("### Share in DMs or another server"),
      Component.text(
        "Open the menu on a message and select *Copy Text* to share a time or date anywhere."
      ),
      Component.text(
        "-# You can even use this in your bio, maybe to show what your noon is to others."
      ),
      Component.media_gallery([
        "https://cdn.lara.lv/timezoner/help/placeholder-example.png"
      ])
    ])
  end

  defp user_time_container do
    Component.container([
      Component.text("### Learn what time it is for someone"),
      Component.text(
        "Want to know if your friend is asleep for example? Well, you can by opening the menu on a user and choosing *Apps -> Get Current Time*."
      ),
      Component.text("-# Just remember that the other user needs to set their timezone first."),
      Component.media_gallery([
        "https://cdn.lara.lv/timezoner/help/placeholder-example.png"
      ])
    ])
  end

  defp footer_section do
    Component.text("-# Use the buttons below for more information.")
  end

  defp action_row do
    ActionRow.action_row([
      Button.link_button("Homepage", "https://timezoner.lara.lv",
        emoji: %Nostrum.Struct.Emoji{
          id: 1_396_299_330_457_178_293,
          name: "globe_showing_europe_africa",
          animated: true
        }
      ),
      Button.link_button("Support Server", "https://discord.com/invite/KUMdnjcE97",
        emoji: %Nostrum.Struct.Emoji{
          id: 1_396_297_056_750_014_546,
          name: "wave",
          animated: true
        }
      )
    ])
  end
end
