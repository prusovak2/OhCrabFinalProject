use oxagworldgenerator::world_generator::{OxAgWorldGenerator, world_generator_builder::OxAgWorldGeneratorBuilder};
use visualizer::{visualizer::try_ggeui, audio::try_sound};
// use robotics_lib::world::world_generator::Generator;
use visualizer::{mock::*, history_cache::try_cache};

fn main() {
    println!("Hello, world!");
    //try_sound();
    //try_ggeui();
    test_runner_main_loop();
    //try_cache()
}
