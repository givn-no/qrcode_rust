defmodule QRCodeRust do
  use Rustler, otp_app: :qrcode_rust, crate: "qrcoderust_nif"

  def generate_svg(data, qr_kind \\ :square, ec_level \\ :ec_l, fg_color \\ "#000", bg_color \\ "#fff", remove_middle \\ false, include_xml_declaration \\ false, min_version \\ nil)

  def generate_svg(_data, _qr_kind, _ec_level, _fg_color, _bg_color, _remove_middle, _include_xml_declaration, _min_version) do
    :erlang.nif_error(:nif_not_loaded)
  end
end
