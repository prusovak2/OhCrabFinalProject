use crate::robot_veronika::distribution_robot::DistributorRobotFactory;
use crate::oh_crab_visualizer::visualizer::visualizer::{OhCrabVisualizerConfig, OhCrabVisualizer, RunMode};
use robotics_lib::world::tile::Content;

pub fn run_distribution_robot(){
    let robot_factory = DistributorRobotFactory::new(vec![Content::Rock(1), Content::Fish(1), Content::Tree(2)]);
    let world_generator = crate::world_gen_utils::load_or_generate_world(20, 420);

    let config = OhCrabVisualizerConfig::new(RunMode::Interactive, false, 500);
    //let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(400), true, 500);
    let mut visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);

    //visualizer.simulate().unwrap();
    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    }
}
