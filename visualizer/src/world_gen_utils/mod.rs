use std::{fs, io::Read};

use oxagworldgenerator::world_generator::{OxAgWorldGenerator, world_generator_builder::OxAgWorldGeneratorBuilder, presets::tile_type_presets::OxAgTileTypePresets};

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
        // .set_content_options_from_preset(OxAgContentPresets::Default)
                .set_tile_type_options_from_preset(OxAgTileTypePresets::WaterWorld)
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