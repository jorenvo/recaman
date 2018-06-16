extern crate svg;

use std::collections::HashSet;

use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
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

pub fn generate_svg_document(recaman_sequence: &Vec<u32>) -> svg::Document {
    const ARC_LARGE_FLAG: u32 = 0;
    const ARC_ANGLE: u32 = 0;
    const ARC_RX: u32 = 1;
    const ARC_RY: u32 = 1;

    const LINE_HEIGHT: u32 = 50;
    const X_SCALE: u32 = 5;
    let scale = |x| x * X_SCALE;

    let mut data = Data::new().move_to((0, LINE_HEIGHT));
    let mut should_go_up = false;
    let mut last_x = 0;
    let mut max_x = u32::min_value();
    for x in recaman_sequence {
        let x = *x;
        let sweep_flag = if x > last_x {
            !should_go_up
        } else { // invert if going backwards
            should_go_up
        };

        data = data.elliptical_arc_to((
            ARC_RX,
            ARC_RY,
            ARC_ANGLE,
            ARC_LARGE_FLAG,
            sweep_flag as u32,
            scale(x),
            LINE_HEIGHT,
        ));

        last_x = x;
        should_go_up = !should_go_up;
        max_x = max_x.max(x);
    }

    let document = Document::new().set("viewBox", (0, 0, scale(max_x), 100));
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);
    let group = Group::new().set("transform", "rotate(0)").add(path);

    document.add(group)
}

pub fn write_svg_document(document: svg::Document, filename: &str) {
    svg::save(filename, &document).unwrap();
}
