use oxagworldgenerator::world_generator;

use crate::visualizer::visualizer::OhCrabVisualizer;

use super::example_robot::ExampleRobotFactory;

pub fn example(){
    let robot_factory = ExampleRobotFactory::new(42);
    let world_generator = crate::world_gen_utils::load_or_generate_world(15, 42);

    let visualizer = OhCrabVisualizer::new(robot_factory, world_generator);
}