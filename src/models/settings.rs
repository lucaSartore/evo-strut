use std::f32;

use crate::support::{
    neural_network::{
        ActivationFunction, LayerTopology, NetworkCrossoverSettings, NetworkMutationRates,
        NetworkMutationSettings, NetworkTopology, NetworkWeightInitialization,
    },
    random_distribution::RandomDistribution,
};

#[derive(Default, Debug, Clone)]
pub struct Settings {
    /// input output parameters
    pub io_settings: IoSettings,
    /// parameters that define what constitute a "critical" surface
    /// (i.e. a surface that needs supports)
    pub criticality_settings: CriticalitySettings,
    /// parameters used to detect floating regions and determine how rigidly
    /// they should be supported
    pub floating_region_detection_settings: FloatingRegionDetectionSettings,
    pub contact_points_optimization_settings: ContactPointsOptimizationSettings,
    /// parameters used to group the contact points into a disjoint sets that should
    /// be considered together when optimizing
    pub contact_points_grouping_settings: ContactPointsGroupingSettings,
    /// parameters that control the optimization of the
    /// contact points. This include cost functions weights as well as
    /// optimization hyper-parameters
    pub support_structure_optimization_settings: SupportStructureOptimizationSettings,
    /// parameters that control the optimization of the
    /// support structure.
    pub support_structure_refinement_settings: SupportStructureRefinementSettings,
    /// settings that define how the support structure is generated
    pub support_settings: SupportSettings,
}

#[derive(Debug, Clone)]
pub struct CriticalitySettings {
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
    pub criticality_expansion_rate: f32,
}

