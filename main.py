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
    points = []
    image = cv2.cvtColor(image, cv2.IMREAD_GRAYSCALE)
    _, image = cv2.threshold(image, 0, 255, cv2.THRESH_BINARY)
    image = cv2.cvtColor(image, cv2.COLOR_BGR2RGB)
    height, width, _ = image.shape
    mid_width = math.ceil(width / 2)

    top_point = None
    bottom_point = None

    for i in range(height):
        if sum(image[i, mid_width]) == 0:  # Black
            if not top_point:
                continue  # Skip black "line" if no top point found.

            bottom_point = (i - 1, mid_width)
            mid = (math.ceil((top_point[0] + bottom_point[0]) / 2), mid_width)
            points.append(mid)
            top_point = None
            bottom_point = None

        else:
            if top_point is None:
                top_point = (i, mid_width)

    if top_point is not None and bottom_point is None:
        mid = (math.ceil((top_point[0] + height) / 2), mid_width)
        points.append(mid)

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
