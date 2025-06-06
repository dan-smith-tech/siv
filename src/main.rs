use clap::Parser;
use image::ImageReader;
use rand::distr::{Distribution, Uniform};
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

    let starting_centroids = get_random_centroids(pixels, 3)?;
    if debug {
        println!("Starting centroids: {:?}", starting_centroids);
    }

    Ok(())
}

fn get_random_centroids(pixels: Vec<u8>, n: u8) -> Result<Vec<[u8; 3]>, Box<dyn Error>> {
    let pixel_count = pixels.len() / 3;
    let mut rng = rand::rng();
    match Uniform::try_from(0..pixel_count) {
        Ok(dist) => Ok((0..n)
            .map(|_| {
                let pos = dist.sample(&mut rng) * 3;
                [pixels[pos], pixels[pos + 1], pixels[pos + 2]]
            })
            .collect()),
        Err(e) => Err(Box::new(e)),
    }
}
