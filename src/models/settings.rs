use std::f32::consts::PI;

#[derive(Default, Debug, Clone)]
pub struct Settings {
    pub bridge_settings: BridgeSettings
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
