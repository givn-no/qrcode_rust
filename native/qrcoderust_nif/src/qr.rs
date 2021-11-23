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

fn draw_finder_patterns(
    code_width: usize,
    code_height: usize,
    radius: usize,
    padding: usize,
) -> String {
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
        y = ((code_height - 7) * radius * 2) + padding
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

pub fn draw_qr(
    data: &str,
    qr_kind: QrKind,
    ec_level: EcLevel,
    fg_color: &str,
    bg_color: &str,
    padding: &usize,
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
        width = width + padding * 2,
        height = height + padding * 2,
    ));
    } else {
        output.push(format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" version="1.1" viewBox="0 0 {width} {height}">"#,
        width = width + padding * 2,
        height = height + padding * 2,
      ));
    }

    output.push(render_defs(&qr_kind, radius, fg_color, bg_color));

    // draw background
    output.push(format!(
        r#"<rect x="0" y="0" width="{width}" height="{height}" fill="{bg_color}"/>"#,
        width = width + padding * 2,
        height = height + padding * 2,
        bg_color = bg_color
    ));

    // draw custom finder pattern for circle qrs
    if qr_kind == QrKind::Circle {
        output.push(draw_finder_patterns(
            code_width,
            code_height,
            radius,
            *padding,
        ));
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

            output.push(draw_module(col, row, radius, *padding));
        }
    }

    output.push("</svg>".to_string());
    return output.join("");
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::io::Reader as ImageReader;
    use image::ImageFormat;
    use qrcode::EcLevel;
    use std::io::Cursor;
    use usvg::{Options, Tree};

    const IMAGE_SIZE: u32 = 300;

    fn test_qr(data: &str, qr_kind: QrKind, ec_level: EcLevel) {
        let svg = draw_qr(data, qr_kind, ec_level, "#000", "#fff", &5, true);
        let tree = Tree::from_str(&svg, &Options::default().to_ref()).unwrap();

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

        // Use default decoder
        let decoder = bardecoder::default_decoder();
        let decoded_image = decoder.decode(&image);
        let maybe_result = decoded_image.first().unwrap().as_ref();
        let result = maybe_result.unwrap();

        assert_eq!(data, result);
    }

    #[test]
    fn qr_code_is_decodable() {
        test_qr("hello world!", QrKind::Circle, EcLevel::L);
        test_qr("hello world!", QrKind::Circle, EcLevel::M);
        test_qr("hello world!", QrKind::Circle, EcLevel::Q);
        test_qr("hello world!", QrKind::Circle, EcLevel::H);
        test_qr("hello world!", QrKind::Square, EcLevel::L);
        test_qr("hello world!", QrKind::Square, EcLevel::M);
        test_qr("hello world!", QrKind::Square, EcLevel::Q);
        test_qr("hello world!", QrKind::Square, EcLevel::H);
    }
}
