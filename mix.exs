defmodule Timezoner.MixProject do
  use Mix.Project

  def project do
    [
      app: :timezoner,
      version: "0.1.0",
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp deps do
    [
      {:nostrum, "~> 0.10"}
    ]
  end
end
