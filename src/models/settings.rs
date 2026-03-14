use std::f32::consts::PI;

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
    pub output_file_path: String
}

impl Default for IoSettings {
    fn default() -> Self {
        Self {
            input_file_path: "test_meshes/dragon.stl".into(),
            output_file_path: "output.stl".into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactPointsOptimizationSettings {
    /// cost associated with not supporting a certain
    /// area with an angle that is steeper than the threshold
    /// unit of measure: [cost/mm^2]
    pub non_supported_cost: f32,
    /// cost associated with placing one support point
    /// unit of measure: [cost]
    pub support_point_cost: f32,
    /// cost associated with placing one support line 
    /// with a specific length
    /// unit of measure [cost/mm]
    pub support_line_cost: f32,
    /// the resolution that is used when evaluating the criticality level
    /// smaller values makes the simulation take longer, but they also make it more precise.
    /// the default value is 1mm
    /// unit of measure3 [mm]
    pub discretization_size: f32,
    /// cost associated with not supporting a "min point"
    /// (i.e. a point that is not touching the flor, and is the
    /// lower among all of his neighbors)
    /// it goes without saying that this should be set to something
    /// sufficiently high
    /// unit of measure: [cost]
    pub cost_of_unsupported_min_point: f32

}

impl Default for ContactPointsOptimizationSettings {
    fn default() -> Self {
        Self {
            non_supported_cost: 0.1,
            support_point_cost: 1.0,
            support_line_cost: 0.5,
            discretization_size: 1.0,
            cost_of_unsupported_min_point: 100.0
        }
    }
}

