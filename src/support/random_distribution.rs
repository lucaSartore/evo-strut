#[derive(Debug, Clone)]
pub enum RandomDistribution {
    /// A flat distribution between low (inclusive) and high (exclusive)
    InRange { low: f32, high: f32 },
    
    /// The classic bell curve
    Normal { mean: f32, std_dev: f32 },
    
    /// A bell curve with a "lean" or "tail"
    SkewNormal { mean: f32, std_dev: f32, shape: f32 },
}