impl Default for CriticalitySettings {
    fn default() -> Self {
        Self {
            support_overhanging_angle: 55.,
            max_detachment_from_z_plane: 0.1,
            criticality_expansion_rate: 1.,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FloatingRegionDetectionSettings {
    /// multiplier used to convert floating region area into a stiffness threshold
    /// unit of measure: stiffness/mm^2
    pub stiffness_threshold_area_multiplier: f32,
}

impl Default for FloatingRegionDetectionSettings {
    fn default() -> Self {
        Self {
            stiffness_threshold_area_multiplier: 1.,
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
    pub target_edge_length: f32,
}

impl Default for IoSettings {
    fn default() -> Self {
        Self {
            // input_file_path: "test_meshes/inclination_test.stl".into(),
            // input_file_path: "test_meshes/inclination_test_re_meshed.stl".into(),
            input_file_path: "test_meshes/dragon_re_meshed.stl".into(),
            re_meshed_input_file_path: None,
            output_file_path: "test_meshes/output.stl".into(),
            target_edge_length: 0.,
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
    /// the if the critical angle is higher than this threshold
    /// it will be clipped (to avoid having criticality that are too high)
    pub critical_angle_clipping_factor_up: f32,
    /// when propagating the cost surplus (using cost_surplus_propagation_factor)
    /// the if the critical angle is lower than this threshold
    /// it will be clipped (to avoid having criticality recover too quickly)
    pub critical_angle_clipping_factor_down: f32,

    /// when propagating the cost, this value will be use to average two factors:
    /// the support given by the contact points
    /// the criticality / costs that surfaces have when no supports are applied.
    ///
    /// if set to zero, only the first one will be kept in to consideration,
    /// meaning that one support will be considered "infinitely rigid"
    pub soft_cost_propagation_factor: f32,

    /// the density of support initially used
    /// unit of measure: 1/mm^2
    /// if is set to 0.05 and the area optimized
    /// has a size of 100mm^2 then 5 supports
    /// will be generated
    pub initialization_support_density: RandomDistribution,

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
    ///    too early
    ///  - Small tournament size => slow selection process => slow to converge, preserve diversity
    pub tournament_size: usize,
    /// number of individual generated/evaluated in every generation
    pub num_elite_individuals: usize,
    /// additional cost that is added to the cost of every
    /// vertex being unsupported.
    /// the cost is calculated as such:
    ///  - given a vertex V in position B
    ///  - find the N = `num_points` closest supported points
    ///  - calculate the euclidean distance, (with a upper threshold of `max_distance`)
    ///  - calculate the `parallel_distance` = 1 / [ 1 / distance(i) for i in 0..N]
    ///     > formula inspired to the resistance in parallel, that give less cost
    ///       if a point is completely surrounded by supports
    ///  - returns the cost as `parallel_distance` * `multiplier`
    /// unit of measure: [cost / mm]
    pub additional_cost_multiplier: f32,
    /// when evaluating the closest points using a k-d tree, points are put in to
    /// buckets of size `divisor_isize`.
    /// this helps the program with cashing, and thus make it faster.
    /// A bigger value means less precision, but more cashing.
    /// unit of measure: mm
    pub additional_cost_divisor_size: f32,
    /// max distance (see description of `additional_cost_multiplier`)
    /// unit of measure: mm
    pub additional_cost_max_distance: f32,
    /// num points (see description of `additional_cost_multiplier`)
    /// unit of measure: [u]
    pub additional_cost_num_points: usize,
}

impl Default for ContactPointsOptimizationSettings {
    fn default() -> Self {
        Self {
            cost_surplus_propagation_factor: 2.5,
            support_point_cost: 800.0,
            support_area_cost: 60.0,
            non_supported_base_cost: 2500.0,
            layer_height: 1.,
            critical_angle_clipping_factor_up: 20.,
            critical_angle_clipping_factor_down: 2.5,
            soft_cost_propagation_factor: 0.03,
            initialization_support_density: RandomDistribution::InRange {
                low: 0.01,
                high: 0.015,
            },
            max_support_radius: 4.,
            min_support_radius: 0.5,
            move_support_mutation_intensity: 2.5,
            change_support_radius_mutation_intensity: 2.,
            num_generations: 2000,
            patience: 25,
            generation_size: 100,
            tournament_size: 10,
            num_elite_individuals: 10,
            additional_cost_multiplier: 50.,
            additional_cost_divisor_size: 2.5,
            additional_cost_max_distance: 20.,
            additional_cost_num_points: 10
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContactPointsGroupingSettings {
    pub area_minimization_weight: f32,
    pub volume_minimization_weight: f32,
    /// cost for creating a new group
    /// the cost is multiplied by the height of the group (taller groups, are more expensive to
    /// support)
    /// unit of measure: [cost/mm]
    pub group_cost_penalty: f32,
    // maximum number of groups produced in the grouping phase
    // this also define the number of neurons in the last layer of the neural network
    pub max_num_groups: usize,
    /// Network used to map 2D contact-point coordinates to group scores.
    /// The input layer is fixed to x/y coordinates, and the output layer has
    /// one neuron for each possible group.
    pub network_topology: NetworkTopology,
    pub network_weight_initialization: NetworkWeightInitialization,
    pub valid_mutations: Vec<NetworkMutationSettings>,
    pub valid_crossovers: Vec<NetworkCrossoverSettings>,
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
    ///    too early
    ///  - Small tournament size => slow selection process => slow to converge, preserve diversity
    pub tournament_size: usize,
    /// number of individual generated/evaluated in every generation
    pub num_elite_individuals: usize,
}

impl Default for ContactPointsGroupingSettings {
    fn default() -> Self {
        let max_num_groups = 25;

        Self {
            group_cost_penalty: 10000.,
            area_minimization_weight: 50.,
            volume_minimization_weight: 30.,
            num_generations: 2000,
            patience: 100,
            generation_size: 100,
            tournament_size: 10,
            num_elite_individuals: 10,
            max_num_groups,
            network_topology: NetworkTopology::new(
                3,
                vec![
                    LayerTopology::new(32, ActivationFunction::Relu)
                        .expect("invalid default contact-point grouping hidden layer"),
                    LayerTopology::new(max_num_groups, ActivationFunction::Sigmoid)
                        .expect("invalid default contact-point grouping output layer"),
                ],
            )
            .expect("invalid default contact-point grouping network topology"),
            network_weight_initialization: NetworkWeightInitialization::He,
            valid_mutations: vec![
                NetworkMutationSettings::new(
                    NetworkMutationRates::new(1.0, 0.0)
                        .expect("invalid default contact-point grouping mutation rates"),
                    RandomDistribution::Normal {
                        mean: 0.,
                        std_dev: 0.1,
                    },
                    RandomDistribution::InRange { low: -1., high: 1. },
                ),
                NetworkMutationSettings::new(
                    NetworkMutationRates::new(0.05, 0.02)
                        .expect("invalid default contact-point grouping mutation rates"),
                    RandomDistribution::Normal {
                        mean: 0.,
                        std_dev: 0.1,
                    },
                    RandomDistribution::InRange { low: -1., high: 1. },
                ),
                NetworkMutationSettings::new(
                    NetworkMutationRates::new(0.01, 0.15)
                        .expect("invalid default contact-point grouping mutation rates"),
                    RandomDistribution::Normal {
                        mean: 0.,
                        std_dev: 0.35,
                    },
                    RandomDistribution::InRange {
                        low: -1.5,
                        high: 1.5,
                    },
                ),
            ],
            valid_crossovers: vec![
                NetworkCrossoverSettings::uniform(),
                NetworkCrossoverSettings::single_point(),
                NetworkCrossoverSettings::arithmetic(0.5)
                    .expect("invalid default contact-point grouping crossover settings"),
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct SupportStructureOptimizationSettings {
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
    ///    too early
    ///  - Small tournament size => slow selection process => slow to converge, preserve diversity
    pub tournament_size: usize,
    /// number of individual generated/evaluated in every generation
    pub num_elite_individuals: usize,
    /// number of points that are on generated on average when a new layer is initialized
    pub num_points_per_layer: RandomDistribution,
    /// multiplier for the covariance matrix used to sample the points within a layer.
    pub points_sampling_covariance_multiplier: f32,
    /// multiplier for the number of groups that are initially present in each individual.
    /// number_of_groups = number_os_supports * num_initial_groups_multiplier
    pub num_initial_groups_multiplier: f32,
    /// how many layer of supports should be present for every mm of height.
    /// Is used in the first part of the support structure optimization stage to regenerate
    /// the support structure of a group from scratch.
    /// unit of measure 1/mm
    pub layers_number_density: RandomDistribution,
    /// how many points there should be in a layer, in relation to the area
    /// formed by the convex hull of all the points that should be supported
    /// unit of measure 1/mm^2
    pub point_in_layer_density: f32,
    /// how many points there should be in a layer, in relation to the perimeter
    /// formed by the convex hull of all the points that should be supported
    /// unit of measure 1/mm
    pub point_in_layer_perimeter_density: f32,
    /// minimum number of points that should be allowed to be in one layer
    pub min_points_in_layer: usize,
    /// used to create new nodes inside a layer starting from the nodes of the layer above
    /// is a measure of how far the nodes will be from the node above
    /// unit of measure: mm
    pub layer_node_creation_update_step: f32,
    /// standard deviation for moving a single point inside a layer.
    /// unit of measure: mm
    pub layer_point_motion_std: f32,
    /// standard deviation for adjusting the height of a layer.
    /// unit of measure: mm
    pub layer_height_motion_std: f32,
    /// used when constructing trees, control chow often points are interpolated
    /// (smaller value means more precise trees, but higher computational cost)
    /// unit of measure: mm
    pub tree_creation_interpolation_size: f32
}

impl Default for SupportStructureOptimizationSettings {
    fn default() -> Self {
        Self {
            num_generations: 5000,
            patience: 50,
            generation_size: 100,
            tournament_size: 10,
            num_elite_individuals: 10,
            num_points_per_layer: RandomDistribution::InRange { low: 1., high: 7. },
            points_sampling_covariance_multiplier: 0.3,
            num_initial_groups_multiplier: 0.7,
            layers_number_density: RandomDistribution::InRange {
                low: 0.015,
                high: 0.04,
            },
            point_in_layer_density: 0.003,
            point_in_layer_perimeter_density: 0.01,
            min_points_in_layer: 1,
            layer_node_creation_update_step: 5.,
            layer_point_motion_std: 5.,
            layer_height_motion_std: 2.5,
            tree_creation_interpolation_size: 2.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SupportStructureRefinementSettings {
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
    ///    too early
    ///  - Small tournament size => slow selection process => slow to converge, preserve diversity
    pub tournament_size: usize,
    /// number of individual generated/evaluated in every generation
    pub num_elite_individuals: usize,
    /// cost for every mm^2 of surface area of supports
    /// unit of measure: cost/mm^2
    pub cost_for_support_area: f32,
    /// cost foe every mm of support that goes over the desired angle by some degrees
    /// unit of measure: cost/(mm*deg)
    pub cost_for_support_too_steep: f32,
    /// cost associated with the support being not stiff enough
    /// unit of measure: cost/(mm/N)
    pub non_stiffness_cost: f32,
    /// resolution used when integrating over a beam stiffness to calculate the cost
    /// unit of measure: mm
    pub stiffness_cost_integration_size: f32,
    /// cost associated with a squared mm of a support cone (the last step of a support)
    /// unit of measure: cost/mm^2
    pub cone_area_cost: f32,
    /// cost associated with a cone being too steep.
    /// unit of measure cost/(mm*deg)
    pub cone_too_steep_cost: f32,
    /// cost of a support cone that is unfeasible (due to it having
    /// the base above the cone's ring
    /// unit of measure: cost
    pub cost_of_un_feasible_cone: f32,
    /// settings used to calculate the stiffness of the material
    pub material_stiffness_settings: MaterialStiffnessSettings,
    /// how much a node in the support graph is moved when mutated
    /// this parameter represent the standard deviation of the multivariate
    /// normal distribution that mutates the position
    /// unit of measure: mm
    pub node_position_mutation_std: f32,
    /// cost of a node that is not stiff at all
    /// unit of measure: cost
    pub max_non_stiffness_cost: f32,
    /// the voxel size of the volume that will be used
    /// to check collisions (to avoid supports intersecting
    /// with the mesh)
    /// unit of measure: mm
    pub collision_volume_voxel_size: f32,
    /// how much to expand the collision detection volume
    /// (this function as a safety margin)
    /// unit of measure: mm
    pub collision_volume_offset: f32,
    /// how often to check for a collision.
    /// example, if a beam is 10mm long, and the interval
    /// is 2mm, 6 checks will be performed (10/2+1) at equally
    /// sampled distance
    /// unit of measure: mm
    pub collision_check_intervals: f32,
    /// cost of a collision, per unit of length
    /// unit of measure: cost / mm
    pub collision_penalization: f32
}

impl Default for SupportStructureRefinementSettings {
    fn default() -> Self {
        Self {
            num_generations: 1,
            patience: 50,
            generation_size: 100,
            tournament_size: 10,
            num_elite_individuals: 10,
            cost_for_support_area: 2.5,
            cost_for_support_too_steep: 15.0,
            non_stiffness_cost: 1.,
            stiffness_cost_integration_size: 1.0,
            cone_area_cost: 2.0,
            cone_too_steep_cost: 10.0,
            cost_of_un_feasible_cone: 1e7,
            material_stiffness_settings: MaterialStiffnessSettings::default(),
            node_position_mutation_std: 3.0,
            max_non_stiffness_cost: 1e7,
            collision_volume_voxel_size: 3.,
            collision_volume_offset: 3.,
            collision_check_intervals: 2.,
            collision_penalization: 200.
        }
    }
}

#[derive(Clone, Debug)]
pub struct MaterialStiffnessSettings {
    // diameter * area multiplier = area of the beam
    pub area_multiplier: f32,
    pub e_mod: f32,
    pub g_mod: f32,
    pub jxx: f32,
    pub iy: f32,
    pub iz: f32,
}

impl Default for MaterialStiffnessSettings {
    fn default() -> Self {
        Self {
            area_multiplier: f32::consts::PI * 0.4,
            e_mod: 1500.0,
            g_mod: 500.0,
            jxx: 6.0,
            iy: 3.0,
            iz: 3.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SupportSettings {
    pub voxel_size: f32,
    pub beam_radius: f32,
    pub cone_thickness: f32,
    pub min_cone_thickness_for_hole: f32,
    pub base_cylinder_radius: f32,
    pub base_cylinder_height: f32,
    pub min_cone_radius: f32,
    pub support_detachment: f32,
    pub support_tree_max_overhanging_angle: f32
}

impl Default for SupportSettings {
    fn default() -> Self {
        Self { 
            voxel_size: 0.3,
            beam_radius: 1.5,
            cone_thickness: 0.6,
            min_cone_thickness_for_hole: 0.1,
            base_cylinder_radius: 5.,
            base_cylinder_height: 1.,
            min_cone_radius: 1.,
            support_detachment: 0.2,
            support_tree_max_overhanging_angle: 60.
        }
    }
}
