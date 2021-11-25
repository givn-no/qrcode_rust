use qrcode::types::Color;
use qrcode::types::{QrError, Version};
use qrcode::{EcLevel, QrCode};
use std::cmp;

#[derive(Debug)]
struct Square {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

#[derive(PartialEq, Eq)]
pub enum QrKind {
    Square,
    Circle,
}

#[derive(rustler::NifStruct)]
#[module = "QrCodeRust.Qr"]
#[derive(Debug)]
pub struct Qr {
    pub svg: String,
    pub version: i16,
    pub module_size: usize,
    // units are in terms of module size
    pub padding: usize,
    pub width: usize,
    pub center_top_left_x: Option<usize>,
    pub center_top_left_y: Option<usize>,
    pub center_width: Option<usize>,
}

fn is_within(x: usize, y: usize, square: &Square) -> bool {
    return x >= square.x1 && x <= square.x2 && y >= square.y1 && y <= square.y2;
}

fn is_finder_pattern(code_width: usize, x: usize, y: usize) -> bool {
    // top left finder pattern
    let top_left_finder = Square {
        x1: 0,
        x2: 0,
        y1: 6,
        y2: 6,
    };

    // top right finder patter
    let top_right_finder = Square {
        x1: code_width - 7,
        x2: code_width,
        y1: 0,
        y2: 6,
    };

    // bottom left finder pattern
    let bottom_left_finder = Square {
        x1: 0,
        x2: 6,
        y1: code_width - 7,
        y2: code_width,
    };

    return is_within(x, y, &top_left_finder)
        || is_within(x, y, &top_right_finder)
        || is_within(x, y, &bottom_left_finder);
}

fn draw_finder_pattern_group(radius: usize, fg_color: &str, bg_color: &str) -> String {
    let diameter = radius * 2;

    let mut output = Vec::new();

    output.push(format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{rx}" fill="{fill}"/>"#,
        x = 0,
        y = 0,
        height = diameter * 7,
        width = diameter * 7,
        rx = radius,
        fill = fg_color
    ));

    output.push(format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" fill="{fill}"/>"#,
        x = 1 * diameter,
        y = 1 * diameter,
        height = diameter * 5,
        width = diameter * 5,
        fill = bg_color
    ));

    output.push(format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{rx}" fill="{fill}"/>"#,
        x = 2 * diameter,
        y = 2 * diameter,
        height = diameter * 3,
        width = diameter * 3,
        rx = radius,
        fill = fg_color
    ));

    return format!(r##"<g id="f">{content}</g>"##, content = output.join(""));
}

fn draw_module_group(qr_kind: &QrKind, radius: usize, fg_color: &str) -> String {
    return match qr_kind {
        QrKind::Circle => format!(
            r#"<g id="m"><circle cx="{radius}" cy="{radius}" r="{radius}" fill="{fg_color}"/></g>"#,
            radius = radius,
            fg_color = fg_color
        ),
        QrKind::Square => format!(
            r#"<g id="m"><rect x="0" y="0" width="{diameter}" height="{diameter}" fill="{fg_color}" shape-rendering="crispEdges"/></g>"#,
            // + 0.1 to avoid gaps between squares because of rounding errors
            diameter = (radius * 2) as f32 + 0.1,
            fg_color = fg_color
        ),
    };
}

