extern crate image;

use std::env;
use std::path::Path;
use image::GenericImage;

fn main() {
    let filename = env::args().nth(1).unwrap();
    let img = image::open(&Path::new(&filename)).unwrap();

    let mut sum: f64 = 0.0;
    for (_, _, pixel) in img.pixels() {
        let red = pixel.data[0] as f64;
        let blue = pixel.data[1] as f64;
        let green = pixel.data[2] as f64;
        let gray = red * 0.30 + blue * 0.59 + green * 0.11;
        sum += gray;
    }

    let (width, height) = img.dimensions();
    println!("{:?}", sum / (width as f64) / (height as f64));
}
