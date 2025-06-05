use clap::Parser;
use image::ImageReader;
use rand::Rng;
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    file: String,

    #[arg(long, short)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { file, debug } = Args::parse();

    let image = ImageReader::open(file)?.decode()?;
    // note this is a 1D array (so there are pixels.len() / 3 actual RGB pixels)
    let pixels = image.to_rgb8().to_vec();

    let starting_centroids = get_random_centroids(pixels.len() as u32 / 3, 3);
    if debug {
        println!("{:?}", starting_centroids);
    }

    Ok(())
}

fn get_random_centroids(pixel_count: u32, n: u8) -> Vec<[u8; 3]> {
    let mut vec = Vec::new();
    let mut rng = rand::rng();

    for _ in 0..n {
        let pos = rng.random_range(0..pixel_count);
        let r = (pos * 3) as u8;
        let g = (pos * 3 + 1) as u8;
        let b = (pos * 3 + 2) as u8;
        vec.push([r, g, b]);
    }

    vec
}
