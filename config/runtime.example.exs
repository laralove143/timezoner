import Config

config :timezoner,
  guild_id: nil

config :nostrum,
  token: "bot token",
  ffmpeg: nil,
  log_full_events: Mix.env() != :prod,
  log_dispatch_events: Mix.env() != :prod,
  gateway_intents: [:guilds]

config :logger, :console,
  level: if(Mix.env() == :prod, do: :info, else: :debug),
  metadata: [:shard, :guild, :channel]
