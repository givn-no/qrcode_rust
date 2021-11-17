defmodule QRCodeRust do
  use Rustler, otp_app: :qrcode_rust, crate: "qrcoderust_nif"

  def generate_svg(_data, _qr_kind, _ec_level, _fg_color, _bg_color),
    do: :erlang.nif_error(:nif_not_loaded)
end
