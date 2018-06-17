extern crate clap;
extern crate recaman_svg;

use clap::{App, Arg, ArgMatches};
use recaman_svg::*;
use std::process;

fn parse_config_number(matches: &ArgMatches, arg_name: &str) -> f32 {
    matches
        .value_of(arg_name)
        .unwrap()
        .parse()
        .unwrap_or_else(|_err| {
            eprintln!("Not a valid {}.", arg_name);
            process::exit(1);
        })
}

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
            Arg::with_name("stroke width")
                .short("-w")
                .long("stroke_width")
                .default_value("1.0")
                .help("Stroke width of the line in the image.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("OUTPUT")
                .help("Write SVG to OUTPUT.")
                .required(true),
        )
        .get_matches();

    let amount: u32 = parse_config_number(&matches, "amount") as u32;
    let stroke_width: f32 = parse_config_number(&matches, "stroke width");
    let file = matches.value_of("OUTPUT").unwrap();

    let sequence = recaman_sequence(amount);
    let svg = generate_svg_document(&sequence, stroke_width);
    write_svg_document(svg, file);
}
