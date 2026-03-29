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
    /// optionally specify a path where to write the
    /// re-meshed input.
    pub re_meshed_input_file_path: Option<String>,
    /// used to re-mesh the input when loading it
    /// smaller length make the process more precise
    /// but also slower.
    /// put 0 to skip the re-meshing phase (note that doing
    /// so in a mesh that is not properly meshed will result in
    /// poor performances of the algorithm)
    /// unit of measure: mm
    pub target_edge_length: f32
}

impl Default for IoSettings {
    fn default() -> Self {
        Self {
            // input_file_path: "test_meshes/inclination_test.stl".into(),
            input_file_path: "test_meshes/dragon_re_meshed.stl".into(),
            re_meshed_input_file_path: None,
            output_file_path: "output.stl".into(),
            target_edge_length: 0.
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactPointsOptimizationSettings {
    /// tell how the cost propagate from one critical surface to the next
    /// example:
    ///  - point A and B are connected by a 2mm gap
    ///  - the triangle has a `critical angle` of 30 (i.e. is 30 degrees less steep than what is
    ///    considered non critical by support_overhanging_angle)
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
    /// unit cost associated with placing one support point
    /// unit of measure: [cost]
    pub support_point_cost: f32,
    /// cost associated the placing a support
    /// with a specific area
    /// unit of measure [cost/mm^2]
    pub support_area_cost: f32,
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

    /// size of the population evaluated (in the genetic algorithm)
    pub population_size: usize,

    /// max allowed radius of optimized supports
    pub max_support_radius: f32,

    /// min allowed radius of optimized supports
    pub min_support_radius: f32,

    /// how much a support should be moved
    /// when his position is mutated
    /// unit of measure: [mm]
    pub move_support_mutation_intensity: f32,

    /// how much the radius of a support should change
    /// when the change radius mutation is applied
    /// unit of measure: [mm]
    pub change_support_radius_mutation_intensity: f32,

    /// number of generations optimized
    pub num_generations: usize,
    /// patience when optimizing (if the score does not improve
    /// for more than `patience` generations the optimization process will
    /// be interrupted)
    pub patience: usize,
    /// the number of individuals in a generation
    pub generation_size: usize,
    /// the size of the tournaments made to select the individuals for crossover.
    /// The tradeoffs are:
    ///  - High tournament size => high selection pressure => fast to converge, may lose diversity
    ///  too early
    ///  - Small tournament size => slow selection process => slow to converge, preserve diversity
    pub tournament_size: usize,
    /// number of individual generated/evaluated in every generation
    pub num_elite_individuals: usize
}

impl Default for ContactPointsOptimizationSettings {
    fn default() -> Self {
        Self {
            cost_surplus_propagation_factor: 10.,
            support_point_cost: 500.0,
            support_area_cost: 50.0,
            non_supported_base_cost: 1000.0,
            layer_height: 1.,
            critical_angle_clipping_factor: 5.,
            initialization_support_density: RandomDistribution::InRange { low: 0.0001, high: 0.001 },
            population_size: 100,
            max_support_radius: 4.,
            min_support_radius: 0.5,
            move_support_mutation_intensity: 2.5,
            change_support_radius_mutation_intensity: 2.,
            num_generations: 2,
            patience: 25,
            generation_size: 100,
            tournament_size: 10,
            num_elite_individuals: 10
        }
    }
}

