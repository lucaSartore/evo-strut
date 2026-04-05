import numpy as np
from custom_types import Stiffness, Point, Settings, StiffnessResult
import math


def calculate_accuracy(stiffness: StiffnessResult, ground_truth: StiffnessResult) -> float:
    errors_percentage = []
    for k in stiffness:
        result = stiffness[k]
        actual = ground_truth[k]
        difference = np.abs(result - actual)
        filter = np.abs(actual) > 1e-4
        percentage = difference[filter] / actual[filter]
        errors_percentage.append(percentage.flatten())

    errors = np.hstack(errors_percentage)
    average_error = np.average(errors)
    return 1.0 - float(average_error)
        

def stiffness_series(base_stiffness: Stiffness, point_from: Point, point_to: Point, settings: Settings) -> Stiffness:
    beam_stiffness = calculate_beam_stiffness(point_from, point_to, settings)
    beam_vector = point_to - point_from

    # transformation matrix to project
    conversion_matrix = np.asarray([
        [1, 0, -beam_vector.y],
        [0, 1, beam_vector.x],
        [0, 0, 1]
    ])

    base_compliance = np.linalg.inv(base_stiffness)
    beam_compliance = np.linalg.inv(beam_stiffness)

    # in the second part of the equation we are:
    # - projecting the force in the base frame (conversion_matrix.T)
    # - calculating the displacement of the force (base_compliance)
    # - re-projecting the displacement on the final beam (conversion_matrix)
    # the two compliances are then summed (summing the distortion that is generated
    # by the first stick and by the second stick)
    full_compliance = beam_compliance + conversion_matrix @ base_compliance @ conversion_matrix.T

    full_stiffness = np.linalg.inv(full_compliance)
    return full_stiffness



# calculate the stiffness of a series of multiple trusses in parallel
def stiffness_parallel(s: list[Stiffness]) -> Stiffness:
    e = np.sum(np.stack(s), axis=0)
    assert e.shape == (3,3)
    return e


# calculate the stiffness of a Cantilever Beam
def calculate_beam_stiffness(point_from: Point, point_to: Point, settings: Settings) -> Stiffness:
    ea = settings.ea
    ei = settings.ei

    # first we assume the beam is horizontal
    distance = Point.distance(point_from, point_to)
    kxx = ea / distance
    kyy = 12 * ei / (distance**3)
    kyt = -6 * ei / (distance**2)
    kty = -6 * ei / (distance**2)
    ktt = 4 * ei / distance

    stiffness = np.asarray([
        [kxx, 0, 0], 
        [0, kyy, kyt],
        [0, kty, ktt]
    ])

    # return stiffness
    # the we rotate the stiffness
    diff = point_to - point_from
    # todo: it is still unclear to me why there shall be a minus here...
    angle = -math.atan2(diff.y, diff.x)

    rotation_matrix = np.asarray(
        [[math.cos(angle), -math.sin(angle), 0],
         [math.sin(angle),  math.cos(angle), 0],
         [0,                0,               1]
         ]
    )

    rotated = rotation_matrix.T @ stiffness @ rotation_matrix
    # print(f"angle: {angle}")
    # print(f"pre-rotation: {stiffness}")
    # print(f"post-rotation: {rotated}")

    return rotated


def calculate_stiffness(point: Point, supports: list[tuple[Point, Stiffness]], settings: Settings) -> Stiffness:
    supports_stiffness: list[Stiffness] = []

    for (support_p ,support_s) in supports:
        full_stiffness = stiffness_series(support_s, support_p, point, settings)
        supports_stiffness.append(full_stiffness)

    return stiffness_parallel(supports_stiffness)
