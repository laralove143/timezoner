defmodule Timezoner.Component do
  def section(media_url) do
    %{
      type: 9,
      components: [],
      accessory: %{
        type: 11,
        media: %{
          url: media_url
        }
      }
    }
  end

  def text(text) do
    %{type: 10, content: text}
  end

  def container do
    %{
      type: 17,
      accent_color: 0x57E8F2,
      components: []
    }
  end

  def put_text(component, text) do
    # credo:disable-for-next-line Credo.Check.Refactor.AppendSingleItem
    %{component | components: component.components ++ [text(text)]}
  end

  def put_media(component, media_url) do
    %{
      component
      | # credo:disable-for-next-line Credo.Check.Refactor.AppendSingleItem
        components: component.components ++ [%{type: 12, items: [%{media: %{url: media_url}}]}]
    }
  end
end
