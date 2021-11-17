use qrcode::types::Color;
use qrcode::{EcLevel, QrCode};

fn is_within(x: usize, y: usize, top_left: (usize, usize), bottom_right: (usize, usize)) -> bool {
    return x >= top_left.0 && x <= bottom_right.0 && y >= top_left.1 && y <= bottom_right.1;
}

fn draw_square_module_path(x: usize, y: usize, r: usize) -> String {
    let diameter = r * 2;

    return format!(
        concat!("M {x}, {y}\n", "h {d} v {d} h -{d} v -{d}"),
        x = x * diameter,
        y = y * diameter,
        d = diameter
    );
}

fn draw_circle_module_path(x: usize, y: usize, r: usize) -> String {
    let diameter = r * 2;
    let cy = (y + 1) * diameter - r;

    return format!(
        concat!(
            "M {x}, {cy}\n",
            "a {r},{r} 0 1,0 {d},0\n",
            "a {r},{r} 0 1,0 -{d},0\n",
        ),
        x = x * diameter,
        d = diameter,
        cy = cy,
        r = r
    );
}

fn is_finder_pattern(width: usize, height: usize, x: usize, y: usize) -> bool {
    // top left finder pattern
    let a1 = (0, 0);
    let a2 = (6, 6);

    // top right finder patter
    let b1 = (width - 7, 0);
    let b2 = (width, 6);

    // bottom left finder pattern
    let c1 = (0, height - 7);
    let c2 = (6, height);

    return is_within(x, y, a1, a2) || is_within(x, y, b1, b2) || is_within(x, y, c1, c2);
}

fn draw_finder_pattern(
    col: usize,
    row: usize,
    radius: usize,
    fg_color: &str,
    bg_color: &str,
) -> String {
    let diameter = radius * 2;

    let mut output = Vec::new();

    output.push(format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{rx}" fill="{fill}" />"#,
        x = col * diameter,
        y = row * diameter,
        height = diameter * 7,
        width = diameter * 7,
        rx = radius,
        fill = fg_color
    ));

    output.push(format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" fill="{fill}" />"#,
        x = (col + 1) * diameter,
        y = (row + 1) * diameter,
        height = diameter * 5,
        width = diameter * 5,
        fill = bg_color
    ));

    output.push(format!(
        r#"<rect x="{x}" y="{y}" width="{width}" height="{height}" rx="{rx}" fill="{fill}" />"#,
        x = (col + 2) * diameter,
        y = (row + 2) * diameter,
        height = diameter * 3,
        width = diameter * 3,
        rx = radius,
        fill = fg_color
    ));

    return output.join("\n");
}

fn draw_finder_patterns(
    width: usize,
    height: usize,
    radius: usize,
    fg_color: &str,
    bg_color: &str,
) -> String {
    let mut output = Vec::new();
    output.push(draw_finder_pattern(0, 0, radius, fg_color, bg_color));
    output.push(draw_finder_pattern(
        width - 7,
        0,
        radius,
        fg_color,
        bg_color,
    ));
    output.push(draw_finder_pattern(
        0,
        height - 7,
        radius,
        fg_color,
        bg_color,
    ));
    return output.join("\n");
}

#[derive(PartialEq, Eq)]
pub enum QrKind {
    Square,
    Circle,
}

pub fn draw_qr(
    data: &str,
    qr_kind: QrKind,
    ec_level: EcLevel,
    fg_color: &str,
    bg_color: &str,
    include_xml_declaration: bool,
) -> String {
    let code = QrCode::with_error_correction_level(data, ec_level).unwrap();
    let colors = code.to_colors();
    let radius = 1;
    let code_width = code.width();
    let code_height = colors.len() / code.width();
    let actual_width = code_width * radius * 2;
    let actual_height = code_height * radius * 2;

    let mut output = Vec::new();

    if include_xml_declaration {
        output.push(format!(
        concat!(
            r#"<?xml version="1.0" standalone="yes"?>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
        ),
        width = actual_width,
        height = actual_height,
    ));
    } else {
        output.push(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
        width = actual_width,
        height = actual_height,
      ));
    }

    output.push(format!(
        r#"<rect x="0" y="0" width="{width}" height="{height}" fill="{bg_color}"/>"#,
        width = actual_width,
        height = actual_height,
        bg_color = bg_color
    ));

    if qr_kind == QrKind::Circle {
        // draw custom finder pattern for circle qrs
        output.push(draw_finder_patterns(
            code_width,
            code_height,
            radius,
            &fg_color,
            &bg_color,
        ));
    }

    output.push(format!(
        r#"<path fill="{fg_color}" {additional_atrs} d=""#,
        fg_color = fg_color,
        additional_atrs = match qr_kind {
            QrKind::Square => r#"shape-rendering="crispEdges""#,
            QrKind::Circle => "",
        }
    ));

    // draw rest of qr
    for (idx, color) in colors.iter().enumerate() {
        let col = idx % code_width;
        let row = idx / code_width;

        if color == &Color::Dark {
            match qr_kind {
                QrKind::Circle => {
                    // only draw finder pattern when in square mode
                    if !is_finder_pattern(code_width, code_height, col, row) {
                        output.push(draw_circle_module_path(col, row, radius))
                    }
                }

                QrKind::Square => output.push(draw_square_module_path(col, row, radius)),
            }
        }
    }

    output.push("\"/></svg>".to_string());
    return output.join("\n");
}
