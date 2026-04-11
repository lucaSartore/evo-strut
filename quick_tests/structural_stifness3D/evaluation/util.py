import numpy as np
from scipy.stats import pearsonr, spearmanr
from custom_types import Stiffness, Point, Settings, StiffnessResult


def filter_stiffness_components(stiffness: StiffnessResult) -> StiffnessResult:
    """
    filter out a stiffness matrix in order to keep only the values we are interested in
    (stiffness along xx and yy directions)
    """
    return {
        k: np.asarray([s[0,0], s[1,1]])
        for k, s in stiffness.items()
    }

def flatten_stiffness_result(stiffness: StiffnessResult) -> np.ndarray:
    """Flatten a StiffnessResult (dict of 6x6 matrices) into a 1D array."""
    arrays = []
    for k in sorted(stiffness.keys()):
        arrays.append(stiffness[k].flatten())
    return np.hstack(arrays) if arrays else np.array([])


def calculate_mape(predicted: StiffnessResult, actual: StiffnessResult) -> float:
    """
    Calculate Mean Absolute Percentage Error (MAPE).
    MAPE = (1/n) * Σ|actual - predicted| / |actual| * 100
    """
    errors_percentage = []
    for k in predicted:
        pred = predicted[k]
        true = actual[k]
        difference = np.abs(true - pred)
        # Filter out near-zero values to avoid division by zero
        mask = np.abs(true) > 1e-4
        if np.any(mask):
            percentage = (difference[mask] / np.abs(true[mask])) * 100
            errors_percentage.append(percentage.flatten())
    
    if not errors_percentage:
        return 0.0

    errors = np.hstack(errors_percentage)
    return float(np.mean(errors))


def calculate_pearson_correlation(predicted: StiffnessResult, actual: StiffnessResult) -> float:
    """
    Calculate Pearson Correlation Coefficient between predicted and actual stiffness values.
    Range: [-1, 1], where 1 is perfect correlation.
    """
    pred_flat = flatten_stiffness_result(predicted)
    actual_flat = flatten_stiffness_result(actual)
    
    if len(pred_flat) < 2 or len(actual_flat) < 2:
        return 0.0
    
    try:
        correlation, _ = pearsonr(actual_flat, pred_flat)
        return float(correlation)
    except:
        return 0.0


def calculate_spearman_correlation(predicted: StiffnessResult, actual: StiffnessResult) -> float:
    """
    Calculate Spearman Rank Correlation Coefficient between predicted and actual stiffness values.
    Range: [-1, 1], where 1 is perfect correlation.
    """
    pred_flat = flatten_stiffness_result(predicted)
    actual_flat = flatten_stiffness_result(actual)
    
    if len(pred_flat) < 2 or len(actual_flat) < 2:
        return 0.0
    
    try:
        correlation, _ = spearmanr(actual_flat, pred_flat)
        return float(correlation)
    except:
        return 0.0


def calculate_accuracy(stiffness: StiffnessResult, ground_truth: StiffnessResult) -> float:
    """Legacy function for backward compatibility."""
    errors_percentage = []
    for k in stiffness:
        result = stiffness[k]
        actual = ground_truth[k]
        difference = np.abs(result - actual)
        filter = np.abs(actual) > 1e-4
        percentage = np.clip(0, 1.0, difference[filter] / np.abs(actual[filter]))
        errors_percentage.append(percentage.flatten())

    errors = np.hstack(errors_percentage)
    average_error = np.average(errors)
    return 1.0 - float(average_error)
        

