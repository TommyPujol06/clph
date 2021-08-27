#!.venv/bin/python3

import sys
import os
import cv2
import numpy as np


def isolate_colour_range(image, _range):
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower, upper = _range
    mask = cv2.inRange(hsv, lower, upper)
    return cv2.bitwise_and(image, image, mask=mask)


def process_image(image):
    image = isolate_colour_range(
        image,
        [
            np.array([22, 93, 0]),  # Lower yellow
            np.array([45, 255, 255]),  # Upper yellow
        ],
    )

    edges = cv2.Canny(image, 100, 200)
    _, image = cv2.threshold(edges, 100, 255, cv2.THRESH_BINARY_INV)

    cv2.imwrite("output.png", image)


def main(*args):
    if len(args) != 2:
        print(f"Usage: {args[0]} <filepath>")
        exit(1)

    if not os.path.exists(args[1]) or not os.path.isfile(args[1]):
        print(f"Image '{args[1]}' could not be found or is not a file.")
        exit(1)

    process_image(cv2.imread(args[1]))


if __name__ == "__main__":
    main(*sys.argv[0:])
