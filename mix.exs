defmodule ExSodium.MixProject do
  use Mix.Project

  @source_url "https://github.com/piyushthapa/ex_sodium"
  @version "0.1.1"

  def project do
    [
      app: :ex_sodium,
      version: @version,
      elixir: "~> 1.18",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      description: description()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  defp description do
    "Elixir bindings for the Sodium cryptographic library."
  end

  def package() do
    [
      name: :ex_sodium,
      files: ["lib", "mix.exs", "README*", "LICENSE*", "priv"],
      maintainers: ["TxBody", "Piyush"],
      licenses: ["MIT"],
      links: %{
        "Changelog" => "#{@source_url}/blob/master/CHANGELOG.md",
        "Docs" => "https://hexdocs.pm/ex_sodium",
        "GitHub" => @source_url
      }
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      # {:dep_from_hexpm, "~> 0.3.0"},
      # {:dep_from_git, git: "https://github.com/elixir-lang/my_dep.git", tag: "0.1.0"}
      {:rustler, "~> 0.36.2", runtime: false},
      {:ex_doc, ">= 0.0.0", only: :dev, runtime: false}
    ]
  end
end
