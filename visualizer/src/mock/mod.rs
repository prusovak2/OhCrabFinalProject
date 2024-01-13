pub mod mock_visualizer;
pub mod mock_robot;
pub mod mock_world_generator;

use std::collections::HashMap;
use std::sync::mpsc;
use std::time::Instant;

use ggez::event;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use robotics_lib::runner::Runner;
use mock_world_generator::MockWorldGenerator;
use mock_robot::MockRobot;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::world_generator::Generator;


use self::mock_visualizer::ChannelItem;
use self::mock_visualizer::MockVisualizer;

const NUM_TICKS: usize = 8;

fn create_mock_world_gen() -> MockWorldGenerator {
    let mut required_tiles:HashMap<TileType, f32> = HashMap::new();
    required_tiles.insert(TileType::Grass, 0.4);
    required_tiles.insert(TileType::Street, 0.4);
    required_tiles.insert(TileType::Sand, 0.05);
   // required_tiles.insert(TileType::Lava, 0.05);
    //required_tiles.insert(TileType::Mountain, 0.05);
    //required_tiles.insert(TileType::DeepWater, 0.1);

    let mut required_contents:HashMap<Content, f32> = HashMap::new();
    required_contents.insert(Content::Coin(3), 0.9);
    //required_contents.insert(Content::None, 0.2);
    //required_contents.insert(Content::Fire, 0.05);
    required_contents.insert(Content::Bank(0..1), 0.05);
    //required_contents.insert(Content::Fish(3), 0.1);

    let seed: Option<[u8; 32]> = Some([43; 32]);

    let world_generator = MockWorldGenerator::new(128, 10, 10, None, Some(required_tiles), None, seed).unwrap();
    world_generator
}

pub fn test_runner_main_loop() {
    crate::world_gen_utils::load_or_generate_world(15, 42);
    let mut generator = crate::world_gen_utils::load_world(15, 42);
    //let mut generator = create_mock_world_gen();

    let (sender, receiver) = mpsc::channel::<ChannelItem>();

    let robot = MockRobot::new(sender);

    let runner: Runner = Runner::new(Box::new(robot), &mut generator).unwrap();

    let mock_visualizer = MockVisualizer::new(runner, receiver, NUM_TICKS, 500);

    let context_builder = ggez::ContextBuilder::new("pile-of-shit", "xxx")
        .window_mode(ggez::conf::WindowMode::default()
        .resizable(true)
        .maximized(true));
    let (ctx, event_loop) = context_builder.build().unwrap();

    event::run(ctx, event_loop, mock_visualizer);

    // println!("Starting the main LOOP:");
    // for _ in 0..NUM_TICKS {
    //     runner.game_tick().unwrap(); // it seems that with current implementation of game_tick() this should never panic?
    // }
}