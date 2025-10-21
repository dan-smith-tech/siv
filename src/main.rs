use clap::Parser;
use image::{ImageBuffer, ImageReader, RgbImage};
use rand::distr::{Distribution, Uniform};
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    file: String,

    #[arg(long, short)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let Args { file, debug: _ } = Args::parse();

    let image = ImageReader::open(file)?.decode()?;
    let pixels = image.to_rgb8().to_vec();

    let n = 3; // TODO: get cluster n from input as u8 with bounds check

    let mut centroids = get_random_centroids(&pixels, n)?;
    let mut labelled_pixels = cluster_pixels(&pixels, &centroids);

    loop {
        centroids = update_centroids(&pixels, &labelled_pixels, n);
        let new_labelled_pixels = cluster_pixels(&pixels, &centroids);

        if compare_clustered_pixels(&new_labelled_pixels, &labelled_pixels) {
            break;
        }

        labelled_pixels = new_labelled_pixels;
    }

    let reconstructed_pixels = reconstruct_image(&pixels, &labelled_pixels, &centroids);

    let img_buffer: RgbImage =
        ImageBuffer::from_raw(image.width(), image.height(), reconstructed_pixels)
            .expect("Pixel data does not match dimensions");
    img_buffer.save("output.png")?;

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

fn cluster_pixels(pixels: &Vec<u8>, centroids: &Vec<[u8; 3]>) -> Vec<u8> {
    let pixel_count = pixels.len() / 3;

    (0..pixel_count)
        .map(|i| {
            let mut closest_centroid_index: u8 = 0;
            let mut closest_distance = u32::MAX;

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

fn compare_clustered_pixels(clustered_pixels_1: &Vec<u8>, clustered_pixels_2: &Vec<u8>) -> bool {
    if clustered_pixels_1.len() != clustered_pixels_2.len() {
        return false;
    }

    clustered_pixels_1
        .iter()
        .zip(clustered_pixels_2.iter())
        .all(|(a, b)| a == b)
}

fn update_centroids(pixels: &Vec<u8>, clustered_pixels: &Vec<u8>, n: u8) -> Vec<[u8; 3]> {
    let mut centroid_sums = vec![[0; 3]; n as usize];

    (0..clustered_pixels.len())
        .map(|clustered_pixel| {
            let pixel_start = clustered_pixel * 3;
            let label = clustered_pixels[clustered_pixel] as usize;
            centroid_sums[label][0] += pixels[pixel_start] as usize;
            centroid_sums[label][1] += pixels[pixel_start + 1] as usize;
            centroid_sums[label][2] += pixels[pixel_start + 2] as usize;
        })
        .for_each(drop);

    let counts: Vec<usize> = (0..n)
        .map(|i| clustered_pixels.iter().filter(|&&label| label == i).count())
        .collect();

    (0..centroid_sums.len())
        .map(|i| {
            if counts[i] > 0 {
                centroid_sums[i][0] /= counts[i];
                centroid_sums[i][1] /= counts[i];
                centroid_sums[i][2] /= counts[i];
            }
            [
                centroid_sums[i][0] as u8,
                centroid_sums[i][1] as u8,
                centroid_sums[i][2] as u8,
            ]
        })
        .collect()
}

fn reconstruct_image(
    pixels: &Vec<u8>,
    clustered_pixels: &Vec<u8>,
    centroids: &Vec<[u8; 3]>,
) -> Vec<u8> {
    let mut reconstructed_pixels = Vec::with_capacity(pixels.len());

    clustered_pixels
        .iter()
        .map(|pixel| {
            let centroid = centroids[*pixel as usize];
            reconstructed_pixels.extend([centroid[0], centroid[1], centroid[2]]);
        })
        .for_each(drop);

    reconstructed_pixels
}
