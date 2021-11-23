pub mod qr;

use qr::QrKind;
use qrcode::EcLevel;
use rustler::Atom;

mod atoms {
    rustler::atoms! {
        ok,
        // ec level
        ec_l,
        ec_m,
        ec_q,
        ec_h,
        invalid_ec_level,
        // qr kind
        square,
        circle,
        invalid_qr_kind,
    }
}

fn atom_to_ec_level(atom: Atom) -> Result<EcLevel, Atom> {
    if atom == atoms::ec_l() {
        Ok(EcLevel::L)
    } else if atom == atoms::ec_m() {
        Ok(EcLevel::M)
    } else if atom == atoms::ec_q() {
        Ok(EcLevel::Q)
    } else if atom == atoms::ec_h() {
        Ok(EcLevel::H)
    } else {
        Err(atoms::invalid_ec_level())
    }
}

fn atom_to_qr_kind(atom: Atom) -> Result<QrKind, Atom> {
    if atom == atoms::square() {
        Ok(QrKind::Square)
    } else if atom == atoms::circle() {
        Ok(QrKind::Circle)
    } else {
        Err(atoms::invalid_qr_kind())
    }
}

#[rustler::nif]
fn generate_svg(
    data: String,
    qr_kind_atom: Atom,
    ec_level_atom: Atom,
    fg_color: String,
    bg_color: String,
    include_xml_declaration: bool,
) -> Result<String, Atom> {
    let ec_level = match atom_to_ec_level(ec_level_atom) {
        Err(e) => return Err(e),
        Ok(ec) => ec,
    };

    let qr_kind = match atom_to_qr_kind(qr_kind_atom) {
        Err(e) => return Err(e),
        Ok(ec) => ec,
    };

    return Ok(qr::draw_qr(
        &data,
        qr_kind,
        ec_level,
        &fg_color,
        &bg_color,
        &0,
        include_xml_declaration,
    ));
}

rustler::init!("Elixir.QRCodeRust", [generate_svg]);
