use crate::support::random_distribution::RandomDistribution;


#[derive(Default, Debug, Clone)]
pub struct Settings {
    /// input output parameters
    pub io_settings: IoSettings,
    /// parameters that define what constitute a "critical" surface
    /// (i.e. a surface that needs supports)
    pub criticality_settings: CriticalitySettings,
    /// parameters that control the optimization of the
    /// contact points. This include cost functions weights as well as 
    /// optimization hyper-parameters
    pub contact_points_optimization_settings: ContactPointsOptimizationSettings,
}

#[derive(Debug, Clone)]
pub  struct CriticalitySettings {
    /// minimum angle for which supports are added
    /// if set to zero all overhangs will be supported
    /// if set to 90 none of the overhangs will be supported
    /// measured in degrees
    pub support_overhanging_angle: f32,
    /// max detachment from the Z plane that a surface can have
    /// before is considered not supported.
    /// measured in mm
    pub max_detachment_from_z_plane: f32,
    /// the critical areas are expanded into adjacent surfaces
    /// in order to merge many small and grouped critical surfaces
    /// measured in mm
    pub criticality_expansion_rate: f32
}

impl Default for CriticalitySettings {
    fn default() -> Self {
        Self { 
            support_overhanging_angle: 60.,
            max_detachment_from_z_plane: 0.1,
            criticality_expansion_rate: 1.
        }
    }
}

#[derive(Debug, Clone)]
pub struct IoSettings {
    pub input_file_path: String,
    pub output_file_path: String,
    /// used to re-mesh the input when loading it
    /// smaller voxel size make the process more precise
    /// but also slower
    /// unit of measure: mm
    pub voxel_size: f32
}

impl Default for IoSettings {
    fn default() -> Self {
        Self {
            input_file_path: "test_meshes/inclination_test.stl".into(),
            // input_file_path: "test_meshes/dragon.stl".into(),
            output_file_path: "output.stl".into(),
            voxel_size: 1.
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactPointsOptimizationSettings {
    /// tell how the cost propagate from one critical surface to the next
    /// example:
    ///  - point A and B are connected by a 2mm gap
    ///  - the triangle has a `critical angle` of 30 (i.e. is 30 degrees less steep than what is
    ///  considered non critical by support_overhanging_angle)
    ///  - A is below B and has a criticality score of 100
    ///  - cost_surplus_propagation_factor is 0.1 [cost/(mm^3*deg)]
    ///  - b represent a triangle that has an area of 4 mm^2
    ///
    /// then the cost associated with B will be:
    /// ```
    ///  C(b) = C(a) + cost_surplus_propagation_factor * distance * angle * area
    ///       = 100 + 0.1 * 2.0 * 30 * 4 = 124
    /// ```
    /// unit of measure: [cost/(mm^3*deg)]
    pub cost_surplus_propagation_factor: f32,
    /// cost associated with placing one support point
    /// unit of measure: [cost]
    pub support_point_cost: f32,
    /// cost associated with placing one support line 
    /// with a specific length
    /// unit of measure [cost/mm]
    pub support_line_cost: f32,
    /// cost associated with an element that has no support at all 
    /// (i.e. a point that is not touching the flor, and is the
    /// lower among all of his neighbors)
    /// it goes without saying that this should be set to something
    /// sufficiently high
    /// unit of measure: [cost]
    pub non_supported_base_cost: f32,
    /// layer height used to propagate criticality
    /// when optimizing the contact points.
    /// note: this has nothing to do with the layer height of your printed.
    /// It should usually be set in the range 0.3-1 times voxel_size.
    /// unit of measure: [mm]
    pub layer_height: f32,

    /// when propagating the cost surplus (using cost_surplus_propagation_factor)
    /// the if the critical angle's absolute is higher than this threshold
    /// it will be clipped (to avoid having criticality that are too high)
    pub critical_angle_clipping_factor: f32,

    /// the density of support initially used
    /// unit of measure: 1/mm^2
    /// if is set to 0.05 and the area optimized
    /// has a size of 100mm^2 then 5 supports
    /// will be generated
    pub initialization_support_density: RandomDistribution,

    pub population_size: usize

}

impl Default for ContactPointsOptimizationSettings {
    fn default() -> Self {
        Self {
            cost_surplus_propagation_factor: 10.,
            support_point_cost: 100.0,
            support_line_cost: 150.0,
            non_supported_base_cost: 1000.0,
            layer_height: 1.,
            critical_angle_clipping_factor: 5.,
            initialization_support_density: RandomDistribution::InRange { low: 0.0001, high: 0.001 },
            population_size: 100
        }
    }
}

