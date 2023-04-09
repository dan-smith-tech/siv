import numpy as np

from filter.image import *
from filter.plot import *


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
    while True:
        labels, centroids, changed = cluster(pixels, labels, centroids)

        if plot:
            print("\nRe-cluster points:")
            draw_graph(pixels, labels, centroids)

        if changed == 0:
            break

        centroids = update_centroids(pixels, labels, centroids)

        if plot:
            print("\nRe-calculate centroids:")
            draw_graph(pixels, labels, centroids)


def filter_image(image, k, plot=False):
    img = downscale_image(image)
    pixels = np.asarray(img.getdata())

    centroids = np.empty([k, 3])
    labels = np.empty(pixels.shape[0])

    rand_indices = np.random.choice(pixels.shape[0], k, replace=False)
    for centroid_i, pixel_i in enumerate(rand_indices):
        centroids[centroid_i] = pixels[pixel_i]

    k_means(pixels, labels, centroids, plot)

    generate_image(img, labels, centroids)
