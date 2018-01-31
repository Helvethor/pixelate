extern crate clap;
extern crate image;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::process;

use clap::{App, Arg};


fn main() {
    let matches = App::new("Pixelate")
        .version("0.1")
        .author("Vincent Pasquier")
        .about("Quickly pixelate an image")
        .arg(Arg::with_name("factor")
            .help("Scale factor")
            .short("f")
            .long("factor")
            .value_name("FACTOR")
            .takes_value(true))
        .arg(Arg::with_name("input")
            .help("Input file (default stdin)")
            .short("i")
            .long("input")
            .value_name("INPUT")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .help("Output file (default stdout)")
            .short("o")
            .long("output")
            .value_name("OUTPUT")
            .takes_value(true))
        .get_matches();

    let factor = match matches.value_of("factor") {
        Some(f) => match f.parse() {
            Ok(f) => f,
            Err(e) => {
                eprint!("Invalid factor {}: {}\n", f, e);
                process::exit(1);
            }
        },
        None => 4
    };

    let mut input = Vec::new();
    match matches.value_of("input") {
        Some(filename) => match fs::File::open(filename) {
            Ok(mut file) => match file.read_to_end(&mut input) {
                Ok(_) => (),
                Err(e)  => {
                    eprint!("Couldn't read input file {}: {}\n", filename, e);
                    process::exit(1);
                }
            },
            Err(e) => {
                eprint!("Couldn't open input file {}: {}\n", filename, e);
                process::exit(1);
            }
        }
        None => match io::stdin().read_to_end(&mut input) {
            Ok(_) => (),
            Err(e) => {
                eprint!("Couldn't read stdin: {}\n", e);
                process::exit(1);
            }
        }
    };

    let mut image = match image::load_from_memory(&input) {
        Ok(i) => i.to_rgb(),
        Err(e) => {
            eprint!("Couldn't read image: {}\n", e);
            process::exit(1);
        }
    };

    let (width, height) = image.dimensions();
    let mut x_steps = width / factor;
    if width % factor != 0 {
        x_steps += 1;
    }

    let mut y_steps = height / factor;
    if height % factor != 0 {
        y_steps += 1;
    }

    let surface = factor * factor;

    for i in 0..x_steps {
        for j in 0..y_steps {
                
            let mut size = 0;
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            for k in 0..surface {
                let x = i * factor + k % factor;
                let y = j * factor + k / factor;
                if x >= width || y >= height {
                    continue;
                }

                let pixel = image.get_pixel(x, y);
                size += 1;
                red += pixel[0] as u32;
                green += pixel[1] as u32;
                blue += pixel[2] as u32;
            }

            let red = (red / size) as u8;
            let green = (green / size) as u8;
            let blue = (blue / size) as u8;
            let pixel = image::Rgb { 
                data: [red, green, blue]
            };

            for k in 0..surface {
                let x = i * factor + k % factor;
                let y = j * factor + k / factor;
                if x >= width || y >= height {
                    continue;
                }
                image.put_pixel(x, y, pixel);
            }
        }
    }

    //let image = image.to

    match matches.value_of("output") {
        Some(filename) => match image.save(filename) {
            Ok(_) => (),
            Err(e) => {
                eprint!("Couldn't write output file {}: {}\n", filename, e);
                process::exit(1);
            }
        },
        None => {
            let encoder = image::png::PNGEncoder::new(io::stdout());
            match encoder.encode(&image.into_vec(), width, height, image::ColorType::RGB(8)) {
                Ok(_) => (),
                Err(e) => {
                    eprint!("Couldn't write to stdout: {}\n", e);
                    process::exit(1);
                }
            };
        }
    };

}
