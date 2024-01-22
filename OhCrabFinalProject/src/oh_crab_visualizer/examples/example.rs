use crate::oh_crab_visualizer::visualizer::visualizer::{OhCrabVisualizerConfig, OhCrabVisualizer, RunMode};
use robotics_lib::world::tile::Content;
use super::example_robot::ExampleRobotFactory;
use crate::robot_veronika::distribution_robot::DistributorRobotFactory;

pub fn visualizer_nonteractive(){
    let robot_factory = ExampleRobotFactory::new(42);
    let world_generator = crate::world_gen_utils::load_or_generate_world(256, 420);
    //let world_generator = crate::world_gen_utils::load_or_generate_world(15, 42);

    let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(400), true);
    let visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);

    //visualizer.simulate().unwrap();
    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    }
}

pub fn visualizer_interactive(){
    let robot_factory = ExampleRobotFactory::new(42);
    let world_generator = crate::world_gen_utils::load_or_generate_world(256, 420);
    //let world_generator = crate::world_gen_utils::load_or_generate_world(15, 42);

    let config = OhCrabVisualizerConfig::new(RunMode::Interactive, true);
    let visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);
    
    //visualizer.simulate().unwrap();
    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    } 
}

pub fn visualizer_smaller(){
    let robot_factory = ExampleRobotFactory::new(42);
    let world_generator = crate::world_gen_utils::load_or_generate_world(128, 420);

    let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(400), false);
    let visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);

    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    }
}

pub fn distribution_small_viz(){
    let robot_factory = DistributorRobotFactory::new(vec![Content::Rock(1).index(), Content::Fish(1).index(), Content::Tree(1).index()]);
    let world_generator = crate::world_gen_utils::load_or_generate_world(20, 420);


    let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(500), true);
    let visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);

    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    }
}

pub fn distribution_bigger_viz(){
    let robot_factory = DistributorRobotFactory::new(vec![Content::Rock(1).index(), Content::Fish(1).index(), Content::Tree(1).index()]);
    let world_generator = crate::world_gen_utils::load_or_generate_world(40, 420);


    let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(500), false);
    let visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);

    match visualizer.run() {
        Ok(_) => {}
        Err(err) => println!("Visualizer run returned error {:?}", err),
    }
}

pub fn distribution_big_simulate(){
    let robot_factory = DistributorRobotFactory::new(vec![Content::Rock(1).index(), Content::Fish(1).index(), Content::Tree(1).index()]);
    let world_generator = crate::world_gen_utils::load_or_generate_world(128, 420);


    let config = OhCrabVisualizerConfig::new(RunMode::NonInteractive(500), false);
    let mut visualizer = OhCrabVisualizer::new(robot_factory, world_generator, config);

    visualizer.simulate().unwrap();
}