use std::{fs, io::Read};
use robotics_lib::world::tile::Content;
use oxagworldgenerator::world_generator::content_options::OxAgContentOptions;
use oxagworldgenerator::world_generator::tile_type_options::OxAgTileTypeOptions;
use oxagworldgenerator::world_generator::{OxAgWorldGenerator, world_generator_builder::OxAgWorldGeneratorBuilder};

fn non_empty_file_exists(filename: &str) -> bool {
    if let Ok(metadata) = fs::metadata(filename) {
        if metadata.is_file() {
            if let Ok(mut file) = fs::File::open(filename) {
                let mut buffer = String::new();
                if let Ok(_) = file.read_to_string(&mut buffer) {
                    return !buffer.is_empty();
                }
            }
        }
    }
    false
}

fn build_path(size: usize, seed: u64) -> String {
    format!(".\\generated_worlds\\size{size}_seed{seed}.json")
}

pub fn generate_and_save_world(size: usize, seed: u64) {
    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_content_options(MARKET()).expect("REASON")
        .set_tile_type_options(MARKET_WORLD).expect("REASON")
                //.set_tile_type_options_from_preset(OxAgTileTypePresets::LowWaterWorld)
        .build()
        .unwrap();

    generator.save(&build_path(size, seed)).unwrap();
}

pub fn load_world(size: usize, seed: u64) -> OxAgWorldGenerator {
    let generator = OxAgWorldGeneratorBuilder::new()
        .load(&format!(".\\generated_worlds\\size{size}_seed{seed}.json"))
        .unwrap();
    //let _tmp = generator.gen().0;
    generator
}

pub fn load_or_generate_world(size: usize, seed: u64) -> OxAgWorldGenerator {
    let path = build_path(size, seed);
    if non_empty_file_exists(&path) {
        println!("Loading world, size: {}, seed {}", size, seed);
        load_world(size, seed)
    }
    else {
        println!("Generating world, size: {}, seed {}", size, seed);
        generate_and_save_world(size, seed);
        load_world(size, seed)
    }
}

const MARKET_WORLD: OxAgTileTypeOptions = OxAgTileTypeOptions {
    deep_water_level: -1.0..=-0.9,
    shallow_water_level: -0.9..=-0.5,
    sand_level: -0.5..=-0.3,
    grass_level: -0.3..=0.2,
    hill_level: 0.2..=0.5,
    mountain_level: 0.5..=0.8,
    snow_level: 0.8..=1.0,
    river_n: 1..=10,
    street_n: 1..=10,
    street_len: 10..=15,
    lava_n: 0..=3,
    lava_radius: 2..=5,
};

const MARKET: fn() -> Vec<(Content, OxAgContentOptions)> = || {
    Vec::from([
        (
            Content::Rock(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 20,
                max_radius: 0,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.2,
            },
        ),
        (
            Content::Tree(0),
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 3,
                max_radius: 10,
                with_max_spawn_number: false,
                max_spawn_number: 100,
                percentage: 0.3,
            },
        ),
        (
            Content::Garbage(0),
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 2,
                max_radius: 2,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.01,
            },
        ),
        (
            Content::Fire,
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 2,
                max_radius: 2,
                with_max_spawn_number: true,
                max_spawn_number: 6,
                percentage: 0.04,
            },
        ),
        (
            Content::Coin(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 2,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.2,
            },
        ),
        (
            Content::Bin(0..0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.01,
            },
        ),
        (
            Content::Crate(0..0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 1,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.04,
            },
        ),
        (
            Content::Bank(0..0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.1,
            },
        ),
        (
            Content::Market(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.2,
            },
        ),
        (
            Content::Water(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 4,
                max_radius: 1,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.07,
            },
        ),
        (
            Content::Fish(0),
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 3,
                max_radius: 4,
                with_max_spawn_number: false,
                max_spawn_number: 100,
                percentage: 0.2,
            },
        ),
    ])};