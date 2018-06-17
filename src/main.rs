extern crate clap;
extern crate recaman_svg;

use clap::{App, Arg};
use recaman_svg::*;

pub fn main() {
    let matches = App::new("Recamán SVG generator")
        .version("1.0")
        .author("Joren Van Onder <joren.vanonder@gmail.com>")
        .about("Generates an SVG image based on a Recamán sequence.")
        .arg(
            Arg::with_name("amount")
                .short("n")
                .long("amount")
                .default_value("20")
                .help("Amount of Recamán points to generate.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Write SVG to OUTPUT.")
                .required(true),
        )
        .get_matches();

    let amount: u32 = matches
        .value_of("amount")
        .unwrap()
        .parse()
        .expect("Not a valid integer.");
    let file = matches.value_of("OUTPUT").unwrap();

    let sequence = recaman_sequence(amount);
    let svg = generate_svg_document(&sequence);
    write_svg_document(svg, file);
}
