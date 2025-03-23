import numpy as np

from image import downscale_image, generate_image
from plot import draw_graph


def cluster(pixels, labels, centroids):
    changed = 0

    for i, pos in enumerate(pixels):
        min_dist = float("inf")
        closest_centroid = -1

        for j, centroid in enumerate(centroids):
            dist = (
                np.power(pos[0] - centroid[0], 2)
                + np.power(pos[1] - centroid[1], 2)
                + np.power(pos[2] - centroid[2], 2)
            )

            if dist < min_dist:
                min_dist = dist
                closest_centroid = j

        if labels[i] != closest_centroid:
            labels[i] = closest_centroid
            changed += 1

    return labels, centroids, changed


def update_centroids(pixels, labels, centroids):
    for i in range(centroids.shape[0]):
        centroids[i] = np.mean(pixels[labels == i, :], axis=0)

    return centroids


def k_means(pixels, labels, centroids, plot):
    i = 0

    while True:
        labels, centroids, changed = cluster(pixels, labels, centroids)

        if plot:
            draw_graph(pixels, labels, centroids, f"Re-cluster points: {i}")

        if changed == 0:
            break

        centroids = update_centroids(pixels, labels, centroids)

        if plot:
            draw_graph(pixels, labels, centroids, f"Re-calculate centroids: {i}")

        i += 1


def filter_image(image_input, image_output, k, plot=False):
    img = downscale_image(image_input)
    pixels = np.asarray(img.getdata())

    def rgb_to_hex(rgb):
        return "#{:02X}{:02X}{:02X}".format(*rgb)

    hex_pixels = np.array([rgb_to_hex(pixel) for pixel in pixels])

    centroids = np.empty([k, 3])
    labels = np.empty(pixels.shape[0])

    rand_indices = np.random.choice(pixels.shape[0], k, replace=False)
    for centroid_i, pixel_i in enumerate(rand_indices):
        centroids[centroid_i] = pixels[pixel_i]

    k_means(pixels, labels, centroids, plot)

    generate_image(img, image_output, labels, centroids)
