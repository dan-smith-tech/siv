import numpy as np
from PIL import Image


def downscale_image(image):
    img = Image.open(image, "r")
    img.thumbnail((1920, 1080))
    return img


def generate_image(image, image_output, labels, centroids):
    print(centroids)

    new_image = np.array(
        [
            [
                np.round(centroids[int(labels[y * image.size[0] + x])]).astype(int)
                for x in range(image.size[0])
            ]
            for y in range(image.size[1])
        ]
    )

    Image.fromarray(new_image.astype(np.uint8), mode="RGB").save(image_output)
