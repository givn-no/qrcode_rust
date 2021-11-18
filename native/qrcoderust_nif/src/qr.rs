use qrcode::types::Color;
use qrcode::{EcLevel, QrCode};

#[derive(PartialEq, Eq)]
pub enum QrKind {
    Square,
    Circle,
}

fn is_within(x: usize, y: usize, top_left: (usize, usize), bottom_right: (usize, usize)) -> bool {
    return x >= top_left.0 && x <= bottom_right.0 && y >= top_left.1 && y <= bottom_right.1;
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
            // + 0.0001 to avoid gaps between squares because of rounding errors
            diameter = (radius * 2) as f32 + 0.0001,
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

fn draw_finder_patterns(code_width: usize, code_height: usize, radius: usize) -> String {
    let mut output = Vec::new();

    output.push(format!(
        r##"<use href="#f" x="{x}" y="{y}"/>"##,
        x = 0,
        y = 0
    ));

    output.push(format!(
        r##"<use href="#f" x="{x}" y="{y}"/>"##,
        x = (code_width - 7) * radius * 2,
        y = 0
    ));

    output.push(format!(
        r##"<use href="#f" x="{x}" y="{y}"/>"##,
        x = 0,
        y = (code_height - 7) * radius * 2
    ));

    return output.join("");
}

fn draw_module(col: usize, row: usize, radius: usize) -> String {
    return format!(
        r##"<use href="#m" x="{x}" y="{y}"/>"##,
        x = col * radius * 2,
        y = row * radius * 2
    );
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
    let width = code_width * radius * 2;
    let height = code_height * radius * 2;

    let mut output = Vec::new();

    if include_xml_declaration {
        output.push(format!(
        concat!(
            r#"<?xml version="1.0" standalone="yes"?>"#,
            r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
        ),
        width = width,
        height = height,
    ));
    } else {
        output.push(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
        width = width,
        height = height,
      ));
    }

    output.push(render_defs(&qr_kind, radius, fg_color, bg_color));

    // draw background
    output.push(format!(
        r#"<rect x="0" y="0" width="{width}" height="{height}" fill="{bg_color}"/>"#,
        width = width,
        height = height,
        bg_color = bg_color
    ));

    // draw custom finder pattern for circle qrs
    if qr_kind == QrKind::Circle {
        output.push(draw_finder_patterns(code_width, code_height, radius));
    }

    // draw rest of qr
    for (idx, color) in colors.iter().enumerate() {
        let col = idx % code_width;
        let row = idx / code_width;

        if color == &Color::Dark {
            // skip finder pattern for circle qr
            if qr_kind == QrKind::Circle && is_finder_pattern(code_width, code_height, col, row) {
                continue;
            }

            output.push(draw_module(col, row, radius));
        }
    }

    output.push("</svg>".to_string());
    return output.join("");
}
