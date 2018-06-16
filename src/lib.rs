#![allow(dead_code)]

extern crate svg;

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

#[derive(Default, Clone)]
struct Point {
    x: u32,
    height: u32,
    has_line_under: bool,
    has_line_over: bool,
}

pub fn generate_svg_document(recaman_sequence: &Vec<u32>) -> svg::Document {
    const LINE_HEIGHT: u32 = 50;
    let max_x = *recaman_sequence.iter().max().unwrap();

    const X_SCALE: u32 = 5;
    let scale = |x: u32| x * X_SCALE;

    let mut data = Data::new().move_to((0, LINE_HEIGHT));
    for el in recaman_sequence {
        let el = *el;
        let rx = 1;
        let ry = 1;
        let angle = 0;
        let large_arc_flag = 0;
        let sweep_flag = 0;
        data = data.elliptical_arc_to((
            rx,
            ry,
            angle,
            large_arc_flag,
            sweep_flag,
            scale(el),
            LINE_HEIGHT,
        ));
    }

    let document = Document::new().set("viewBox", (0, 0, scale(max_x), 100));
    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 1)
        .set("d", data);

    document.add(path)
}

pub fn write_svg_document(document: svg::Document, filename: &str) {
    svg::save(filename, &document).unwrap();
}
