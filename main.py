#!.venv/bin/python3

import os
import sys
import tempfile
import cv2
import numpy as np
from PIL import Image


def isolate_colour_range(image, _range):
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower, upper = _range
    mask = cv2.inRange(hsv, lower, upper)
    return cv2.bitwise_and(image, image, mask=mask)


# SRC: https://stackoverflow.com/a/55590133
def unsharp_mask(image, kernel_size=(5, 5), sigma=1.0, amount=1.0, threshold=0):
    """Return a sharpened version of the image, using an unsharp mask."""
    blurred = cv2.GaussianBlur(image, kernel_size, sigma)
    sharpened = float(amount + 1) * image - float(amount) * blurred
    sharpened = np.maximum(sharpened, np.zeros(sharpened.shape))
    sharpened = np.minimum(sharpened, 255 * np.ones(sharpened.shape))
    sharpened = sharpened.round().astype(np.uint8)
    if threshold > 0:
        low_contrast_mask = np.absolute(image - blurred) < threshold
        np.copyto(sharpened, image, where=low_contrast_mask)

    return sharpened


def find_edge_coords(image):
    tmp = f"{tempfile.mktemp()}.png"
    cv2.imwrite(tmp, image)
    image = Image.open(tmp).convert("RGB")

    all_pixels = []
    pixels = image.load()
    w, h = image.size
    for i in range(w):
        for j in range(h):
            pixel = pixels[i, j]
            if sum(pixel) == 255 * 3:
                continue

            all_pixels.append(
                (
                    pixel,
                    (i, j),
                )
            )

    os.remove(tmp)
    return all_pixels


def process_image(image):
    image = unsharp_mask(image, sigma=2, amount=6.5)
    cl = isolate_colour_range(
        image,
        [
            np.array([22, 93, 0]),  # Lower yellow
            np.array([45, 255, 255]),  # Upper yellow
        ],
    )

    edges = cv2.Canny(cl, 100, 200)
    _, sqrs = cv2.threshold(edges, 100, 255, cv2.THRESH_BINARY_INV)

    coords = find_edge_coords(sqrs)
    print(coords)

    # cv2.imwrite("output.png", sqrs)


def main(*args):
    if len(args) != 2:
        print(f"Usage: {args[0]} <filepath>")
        exit(1)

    if not os.path.exists(args[1]) or not os.path.isfile(args[1]):
        print(f"Image '{args[1]}' could not be found or is not a file.")
        exit(1)

    process_image(cv2.imread(args[1]))


if __name__ == "__main__":
    main(*sys.argv)
