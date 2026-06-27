mod evolution;
mod models;
mod stages;
mod support;

use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Context;
use clap::Parser;
use env_logger::Builder;
use log::{error, info, LevelFilter};

use crate::{
    models::Settings,
    stages::{
        contact_point_optimization::SimpleContactPointOptimizer,
        contact_points_grouping::SimpleContactPointsGrouper,
        criticality_detection::PropagationBasedCriticalityDetector,
        criticality_grouping::DistanceBasedCriticalityGrouper,
        floating_region_detection::AreaBasedFloatingRegionDetector,
        support_structure_optimization::SimpleSupportStructureOptimizer,
        Pipeline,
        PipelineBehaviour,
        StartedState,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Load settings from a JSON file.
    #[arg(long)]
    settings: Option<PathBuf>,

    /// Write the default settings to a JSON file and exit.
    #[arg(long)]
    dump_settings: Option<PathBuf>,
}

fn main() {
    Builder::new()
        .filter_level(LevelFilter::Error)
        .filter_module("evo_strut", LevelFilter::Info)
        .init();

    if let Err(e) = run() {
        error!("{e:?}");
    }
}

fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if let Some(path) = cli.dump_settings {
        let settings = Settings::default();
        let serialized = serde_json::to_string_pretty(&settings)
            .context("failed to serialize default settings")?;
        fs::write(&path, serialized)
            .with_context(|| format!("failed to write settings to {}", path.display()))?;
        return Ok(());
    }

    let settings = match cli.settings {
        Some(path) => {
            let serialized = fs::read_to_string(&path)
                .with_context(|| format!("failed to read settings from {}", path.display()))?;
            serde_json::from_str(&serialized)
                .with_context(|| format!("failed to parse settings from {}", path.display()))?
        }
        None => Settings::default(),
    };

    // writing the settings used
    let serialized =
        serde_json::to_string_pretty(&settings).context("failed to serialize default settings")?;
    fs::write(&settings.io_settings.output_settings_path, serialized).with_context(|| {
        format!(
            "failed to write settings to {}",
            settings.io_settings.output_settings_path
        )
    })?;

    type Behaviour = PipelineBehaviour<
        PropagationBasedCriticalityDetector,
        AreaBasedFloatingRegionDetector,
        DistanceBasedCriticalityGrouper,
        SimpleContactPointOptimizer,
        SimpleContactPointsGrouper,
        SimpleSupportStructureOptimizer,
        // SimpleSupportStructureRefiner,
    >;

    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let to_return = Pipeline::<StartedState, Behaviour>::run(settings);
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let duration = end - start;
    info!("total execution time was: {} [s]", duration.as_secs());
    to_return
}