fn render_defs(qr_kind: &QrKind, radius: usize, fg_color: &str, bg_color: &str) -> String {
    let mut output = Vec::new();

    output.push(r#"<defs>"#.to_string());

    if qr_kind == &QrKind::Circle {
        output.push(draw_finder_pattern_group(radius, fg_color, bg_color));
    }

    output.push(draw_module_group(&qr_kind, radius, fg_color));
    output.push(r#"</defs>"#.to_string());

    return output.join("");
}

fn draw_finder_patterns(code_width: usize, radius: usize, padding: usize) -> String {
    let mut output = Vec::new();

    output.push(format!(
        r##"<use href="#f" x="{x}" y="{y}"/>"##,
        x = padding,
        y = padding
    ));

    output.push(format!(
        r##"<use href="#f" x="{x}" y="{y}"/>"##,
        x = ((code_width - 7) * radius * 2) + padding,
        y = padding
    ));

    output.push(format!(
        r##"<use href="#f" x="{x}" y="{y}"/>"##,
        x = padding,
        y = ((code_width - 7) * radius * 2) + padding
    ));

    return output.join("");
}

fn draw_module(col: usize, row: usize, radius: usize, padding: usize) -> String {
    return format!(
        r##"<use href="#m" x="{x}" y="{y}"/>"##,
        x = (col * radius * 2) + padding,
        y = (row * radius * 2) + padding
    );
}

fn usize_to_version(v: usize) -> Version {
    return Version::Normal(cmp::max(cmp::min(v as i16, 40), 1));
}

fn create_code(
    data: &str,
    min_version: Option<usize>,
    ec_level: EcLevel,
) -> Result<QrCode, QrError> {
    return match min_version {
        Some(v) => match QrCode::with_version(data, usize_to_version(v), ec_level) {
            Ok(code) => Ok(code),
            Err(e) => match e {
                QrError::DataTooLong => create_code(data, None, ec_level),
                QrError::InvalidVersion => create_code(data, None, ec_level),
                _ => Err(e),
            },
        },
        None => QrCode::with_error_correction_level(data, ec_level),
    };
}

fn get_version_number(version: Version) -> i16 {
    return match version {
        Version::Normal(v) => v,
        Version::Micro(v) => v,
    };
}

fn find_safe_center_cutout(code: &QrCode) -> Square {
    let width = code.width();
    let allowed_error_pct = match code.error_correction_level() {
        EcLevel::L => 0.07,
        EcLevel::M => 0.15,
        EcLevel::Q => 0.25,
        EcLevel::H => 0.3,
    };
    let allowed_error_square_width = width as f64 * allowed_error_pct;
    let center_width = allowed_error_square_width.floor() as usize;
    let center = width / 2;

    return Square {
        x1: center - center_width / 2,
        y1: center - center_width / 2,
        x2: center + center_width / 2,
        y2: center + center_width / 2,
    };
}

pub fn draw_qr(
    data: &str,
    qr_kind: QrKind,
    ec_level: EcLevel,
    fg_color: &str,
    bg_color: &str,
    padding: &usize,
    remove_middle: bool,
    include_xml_declaration: bool,
    min_version: Option<usize>,
) -> Result<Qr, QrError> {
    let code = create_code(data, min_version, ec_level)?;
    let radius = 1;
    let diameter = radius * 2;
    let code_width = code.width();

    // padding & width in svg units
    let svg_padding = diameter * padding;
    let svg_width = code_width * diameter;

    let mut output = Vec::new();

    if include_xml_declaration {
        output.push(format!(
            concat!(
                r#"<?xml version="1.0" standalone="yes"?>"#,
                r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
            ),
            width = svg_width + svg_padding,
            height = svg_width + svg_padding,
        ));
    } else {
        output.push(format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
            width = svg_width + svg_padding,
            height = svg_width + svg_padding,
      ));
    }

    output.push(render_defs(&qr_kind, radius, fg_color, bg_color));

    // draw background
    output.push(format!(
        r#"<rect x="0" y="0" width="{width}" height="{height}" fill="{bg_color}"/>"#,
        width = svg_width + svg_padding,
        height = svg_width + svg_padding,
        bg_color = bg_color
    ));

    // draw custom finder pattern for circle qrs
    if qr_kind == QrKind::Circle {
        output.push(draw_finder_patterns(code_width, radius, *padding));
    }

    let max_cutout_square = find_safe_center_cutout(&code);

    // draw rest of qr
    for (idx, color) in code.to_colors().iter().enumerate() {
        let col = idx % code_width;
        let row = idx / code_width;

        if color == &Color::Dark {
            // skip finder pattern for circle qr
            if qr_kind == QrKind::Circle && is_finder_pattern(code_width, col, row) {
                continue;
            }

            // remove middle squares
            if remove_middle && is_within(col, row, &max_cutout_square) {
                continue;
            }

            output.push(draw_module(col, row, radius, *padding));
        }
    }

    output.push("</svg>".to_string());
    let svg = output.join("");

    let qr = match remove_middle {
        true => Qr {
            svg: svg,
            version: get_version_number(code.version()),
            padding: svg_padding,
            module_size: diameter,
            width: code_width,
            center_top_left_x: Some(max_cutout_square.x1),
            center_top_left_y: Some(max_cutout_square.y1),
            center_width: Some((max_cutout_square.x2 - max_cutout_square.x1) + 1),
        },
        false => Qr {
            svg: svg,
            version: get_version_number(code.version()),
            padding: svg_padding,
            module_size: diameter,
            width: code_width,
            center_top_left_x: None,
            center_top_left_y: None,
            center_width: None,
        },
    };

    return Ok(qr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::io::Reader as ImageReader;
    use image::ImageFormat;
    use qrcode::EcLevel;
    use std::io::Cursor;
    use usvg::{Options, Tree};

    const IMAGE_SIZE: u32 = 512;
    const BASE_STRING: &str = "hello world! hello world!";

    fn test_qr(data: &str, qr_kind: QrKind, ec_level: EcLevel, remove_middle: bool) {
        let include_xml_declaration = true;
        let padding = 5;
        let qr = draw_qr(
            data,
            qr_kind,
            ec_level,
            "#000",
            "#fff",
            &padding,
            remove_middle,
            include_xml_declaration,
            None,
        )
        .unwrap();
        let tree = Tree::from_str(&qr.svg, &Options::default().to_ref()).unwrap();

        // render image
        let fit_to = usvg::FitTo::Width(IMAGE_SIZE);
        let size = fit_to
            .fit_to(tree.svg_node().size.to_screen_size())
            .unwrap();
        let mut pixmap = tiny_skia::Pixmap::new(size.width(), size.height()).unwrap();
        resvg::render(&tree, fit_to, pixmap.as_mut()).unwrap();
        let png_bytes = pixmap.encode_png().unwrap();
        let image = ImageReader::with_format(Cursor::new(png_bytes), ImageFormat::Png)
            .decode()
            .unwrap();
        let image_gray = image.into_luma8();

        // Use default decoder
        let mut decoder = quircs::Quirc::default();
        let codes = decoder.identify(
            image_gray.width() as usize,
            image_gray.height() as usize,
            &image_gray,
        );

        for code in codes {
            let code = code.expect("failed to extract qr code");
            let decode_result = code.decode();

            if decode_result.is_err() {
                let _result = image_gray.save("failed.png").unwrap();
            }

            let decoded = decode_result.expect("failed to decode qr code");
            let payload = std::str::from_utf8(&decoded.payload).unwrap();
            assert_eq!(data, payload);
        }
    }

    #[test]
    fn qr_code_is_decodable() {
        let mut qr_data = BASE_STRING.to_string();

        // XXX: For some reason circle qr scans fine on e.g. phones, but quirc is unable to scan it
        //      so tests for circle qrs are disabled for now...
        for _ in 1..25 {
            // test_qr(&qr_data, QrKind::Circle, EcLevel::L, false);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::M, false);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::Q, false);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::H, false);
            test_qr(&qr_data, QrKind::Square, EcLevel::L, false);
            test_qr(&qr_data, QrKind::Square, EcLevel::M, false);
            test_qr(&qr_data, QrKind::Square, EcLevel::Q, false);
            test_qr(&qr_data, QrKind::Square, EcLevel::H, false);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::L, true);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::M, true);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::Q, true);
            // test_qr(&qr_data, QrKind::Circle, EcLevel::H, true);
            test_qr(&qr_data, QrKind::Square, EcLevel::L, true);
            test_qr(&qr_data, QrKind::Square, EcLevel::M, true);
            test_qr(&qr_data, QrKind::Square, EcLevel::Q, true);
            test_qr(&qr_data, QrKind::Square, EcLevel::H, true);
            qr_data = format!("{} {}", qr_data, BASE_STRING.to_string());
        }
    }
}
