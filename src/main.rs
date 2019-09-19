use num::Complex;
use rayon::prelude::*;

use std::str::FromStr;

//TODO add rayon
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}



use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(filename: &str, pixels: &[u8],
               bounds: (usize, usize))
    -> Result<(), std::io::Error>
{
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
                   bounds.0 as u32, bounds.1 as u32,
                   ColorType::Gray(8))?;

    Ok(())
}

fn value(x: u32, y: u32, bounds: (usize, usize) ) -> u8 {
    let x = x as f32;
    let y = y as f32;
    let a = bounds.0 as f32;
    let b = bounds.1 as f32;
    let point = Complex { re: x/a - 1.5, im: y/b -0.5 };
    let mut z = Complex {re: 0 as f32, im: 0 as f32};

    for i in 0..255 {
        z = z * z + point;
        if z.norm_sqr() > 4.0 {
            return i as u8;
        }
    }
    0


}
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        writeln!(std::io::stderr(),
                 "Usage: mandelbrot filename dimensions(1200x800) ")
            .unwrap();
        std::process::exit(1);
    }

    let bounds: (usize, usize) = parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    let mut n = 0;
    let nox = bounds.0 as u32;
    let noy =  bounds.1 as u32;
    for x in 0..nox{
        for y in 0..noy{
            pixels[n]=value(x, y, bounds);
            n = n + 1;
        }
    }

    write_image(&args[1], &pixels, bounds).expect("error writing PNG file");
}
