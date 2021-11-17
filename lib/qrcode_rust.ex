defmodule QRCodeRust do
  use Rustler, otp_app: :qrcode_rust, crate: "qrcoderust_nif"

  def generate_svg(_data, _qr_kind, _ec_level, _fg_color, _bg_color),
    do: :erlang.nif_error(:nif_not_loaded)

  def generate_svg(data) do
    generate_svg(data, :square, :ec_l, "#222", "#fff")
  end
end
