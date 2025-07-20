defmodule Timezoner.Interactions.Help do
  @behaviour Timezoner.Interactions.Behaviour

  alias Nostrum.Api.Interaction
  alias Timezoner.{Component, Error}
  alias Nostrum.Struct.Component.{ActionRow, Button}

  def name, do: "help"

  def handle(interaction) do
    title_section =
      Component.section("https://cdn.lara.lv/emoji/sos.gif")
      |> Component.put_text("# Timezoner")
      |> Component.put_text(
        "I let you send times and dates that everyone sees in their own timezone."
      )

    convert_container =
      Component.container()
      |> Component.put_text("### Convert a time or date in a message")
      |> Component.put_text(
        "When there's a time in a message, the bot will add a reaction to it. Simply hit that reaction and everyone magically sees the time in their own timezone."
      )
      |> Component.put_text(
        "-# Only the person that sent the message needs to set their timezone, the ones reading the time don't even need to do anything."
      )
      |> Component.put_media("https://cdn.lara.lv/timezoner/help/convert.gif")

    date_container =
      Component.container()
      |> Component.put_text("### Send a time or date")
      |> Component.put_text(
        "You can also send a time or date directly by using the command `/date`."
      )
      |> Component.put_text("-# You can style it too, showing just the date for example.")
      |> Component.put_media("https://cdn.lara.lv/timezoner/help/date.gif")

    copy_container =
      Component.container()
      |> Component.put_text("### Share in DMs or another server")
      |> Component.put_text(
        "Open the menu on a message and select *Copy Text* to share a time or date anywhere."
      )
      |> Component.put_text(
        "-# You can even use this in your bio, maybe to show what your noon is to others."
      )
      |> Component.put_media("https://cdn.lara.lv/timezoner/help/copy.mov")

    user_time_container =
      Component.container()
      |> Component.put_text("### Learn what time it is for someone")
      |> Component.put_text(
        "Want to know if your friend is asleep for example? Well, you can by opening the menu on a user and choosing *Apps -> Get Current Time*."
      )
      |> Component.put_text(
        "-# Just remember that the other user needs to set their timezone first."
      )
      |> Component.put_media("https://cdn.lara.lv/timezoner/help/user-time.gif")

    footer_section = Component.text("-# Use the buttons below for more information.")

    action_row =
      ActionRow.action_row()
      |> ActionRow.append(
        Button.link_button("Homepage", "https://timezoner.lara.lv",
          emoji: %{
            id: 1_396_299_330_457_178_293,
            name: "globe_showing_europe_africa",
            animated: true
          }
        )
      )
      |> ActionRow.append(
        Button.link_button("Support Server", "https://discord.com/invite/KUMdnjcE97",
          emoji: %{
            id: 1_396_297_056_750_014_546,
            name: "wave",
            animated: true
          }
        )
      )

    Interaction.create_response(interaction, %{
      type: 4,
      data: %{
        flags: Bitwise.bsl(1, 15),
        components: [
          title_section,
          convert_container,
          date_container,
          copy_container,
          user_time_container,
          footer_section,
          action_row
        ]
      }
    })
    |> Error.handle()
  end

  def command do
    %{
      name: name(),
      description: "Get information about the bot",
      type: Nostrum.Constants.ApplicationCommandType.chat_input()
    }
  end
end
