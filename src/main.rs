extern crate image;
extern crate rand;
extern crate docopt;
extern crate rustc_serialize;

use std::path::Path;
use std::vec::Vec;

use rand::thread_rng;

use image::GenericImage;

use docopt::Docopt;

const USAGE: &'static str = r#"
Brightness Calculator.

Usage:
  brightness <file> [--sample=<sample>]
  brightness (-h | --help)
Options:
  --sample=<sample> Specify sample ratio [default: 100].
"#;

#[derive(Debug, RustcDecodable)]
pub struct Args {
    arg_file: Option<String>,
    flag_sample: Option<f64>,
}

#[derive(Debug)]
struct SamplePixels<'a, I: 'a> {
    image:  &'a I,
    i:      usize,
    j:      usize,
    x_points: Vec<u32>,
    y_points: Vec<u32>
}

impl<'a, I: GenericImage> SamplePixels<'a, I> {
    pub fn from_image(image: &'a I, smpl: f64) -> Self {
        let (width, height) = image.dimensions();
        let mut rng = thread_rng();

        SamplePixels {
            image: image,
            i:     0,
            j:     0,
            x_points: rand::sample(&mut rng, 0..width, ((width as f64) * smpl / 100.0) as usize),
            y_points: rand::sample(&mut rng, 0..height, ((height as f64) * smpl / 100.0) as usize)
        }
    }
}

impl<'a, I: GenericImage> Iterator for SamplePixels<'a, I> {
    type Item = (u32, u32, I::Pixel);

    fn next(&mut self) -> Option<(u32, u32, I::Pixel)> {
        if self.i >= self.x_points.len() {
            self.i =  0;
            self.j += 1;
        }

        if self.j >= self.y_points.len() {
            None
        } else {
            let x = self.x_points[self.i];
            let y = self.y_points[self.j];
            let pixel = self.image.get_pixel(x, y);
            let p = (x, y, pixel);

            self.i += 1;

            Some(p)
        }
    }
}

fn calc_gray<'a, I>(pixels: I) -> f64
    where I: Iterator<Item=(u32, u32, image::Rgba<u8>)>
{
    let mut sum: f64 = 0.0;
    let mut count = 0;

    for (_, _, pixel) in pixels {
        let red = pixel.data[0] as f64;
        let blue = pixel.data[1] as f64;
        let green = pixel.data[2] as f64;
        let gray = red * 0.30 + blue * 0.59 + green * 0.11;
        sum += gray;
        count += 1;
    }

    return sum / (count as f64);
}

fn calc_sample_gray(img: &image::DynamicImage, smpl: f64) -> f64 {
    if smpl <= 0.0 || smpl >= 100.0 {
        return calc_gray(img.pixels());
    }

    return calc_gray(SamplePixels::from_image(img, smpl));
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let filename = args.arg_file.unwrap();

    let img = image::open(&Path::new(&filename)).unwrap();

    let gray = match args.flag_sample {
        Some(s) => calc_sample_gray(&img, s),
        _ => calc_gray(img.pixels()),
    };

    println!("{}", gray.round() as u32);
}
