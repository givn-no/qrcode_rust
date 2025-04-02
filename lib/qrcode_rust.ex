defmodule QRCodeRust do
  mix_config = Mix.Project.config()
  version = mix_config[:version]
  github_url = mix_config[:package][:links]["GitHub"]

  use RustlerPrecompiled,
    otp_app: :qrcode_rust,
    crate: "qrcoderust_nif",
    base_url: "#{github_url}/releases/download/v#{version}",
    force_build: System.get_env("QRCODE_RUST_BUILD") in ["1", "true"],
    targets:
      Enum.uniq(["aarch64-unknown-linux-musl" | RustlerPrecompiled.Config.default_targets()]),
    version: version,
    nif_versions: ["2.15", "2.16"]

  def generate_svg(
        data,
        qr_kind \\ :square,
        ec_level \\ :ec_l,
        fg_color \\ "#000",
        bg_color \\ "#fff",
        remove_middle \\ false,
        include_xml_declaration \\ false,
        min_version \\ nil
      )

  def generate_svg(
        _data,
        _qr_kind,
        _ec_level,
        _fg_color,
        _bg_color,
        _remove_middle,
        _include_xml_declaration,
        _min_version
      ) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
