import numpy as np
from scipy import stats
from custom_types import Stiffness, Point, Settings, StiffnessResult


def filter_stiffness_components(stiffness: StiffnessResult):
    return {
        k: np.asarray([v[0,0], v[1,1]])
        for k,v in stiffness.items()
    }

def calculate_root_mean_squared_relative_error(stiffness: StiffnessResult, ground_truth: StiffnessResult, epsilon = 1e-5) -> float:
    values = np.stack([s for s in stiffness.values()]);
    gt = np.stack([s for s in ground_truth.values()]);

    error = np.abs(values - gt)
    denominator = np.abs(gt) + epsilon

    mse = np.average((error / denominator) ** 2)
    return mse * 100

def calculate_spearman_correlation(stiffness: StiffnessResult, ground_truth: StiffnessResult, epsilon = 1e-5) -> float:
    values = np.stack([s for s in stiffness.values()]).flatten()
    gt = np.stack([s for s in ground_truth.values()]).flatten()
    
    correlation, _ = stats.spearmanr(values, gt)
    return float(correlation) #type: ignore

def calculate_pearson_correlation(stiffness: StiffnessResult, ground_truth: StiffnessResult, epsilon = 1e-5) -> float:
    values = np.stack([s for s in stiffness.values()]).flatten()
    gt = np.stack([s for s in ground_truth.values()]).flatten()
    correlation, _ = stats.pearsonr(values, gt)
    return float(correlation) #type: ignore

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
