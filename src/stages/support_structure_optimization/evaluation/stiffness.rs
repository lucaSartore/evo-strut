use nalgebra::Matrix4;

/// stiffness of a point across all major axis (as well as the
/// distortion coefficients of the two angles)
struct Stiffness(Matrix6<f32>);

#[derive(Clone, Debug)]
struct MaterialStiffnessSettings {
    area: float,
    e_mod: float,
    g_mod: float,
    jxx: float,
    iy: float,
    iz: float
}

impl Default for MaterialStiffnessSettings {
    fn default() -> Self {
        Self { 
            area: 4,
            e_mod: 3000,
            g_mod: 1000,
            jxx: 6,
            iy: 3,
            iz: 3
        }
    }
}


pub fn stiffness_series(base_stiffness: Stiffness, point_from: Point, point_to: Point, settings: MaterialStiffnessSettings) -> Stiffness {
}


pub fn stiffness_parallel(s: &[Stiffness]) -> Stiffness {
}


pub fn calculate_beam_stiffness(point_from: Point, point_to: Point, settings: MaterialStiffnessSettings) -> Stiffness {
}


pub fn calculate_stiffness(point: Point, supports: &[(Point, Stiffness)], settings: MaterialStiffnessSettings) -> Stiffness {
}
