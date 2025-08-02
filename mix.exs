defmodule Timezoner.MixProject do
  use Mix.Project

  def project do
    [
      app: :timezoner,
      version: "0.1.0",
      elixir: "~> 1.18",
      # credo:disable-for-next-line Credo.Check.Warning.MixEnv
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger],
      mod: {Timezoner.Main, []}
    ]
  end

  defp deps do
    [
      {:nostrum, "~> 0.10"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:exsync, "~> 0.4", only: :dev}
    ]
  end
end
