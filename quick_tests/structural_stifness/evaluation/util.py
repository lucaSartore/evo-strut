import numpy as np
from functools import reduce
from custom_types import Stiffness, Point, Settings
import math


# calculate the stiffness of a series of multiple trusses in series
def stiffness_series(s: list[Stiffness]) -> Stiffness:
    e = np.sum(np.stack([np.linalg.inv(e) for e in s]), axis=0)
    assert e.shape == (2,2)
    return np.linalg.inv(e)


# calculate the stiffness of a series of multiple trusses in parallel
def stiffness_parallel(s: list[Stiffness]) -> Stiffness:
    e = np.sum(np.stack(s), axis=0)
    assert e.shape == (2,2)
    return e

# calculate the stiffness of a Cantilever Beam
def calculate_beam_stiffness(point_from: Point, point_to: Point, settings: Settings) -> Stiffness:
    ea = settings.ea
    ei = settings.ei

    # first we assume the beam is horizontal
    distance = Point.distance(point_from, point_to)
    stiffness_y = 3 * ei / (distance**3)
    stiffness_x = ea / distance

    stiffness = np.asarray([[stiffness_x, 0], [0,stiffness_y]])

    # return stiffness
    # the we rotate the stiffness
    diff = point_to - point_from
    angle = -math.atan2(diff.y, diff.x)

    rotation_matrix = np.asarray(
        [[math.cos(angle), -math.sin(angle)],
         [math.sin(angle),  math.cos(angle)]]
    )
    rotated = rotation_matrix.T @ stiffness @ rotation_matrix
    print(f"angle: {angle}")
    print(f"pre-rotation: {stiffness}")
    print(f"post-rotation: {rotated}")

    return rotated


def calculate_stiffness(point: Point, supports: list[tuple[Point, Stiffness]], settings: Settings) -> Stiffness:
    supports_stiffness: list[Stiffness] = []

    for (support_p ,support_s) in supports:
        beam_stiffness = calculate_beam_stiffness(support_p, point, settings)
        full_stiffness = stiffness_series([support_s, beam_stiffness])
        supports_stiffness.append(full_stiffness)

    return stiffness_parallel(supports_stiffness)
