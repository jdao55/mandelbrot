use num::Complex;

use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(filename: &str, pixels: &[u8], x: u32, y: u32) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, x, y, ColorType::Gray(8))?;

    Ok(())
}
fn complex_point(x: u32, y: u32, R: f32, dim: (u32, u32)) -> Complex<f32> {
    Complex {
        re: (x as f32) / (dim.0 as f32) * 2.0 - 1.0,
        im: (y as f32) / (dim.1 as f32) * 2.0 * R - R,
    }
}
fn value(x: u32, y: u32, max_x: u32, max_y: u32, R: f32) -> u8 {
    let c = Complex {
        re: 0.285,
        im: 0.01,
    };
    let mut z = complex_point(x, y, R, (max_x, max_y)) * 1.5;

    for i in 0..64 {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return 255 - ((255.0 * (i as f32 / 64.0)) as u8);
        }
    }
    0
}

use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0")]
struct Args {
    file: String,
    x: u32,
    y: u32,
}

fn main() {
    let args: Args = Args::parse();

    let mut pixels = vec![0; (args.x * args.y) as usize];

    let mut n = 0;
    let r = args.y as f32 / args.x as f32;
    for y in 0..args.y {
        for x in 0..args.x {
            pixels[n] = value(x, y, args.x, args.y, r);
            n = n + 1;
        }
    }
    write_image(&args.file, &pixels, args.x, args.y).expect("error writing PNG file");
}
