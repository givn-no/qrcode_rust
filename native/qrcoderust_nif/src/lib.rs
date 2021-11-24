pub mod qr;

use qr::QrKind;
use qrcode::types::QrError;
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

        // qr encoder errors
        data_too_long,
        invalid_version,
        unsupported_character_set,
        invalid_eci_designator,
        invalid_character
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

fn qr_error_to_atom(e: QrError) -> Atom {
    return match e {
        QrError::DataTooLong => atoms::data_too_long(),
        QrError::InvalidVersion => atoms::invalid_version(),
        QrError::UnsupportedCharacterSet => atoms::unsupported_character_set(),
        QrError::InvalidEciDesignator => atoms::invalid_eci_designator(),
        QrError::InvalidCharacter => atoms::invalid_character(),
    };
}

#[rustler::nif]
fn generate_svg(
    data: String,
    qr_kind_atom: Atom,
    ec_level_atom: Atom,
    fg_color: String,
    bg_color: String,
    remove_middle: bool,
    include_xml_declaration: bool,
    min_version: Option<usize>,
) -> Result<qr::Qr, Atom> {
    let ec_level = match atom_to_ec_level(ec_level_atom) {
        Err(e) => return Err(e),
        Ok(ec) => ec,
    };

    let qr_kind = match atom_to_qr_kind(qr_kind_atom) {
        Err(e) => return Err(e),
        Ok(ec) => ec,
    };

    let result = qr::draw_qr(
        &data,
        qr_kind,
        ec_level,
        &fg_color,
        &bg_color,
        &0,
        remove_middle,
        include_xml_declaration,
        min_version,
    );

    return match result {
        Ok(qr) => Ok(qr),
        Err(e) => Err(qr_error_to_atom(e)),
    };
}

rustler::init!("Elixir.QRCodeRust", [generate_svg]);
