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

    // TODO: get cluster n from input but as a u8 and ensure it is below u8::MAX

    let starting_centroids = get_random_centroids(&pixels, 3)?;
    if debug {
        println!("Starting centroids: {:?}", starting_centroids);
    }

    let clustered_pixels = cluster_pixels(&pixels, starting_centroids);
    if debug {
        println!("Clustered pixels: {:?}", clustered_pixels);
    }

    Ok(())
}

fn get_random_centroids(pixels: &Vec<u8>, n: u8) -> Result<Vec<[u8; 3]>, Box<dyn Error>> {
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

fn cluster_pixels(pixels: &Vec<u8>, centroids: Vec<[u8; 3]>) -> Vec<u8> {
    let pixel_count = pixels.len() / 3;

    let mut closest_centroid_index: u8 = 0;
    let mut closest_distance = u32::MAX;

    (0..pixel_count)
        .map(|i| {
            (0..centroids.len())
                .map(|j| {
                    let r_pixel = pixels[i * 3];
                    let g_pixel = pixels[i * 3 + 1];
                    let b_pixel = pixels[i * 3 + 2];

                    let r_centroid = centroids[j][0];
                    let g_centroid = centroids[j][1];
                    let b_centroid = centroids[j][2];

                    let r_distance = (r_pixel as i32 - r_centroid as i32).pow(2);
                    let g_distance = (g_pixel as i32 - g_centroid as i32).pow(2);
                    let b_distance = (b_pixel as i32 - b_centroid as i32).pow(2);

                    let distance = (r_distance + g_distance + b_distance) as u32;

                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_centroid_index = j as u8;
                    }
                })
                .for_each(drop);

            closest_centroid_index
        })
        .collect()
}
