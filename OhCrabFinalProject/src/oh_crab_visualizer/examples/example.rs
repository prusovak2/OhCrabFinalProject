use crate::oh_crab_visualizer::visualizer::visualizer::{OhCrabVisualizerConfig, OhCrabVisualizer, RunMode};

use super::example_robot::ExampleRobotFactory;

pub fn example(){
    let robot_factory = ExampleRobotFactory::new(42);
    let world_generator = crate::world_gen_utils::load_or_generate_world(256, 420);
    //let world_generator = crate::world_gen_utils::load_or_generate_world(15, 42);

    //let config = OhCrabVisualizerConfig::new(RunMode::Interactive, true, 500);
    let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(400), true, 500);
    let mut visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);
    
    //visualizer.simulate().unwrap();
    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    } 
}