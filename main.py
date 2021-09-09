import os
import sys
import cv2
import math
import numpy as np


def isolate_colour_range(image, _range):
    hsv = cv2.cvtColor(image, cv2.COLOR_BGR2HSV)
    lower, upper = _range
    mask = cv2.inRange(hsv, lower, upper)
    return cv2.bitwise_and(image, image, mask=mask)


def find_mid_points(image):
    image = cv2.cvtColor(image, cv2.IMREAD_GRAYSCALE)
    _, _image = cv2.threshold(image, 10, 255, cv2.THRESH_BINARY)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)

    ro_height, width, _ = image.shape
    height = ro_height
    mid_width = math.ceil(width / 2)

    # FIXME: After the first square the "middle points" found are actually the bottom.
    points = []
    while True:
        top_point = None
        bottom_point = None
        for i in range(height):
            lp_height = ro_height - height
            if sum(image[lp_height + i, mid_width]) == 0:
                if top_point is None:
                    continue

                bottom_point = (lp_height + (i - 1), mid_width)
                break

            if top_point is None:
                top_point = (lp_height + i, mid_width)
                continue

        if top_point is None or bottom_point is None:
            break

        sqr = (math.ceil((top_point[0] + bottom_point[0]) / 2), mid_width)
        points.append(sqr)
        height -= bottom_point[0]

    return points


def process_sqrs(image, tube):
    image = isolate_colour_range(
        image,
        [
            np.array([0, 50, 0]),  # Lower yellow
            np.array([45, 255, 255]),  # Upper yellow
        ],
    )

    image = cv2.medianBlur(image, 101)
    points = find_mid_points(image)
    print(points)


def process_tube(image):
    image = cv2.medianBlur(image, 101)
    height, width, _ = image.shape
    mid = math.ceil(height / 2), math.ceil(width / 2)
    r, g, b = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)[mid]
    return (r, g, b)


if __name__ == "__main__":
    sqrs = "sqrs.jpeg"
    tube = "tube.jpeg"
    if len(sys.argv) > 3:
        sqrs, tube = sys.argv[1:3]

    images = [sqrs, tube]

    if not all([os.path.exists(image) for image in images]) or not all(
        [os.path.isfile(image) for image in images]
    ):
        raise ValueError(f"Could not find one or more of these images: {images}")

    sqrs = cv2.imread(sqrs)
    tube = cv2.imread(tube)

    tube = process_tube(tube)
    sqrs = process_sqrs(sqrs, tube)
