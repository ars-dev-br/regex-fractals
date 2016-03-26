extern crate getopts;
extern crate image;
extern crate regex;
extern crate rustc_serialize;

use getopts::{
    Matches,
    Options,
};
use image::{
    Rgb,
    RgbImage,
};
use regex::Regex;
use rustc_serialize::hex::FromHex;
use std::env;
use std::path::Path;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const MATCH_COLOR: &'static str = "#ffffff";
const OFF_COLOR: &'static str = "#222222";
const ON_COLOR: &'static str = "#ffffff";
const OUTPUT: &'static str = "output.png";
const SIZE: u32 = 2048;

/// The options used when creating an image.
struct ImageOptions {
    match_color: [u8; 3],
    off_color: [u8; 3],
    on_color: [u8; 3],
    regex: String,
    size: u32,
}

/// Create and save the image using the options from the command line.
fn do_work(matches: Matches) {
    let match_color = match matches.opt_str("match-color") {
        Some(s) => s,
        None => String::from(MATCH_COLOR)
    };

    let off_color = match matches.opt_str("off-color") {
        Some(s) => s,
        None => String::from(OFF_COLOR)
    };

    let on_color = match matches.opt_str("on-color") {
        Some(s) => s,
        None => String::from(ON_COLOR)
    };

    let output = match matches.opt_str("o") {
        Some(s) => s,
        None => String::from(OUTPUT)
    };

    let size = match matches.opt_str("s") {
        Some(s) => s.parse::<u32>().unwrap(),
        None => SIZE
    };

    let image_options = ImageOptions {
        match_color: to_color(match_color),
        off_color: to_color(off_color),
        on_color: to_color(on_color),
        regex: matches.free[0].clone(),
        size: size
    };

    let image = create_image(image_options);
    let _ = image.save(&Path::new(&output));
}

/// Create the image with the given options.
fn create_image(options: ImageOptions) -> RgbImage {
    let regex = Regex::new(options.regex.as_str()).unwrap();

    RgbImage::from_fn(options.size, options.size, |x, y| {
        let pixel_id = pixel_string(x, y, options.size);

        if regex.is_match(pixel_id.as_str()) {
            let captures = regex.captures(pixel_id.as_str()).unwrap();
            let t = match captures.at(1) {
                Some(c) => c.len() as f64 / pixel_id.len() as f64,
                None => 0 as f64
            };

            Rgb(color_lerp(t, options.on_color, options.match_color))
        } else {
            Rgb(options.off_color)
        }
    })
}

fn color_lerp(t: f64, fst_color: [u8; 3], snd_color: [u8; 3]) -> [u8; 3] {
    [u8_lerp(t, fst_color[0], snd_color[0]),
     u8_lerp(t, fst_color[1], snd_color[1]),
     u8_lerp(t, fst_color[2], snd_color[2])]
}

fn u8_lerp(t: f64, x: u8, y: u8) -> u8 {
    let fx = x as f64;
    let fy = y as f64;

    (fx + t * (fy - fx)) as u8
}

/// Convert a pixel position into a string as "1121324"
fn pixel_string(x: u32, y: u32, size: u32) -> String {
    let mut cx = x;
    let mut cy = y;

    let mut half = size / 2;
    let mut id: String = String::new();

    let sqrt = (size as f64).sqrt() as usize + 1;
    id.reserve(sqrt);

    while half > 1 {
        if cx >= half && cy < half {
            id.push('1');
            cx -= half;
        } else if cx < half && cy < half {
            id.push('2');
        } else if cx < half && cy >= half {
            id.push('3');
            cy -= half;
        } else {
            id.push('4');
            cx -= half;
            cy -= half;
        }

        half = half / 2;
    }

    id
}

/// Print the program usage.
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options] REGEX", program);
    print!("{}", opts.usage(&brief));
}

/// Print the program version.
fn print_version(program: &str) {
    println!("{} v{}", program, VERSION);
}

/// Convert a string such as "#f0f0f0" or "99cc33" to a u8 array.
fn to_color(text: String) -> [u8; 3] {
    let base = text.len() - 6;
    let hex = text[base..base+6].from_hex().unwrap();

    [hex[0], hex[1], hex[2]]
}

/// Set up the options and check if usage is correct.
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt("", "on-color", "set color for pixels with matches", "COLOR");
    opts.optopt("", "off-color", "set color for pixels without matches", "COLOR");
    opts.optopt("", "match-color", "set color for pixels colored by match size", "COLOR");
    opts.optopt("o", "output", "set output filename", "FILE");
    opts.optopt("s", "size", "set output image size", "SIZE");
    opts.optflag("h", "help", "print this help menu and quit");
    opts.optflag("v", "version", "print this program version and quit");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    if matches.opt_present("v") {
        print_version(&program);
        return;
    }

    if !matches.free.is_empty() {
        do_work(matches);
    } else {
        print_usage(&program, opts);
        return;
    };
}

