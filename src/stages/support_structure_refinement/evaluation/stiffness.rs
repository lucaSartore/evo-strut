use std::ops::Mul;

use crate::models::{MaterialStiffnessSettings, Point};
use nalgebra::{ArrayStorage, Matrix6};

/// stiffness of a point across all major axis (as well as the
/// distortion coefficients of the two angles)
#[derive(Clone, Debug)]
pub struct Stiffness(pub Matrix6<f32>);

impl Mul<f32> for Stiffness {
    type Output = Stiffness;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Stiffness {
    pub const STF: f32 = 1e10;
    /// stiffness of element that are supported
    pub const SUPPORTED_STIFFNESS: Stiffness =
        Stiffness(Matrix6::from_array_storage(ArrayStorage([
            [Self::STF, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, Self::STF, 0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, Self::STF, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, Self::STF, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0, Self::STF, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, Self::STF],
        ])));
}

/// Calculate the stiffness of a series of two trusses
///
/// This function computes the combined stiffness when one beam (base_stiffness)
/// is connected from point_from to point_to, and the result is in series with
/// a cantilever beam from point_from to point_to.
pub fn stiffness_series(
    base_stiffness: &Stiffness,
    point_from: Point,
    point_to: Point,
    radius: f32,
    settings: &MaterialStiffnessSettings,
) -> Stiffness {
    if base_stiffness.0.iter().all(|x| *x == 0.) {
        return Stiffness(Matrix6::zeros());
    }
    let beam_stiffness = calculate_beam_stiffness(point_from, point_to, radius, settings);
    let v = point_to - point_from;

    // Jacobian matrix that maps the beam's "point_to" from the degrees of freedom
    // of the beam's "point_from". There are 6 degrees of freedom: [x,y,z,roll,pitch,yaw]
    let mut jacobian = Matrix6::zeros();
    jacobian[(0, 0)] = 1.0;
    jacobian[(0, 4)] = v.z;
    jacobian[(0, 5)] = -v.y;

    jacobian[(1, 1)] = 1.0;
    jacobian[(1, 3)] = -v.z;
    jacobian[(1, 5)] = v.x;

    jacobian[(2, 2)] = 1.0;
    jacobian[(2, 3)] = v.y;
    jacobian[(2, 4)] = -v.x;

    jacobian[(3, 3)] = 1.0;
    jacobian[(4, 4)] = 1.0;
    jacobian[(5, 5)] = 1.0;

    // Calculate compliances (inverse of stiffness)
    let Some(base_compliance) = base_stiffness.0.try_inverse() else {
        panic!("Failed to invert base stiffness matrix {:?}", base_stiffness.0);
    };
    // when the beam is too short, the stiffness matrix is so high that is some times not
    // invertible. In that case we default to a zero matrix
    let beam_compliance = beam_stiffness.0.try_inverse().unwrap_or(Matrix6::zeros());
    // Calculate full compliance: beam_compliance + jacobian * base_compliance * jacobian.T
    let full_compliance = beam_compliance + jacobian * base_compliance * jacobian.transpose();

    // Invert to get stiffness
    let full_stiffness = full_compliance
        .try_inverse()
        .expect("Failed to invert full compliance matrix");

    Stiffness(full_stiffness)
}

/// Calculate the stiffness of multiple trusses in parallel
///
/// When multiple supports act in parallel, their stiffness matrices are summed.
pub fn stiffness_parallel(s: &[Stiffness]) -> Stiffness {
    if s.is_empty() {
        return Stiffness(Matrix6::zeros());
    }

    let mut result = s[0].0;
    for stiffness in &s[1..] {
        result += stiffness.0;
    }
    // ensuring stiffness is never too high
    result
        .iter_mut()
        .for_each(|x| *x = x.clamp(-Stiffness::STF, Stiffness::STF));

    Stiffness(result)
}

/// Calculate the stiffness of a cantilever beam
///
/// This function calculates the stiffness matrix of a beam in its local coordinate
/// system and then rotates it to the global coordinate system based on the beam's orientation.
pub fn calculate_beam_stiffness(
    point_from: Point,
    point_to: Point,
    radius: f32,
    settings: &MaterialStiffnessSettings,
) -> Stiffness {
    let beam_length = (point_to - point_from).abs();
    let beam_stiffness = get_stiffness_matrix(beam_length, radius, &settings);

    let v = point_to - point_from;
    let beam_vec = [v.x, v.y, v.z];
    let translation_matrix = get_rotation_matrix(&beam_vec);

    // Rotate: T.T * K * T
    let mut rotated = translation_matrix.transpose() * beam_stiffness * translation_matrix;

    // ensuring stiffness is never too high
    rotated
        .iter_mut()
        .for_each(|x| *x = x.clamp(-Stiffness::STF, Stiffness::STF));

    Stiffness(rotated)
}

/// Return the stiffness matrix of a beam with a specific length
///
/// The beam is assumed to be parallel to the x-axis. The matrix includes
/// effects of axial, bending, and torsional stiffness.
fn get_stiffness_matrix(beam_length: f32, radius: f32, settings: &MaterialStiffnessSettings) -> Matrix6<f32> {
    let area = 2. * radius * settings.area_multiplier;
    let ea = settings.e_mod * area;
    let eiz = settings.e_mod * settings.iz;
    let eiy = settings.e_mod * settings.iy;
    let gj = settings.g_mod * settings.jxx;

    let l = beam_length;

    let kxx = ea / l;
    let kyy = 12.0 * eiz / (l * l * l);
    let kiy = -6.0 * eiz / (l * l);
    let kzz = 12.0 * eiy / (l * l * l);
    let kpz = 6.0 * eiy / (l * l);
    let krr = gj / l;
    let kpp = 4.0 * eiy / l;
    let kii = 4.0 * eiz / l;

    // Matrix axis ordered as: x,y,z,roll,pitch,yaw
    let mut matrix = Matrix6::zeros();

    matrix[(0, 0)] = kxx;

    matrix[(1, 1)] = kyy;
    matrix[(1, 5)] = kiy;

    matrix[(2, 2)] = kzz;
    matrix[(2, 4)] = kpz;

    matrix[(3, 3)] = krr;

    matrix[(4, 2)] = kpz;
    matrix[(4, 4)] = kpp;

    matrix[(5, 1)] = kiy;
    matrix[(5, 5)] = kii;

    matrix
}

/// Compute the rotation matrix for transforming a beam to its local coordinate system
///
/// This creates a 6x6 transformation matrix where the top-left and bottom-right 3x3 blocks
/// contain the rotation matrix. The rotation matrix has the beam direction as its x-axis.
fn get_rotation_matrix(beam_vec: &[f32; 3]) -> Matrix6<f32> {
    let beam_vec = [beam_vec[0], beam_vec[1], beam_vec[2]];

    // 1. Local x-axis (normalized beam direction)
    let l = (beam_vec[0].powi(2) + beam_vec[1].powi(2) + beam_vec[2].powi(2)).sqrt();
    let nx = [beam_vec[0] / l, beam_vec[1] / l, beam_vec[2] / l];

    // 2. Define a temporary 'up' vector
    // If the beam is nearly vertical, use the z-axis as a reference instead
    let up = if (nx[0].abs() < 1e-6 && (nx[1] - 1.0).abs() < 1e-6)
        || (nx[0].abs() < 1e-6 && (nx[1] + 1.0).abs() < 1e-6)
    {
        [0.0, 0.0, 1.0]
    } else {
        [0.0, 1.0, 0.0]
    };

    // 3. Derive local z and y using cross products
    // nz = nx × up
    let nz_raw = [
        nx[1] * up[2] - nx[2] * up[1],
        nx[2] * up[0] - nx[0] * up[2],
        nx[0] * up[1] - nx[1] * up[0],
    ];
    let nz_norm = (nz_raw[0].powi(2) + nz_raw[1].powi(2) + nz_raw[2].powi(2)).sqrt();
    let nz = [
        nz_raw[0] / nz_norm,
        nz_raw[1] / nz_norm,
        nz_raw[2] / nz_norm,
    ];

    // ny = nz × nx
    let ny = [
        nz[1] * nx[2] - nz[2] * nx[1],
        nz[2] * nx[0] - nz[0] * nx[2],
        nz[0] * nx[1] - nz[1] * nx[0],
    ];

    // 4. Create 3x3 R and embed in 6x6 T
    let mut t = Matrix6::zeros();

    // Top-left 3x3 block: rotation matrix
    t[(0, 0)] = nx[0];
    t[(0, 1)] = nx[1];
    t[(0, 2)] = nx[2];
    t[(1, 0)] = ny[0];
    t[(1, 1)] = ny[1];
    t[(1, 2)] = ny[2];
    t[(2, 0)] = nz[0];
    t[(2, 1)] = nz[1];
    t[(2, 2)] = nz[2];

    // Bottom-right 3x3 block: same rotation matrix
    t[(3, 3)] = nx[0];
    t[(3, 4)] = nx[1];
    t[(3, 5)] = nx[2];
    t[(4, 3)] = ny[0];
    t[(4, 4)] = ny[1];
    t[(4, 5)] = ny[2];
    t[(5, 3)] = nz[0];
    t[(5, 4)] = nz[1];
    t[(5, 5)] = nz[2];

    t
}
