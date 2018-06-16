extern crate recaman_svg;

use recaman_svg::*;

pub fn main() {
    let sequence = recaman_sequence(200);
    let svg = generate_svg_document(&sequence);
    write_svg_document(svg, "image.svg");
}
