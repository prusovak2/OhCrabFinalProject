use std::env;

use visualizer::oh_crab_visualizer::examples::example::{visualizer_nonteractive, visualizer_interactive, visualizer_smaller,
                                                        distribution_big_simulate, distribution_bigger_viz, distribution_small_viz};

#[derive(Debug)]
enum RunMode {
    VisualizerInteractive,
    VisualizerSimulate,
    VisualizerSmaller,
    DistributionSmall,
    DistributionBig,
    DistributionSimulation,
    Tanya,
}

impl RunMode {
    fn from_str(s: &str) -> Option<RunMode> {
        match s.to_lowercase().as_str() {
            "visualizer-interactive" => Some(RunMode::VisualizerInteractive),
            "visualizer-simulate" => Some(RunMode::VisualizerSimulate),
            "visualizer-smaller" => Some(RunMode::VisualizerSmaller),
            "distribution-small" => Some(RunMode::DistributionSmall),
            "distribution-big" => Some(RunMode::DistributionBig),
            "distribution-simulation" => Some(RunMode::DistributionSimulation),
            "tanya" => Some(RunMode::Tanya),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <mode>", args[0]);
        std::process::exit(1);
    }

    let input_name = &args[1];
    match RunMode::from_str(input_name) {
        Some(RunMode::VisualizerInteractive) => {
            visualizer_interactive();
        }
        Some(RunMode::VisualizerSimulate) => {
            visualizer_nonteractive();
        }
        Some(RunMode::VisualizerSmaller) => {
            visualizer_smaller();
        }
        Some(RunMode::DistributionSmall) => {
            distribution_small_viz();
        }
        Some(RunMode::DistributionBig) => {
            distribution_bigger_viz();
        }
        Some(RunMode::DistributionSimulation) => {
            distribution_big_simulate();
        }
        Some(RunMode::Tanya) => {
            println!("Run your example here.")
        }
        None => eprintln!("Invalid name: {}", input_name),
    }
    //example();
}
