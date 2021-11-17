defmodule QRCodeRust do
  use Rustler, otp_app: :qrcode_rust, crate: "qrcoderust_nif"

  def generate_svg(_data, _qr_kind, _ec_level, _fg_color, _bg_color, _include_xml_declaration),
    do: :erlang.nif_error(:nif_not_loaded)

  def generate_svg(data, qr_kind, ec_level, fg_color, bg_color) do
    generate_svg(data, qr_kind, ec_level, fg_color, bg_color, false)
  end

  def generate_svg(data) do
    generate_svg(data, :square, :ec_l, "#222", "#fff")
  end
end
