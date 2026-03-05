use std::f32::consts::PI;

#[derive(Default, Debug, Clone)]
pub struct Settings {
    /// input output parameters
    pub io_settings: IoSettings,
    /// parameters that decide what constitute a valid bridge
    pub bridge_settings: BridgeSettings,
    /// parameters that define what constitute a "critical" surface
    /// (i.e. a surface that needs supports)
    pub criticality_settings: CriticalitySettings,
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
    pub max_detachment_from_z_plane: f32
}

impl Default for CriticalitySettings {
    fn default() -> Self {
        Self { 
            support_overhanging_angle: 60.,
            max_detachment_from_z_plane: 0.1
        }
    }
}

#[derive(Debug, Clone)]
pub struct BridgeSettings {
    pub max_bridge_length: f32,
    pub valid_bridge_angles: Vec<f32>
}

impl Default for BridgeSettings {
    fn default() -> Self {
        Self {
            max_bridge_length: 30.0,
            valid_bridge_angles: vec![PI/2., PI/4., 0., -PI/4.]
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


