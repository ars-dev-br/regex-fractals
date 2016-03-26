extern crate getopts;

use getopts::Options;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} REGEX [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt("", "on-color", "set color for pixels with matches", "COLOR");
    opts.optopt("", "off-color", "set color for pixels without matches", "COLOR");
    opts.optopt("o", "output", "set output filename", "FILE");
    opts.optopt("s", "size", "set output image size", "SIZE");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = matches.opt_str("o");

    let regex = if !matches.free.is_empty() {
        matches.free[0].clone();
    } else {
        print_usage(&program, opts);
        return;
    };
}