def stiffness_series(base_stiffness: Stiffness, point_from: Point, point_to: Point, settings: Settings) -> Stiffness:
    beam_stiffness = calculate_beam_stiffness(point_from, point_to, settings)
    v = point_to - point_from

    # jacobian matrix of the function that maps the beam's "point_to"
    # starting from the degrees of freedom of the beams's "point_from"
    # there are 6 degrees of freedom as input and output [x,y,z,roll,pitch,yaw]
    jacobian = np.asarray([
     [1, 0, 0, 0   , v.z , -v.y ],
     [0, 1, 0, -v.z, 0   , v.x ],
     [0, 0, 1, v.y , -v.x, 0 ],
     [0, 0, 0, 1   , 0   , 0 ],
     [0, 0, 0, 0   , 1   , 0 ],
     [0, 0, 0, 0   , 0   , 1 ],
    ])

    base_compliance = np.linalg.inv(base_stiffness)
    beam_compliance = np.linalg.inv(beam_stiffness)

    # in the second part of the equation we are:
    # - projecting the force in the base frame (conversion_matrix.T)
    # - calculating the displacement of the force (base_compliance)
    # - re-projecting the displacement on the final beam (conversion_matrix)
    # the two compliances are then summed (summing the distortion that is generated
    # by the first stick and by the second stick)
    full_compliance = beam_compliance + jacobian @ base_compliance @ jacobian.T

    full_stiffness = np.linalg.inv(full_compliance)

    return full_stiffness



# calculate the stiffness of a series of multiple trusses in parallel
def stiffness_parallel(s: list[Stiffness]) -> Stiffness:
    e = np.sum(np.stack(s), axis=0)
    assert e.shape == (6,6)
    return e


# calculate the stiffness of a Cantilever Beam
def calculate_beam_stiffness(point_from: Point, point_to: Point, settings: Settings) -> Stiffness:
    len = (point_from - point_to).abs()
    stiffness = get_stiffness_matrix(len, settings)

    v = np.asarray((point_to - point_from).as_list())
    translation_matrix = get_rotation_matrix(v)

    rotated = translation_matrix.T @ stiffness @ translation_matrix

    return rotated

# return the stiffness matrix of a beam with a specific length
# the beam is assumed to be parallel to the x axis
def get_stiffness_matrix(beam_length: float, settings: Settings) -> Stiffness:
    ea = settings.e_mod * settings.area
    eiz = settings.e_mod * settings.iz
    eiy = settings.e_mod * settings.iy
    gj = settings.g_mod * settings.jxx

    l = beam_length

    kxx = ea / l
    kyy = 12 * eiz / (l**3)
    kiy = -6 * eiz / (l**2)
    kzz = 12 * eiy / (l**3)
    kpz = 6 * eiy / (l**2)
    krr = gj / l
    kpp = 4 * eiy / l
    kii = 4 * eiz / l

    # matrix axis ordered in: x,y,z,roll,pitch,yaw
    return np.asarray([
        [kxx, 0,   0,   0,   0,   0   ], 
        [0,   kyy, 0,   0,   0,   kiy ], 
        [0,   0,   kzz, 0,   kpz, 0   ], 
        [0,   0,   0,   krr, 0,   0   ], 
        [0,   0,   kpz, 0,   kpp, 0   ], 
        [0,   kiy, 0,   0,   0,   kii ]
    ])

def get_rotation_matrix(beam_vec: np.ndarray) -> np.ndarray:
    # 1. Local x-axis
    L = np.linalg.norm(beam_vec)
    nx = beam_vec / L
    
    # 2. Define a temporary 'up' vector
    # If the beam is vertical, use the Z-axis as a reference instead
    if np.allclose(nx, [0, 1, 0]) or np.allclose(nx, [0, -1, 0]):
        up = np.array([0, 0, 1])
    else:
        up = np.array([0, 1, 0])
    
    # 3. Derive local z and y using cross products
    nz = np.cross(nx, up)
    nz /= np.linalg.norm(nz)
    ny = np.cross(nz, nx)
    
    # 4. Create 3x3 R
    R = np.array([nx, ny, nz])
    
    # 5. Create 6x6 T
    T = np.zeros((6, 6))
    T[0:3, 0:3] = R
    T[3:6, 3:6] = R
    
    return T


def calculate_stiffness(point: Point, supports: list[tuple[Point, Stiffness]], settings: Settings) -> Stiffness:
    supports_stiffness: list[Stiffness] = []

    for (support_p ,support_s) in supports:
        full_stiffness = stiffness_series(support_s, support_p, point, settings)
        supports_stiffness.append(full_stiffness)

    return stiffness_parallel(supports_stiffness)
