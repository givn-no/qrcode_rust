defmodule QrcodeRust.MixProject do
  use Mix.Project

  @source_url "https://github.com/givn-no/qrcode_rust"
  @version "0.4.0"

  def project do
    [
      app: :qrcode_rust,
      version: @version,
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      package: package(),
      hex: hex(),
      description: "QR code generator implemented as a rust nif"
    ]
  end

  defp package do
    [
      files: [
        "lib",
        "native/qrcoderust_nif/.cargo",
        "native/qrcoderust_nif/src",
        "native/qrcoderust_nif/Cargo*",
        "checksum-*.exs",
        "mix.exs"
      ],
      links: %{"GitHub" => @source_url},
      licenses: ["MIT"]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler_precompiled, ">= 0.0.0"},
      {:rustler, ">= 0.0.0", optional: true}
    ]
  end

  defp hex do
    [
      api_url: System.get_env("HEX_API_URL"),
      api_key: System.get_env("HEX_API_KEY")
    ]
  end
end
