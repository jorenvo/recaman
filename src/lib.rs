extern crate svg;

use std::process;
use std::collections::HashSet;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub fn recaman_sequence(n: u32) -> Vec<u32> {
    if n == 0 {
        return vec![];
    }

    let mut used_numbers = HashSet::new();
    let mut seq: Vec<u32> = vec![0];
    while seq.len() < n as usize {
        let seq_len = seq.len() as i32;
        let last = *seq.last().unwrap() as i32;
        let subtract_result = last - seq_len;

        let a: i32;
        if subtract_result > 0 && !used_numbers.contains(&(subtract_result as u32)) {
            a = subtract_result;
        } else {
            a = last + seq_len;
        }

        seq.push(a as u32);
        used_numbers.insert(a as u32);
    }

    seq
}

/// Calculates the line height that will center the image.
///
/// Every n will jump one position farther than the last
/// one. Therefore every arc will have a radius exactly 1 bigger than
/// the last one. Because unit arcs (radius=1) are used the required
/// line height at which we have to draw is 0.5 * n.
fn get_line_height(recaman_sequence: &Vec<u32>) -> f32 {
    0.5 * recaman_sequence.len() as f32
}

pub fn generate_svg_document(recaman_sequence: &Vec<u32>, stroke_width: f32) -> svg::Document {
    const ARC_LARGE_FLAG: u32 = 0;
    const ARC_ANGLE: u32 = 0;
    const ARC_RX: u32 = 1;
    const ARC_RY: u32 = 1;
    const X_SCALE: u32 = 2;

    let scale = |x: f32| x * X_SCALE as f32;
    let line_height = scale(get_line_height(recaman_sequence));

    let mut data = Data::new().move_to((0, line_height));
    let mut should_go_up = false;
    let mut last_x = 0;
    let mut max_x = u32::min_value();
    for x in recaman_sequence {
        let x = *x;
        let sweep_flag = if x > last_x {
            !should_go_up
        } else {
            // invert if going backwards
            should_go_up
        };

        data = data.elliptical_arc_to((
            ARC_RX,
            ARC_RY,
            ARC_ANGLE,
            ARC_LARGE_FLAG,
            sweep_flag as u32,
            scale(x as f32),
            line_height,
        ));

        last_x = x;
        should_go_up = !should_go_up;
        max_x = max_x.max(x);
    }

    const MARGIN: f32 = 2.0;
    let document = Document::new().set(
        "viewBox",
        (
            scale(0.0 - MARGIN),
            scale(0.0 - MARGIN),
            scale(max_x as f32 + MARGIN * 2.0),
            line_height * 2.0 + scale(MARGIN * 2.0),
        ),
    );
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", stroke_width)
        .set("d", data);

    document.add(path)
}

pub fn write_svg_document(document: svg::Document, filename: &str) {
    svg::save(filename, &document).unwrap_or_else(|err| {
        eprintln!("Couldn't write file: {}", err.to_string().to_lowercase());
        process::exit(1);
    });
}
