defmodule Timezoner.Component do
  def section(media_url, components) do
    %{
      type: 9,
      components: components,
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

  def media_gallery(media_urls) do
    %{type: 12, items: Enum.map(media_urls, &%{media: %{url: &1}})}
  end

  def container(components) do
    %{
      type: 17,
      accent_color: 0x57E8F2,
      components: components
    }
  end
end
