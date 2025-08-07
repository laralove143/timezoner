defmodule Timezoner.InteractionResponse do
  import Bitwise

  def channel_message_with_source(components) do
    %{
      type: 4,
      data: %{
        flags: 1 <<< 15,
        components: components
      }
    }
  end
end
