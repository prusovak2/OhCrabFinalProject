use std::collections::HashMap;
use rand::rngs::StdRng;

use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use robotics_lib::world::world_generator::Generator;
use strum::IntoEnumIterator;
use robotics_lib::world::tile::*;
use robotics_lib::world::environmental_conditions::*;


pub const SOME_FIXED_SEED: [u8; 32] = [42; 32];

pub(crate) struct MockWorldGenerator {
    world_dimension: usize,
    enviromental_conditions_option: Option<EnvironmentalConditions>,
    rng: StdRng,
    content_upper_bound: usize,
    elevation_upper_bound: usize,

    required_contents: Option<HashMap<Content, f32>>,
    required_tile_types: Option<HashMap<TileType, f32>>,

    tile_types: Vec<TileType>,
    content_types: Vec<Content>
}

impl MockWorldGenerator {
    /// Creates new MockWorldGenerator
    /// 
    /// # Arguments
    /// world_dimension: size of the world grid taht should be genrated
    /// content_upper_bound: max amount of content that will be places on tiles
    /// elevation_upper_bound: max elevation of a tile possible in the world
    /// enviromental_conditions: option to specify env conditions explicitely (could be useful for Tanya), if None is passed env conditions are generated
    /// 
    pub fn new(
            world_dimension: usize,
            content_upper_bound: usize,
            elevation_upper_bound: usize,
            required_contents: Option<HashMap<Content, f32>>,
            required_tile_types: Option<HashMap<TileType, f32>>,
            enviromental_conditions: Option<EnvironmentalConditions>,
            seed: Option<[u8; 32]>) 
        -> Result<MockWorldGenerator, String> {
        // create rng
        let rng: StdRng;
        match seed {
            Some(s) => {
                rng = rand::SeedableRng::from_seed(s);
            }
            None => {
                rng = rand::SeedableRng::from_entropy();
            }
        }

        let mut percentage_sum:f32 = 0 as f32;
        if let Some(content_map) = &required_contents {
            for (_, percentage) in content_map.iter(){
                percentage_sum += percentage;
            }
        }

        if let Some(tile_map) = &required_tile_types {
            for (_, percentage) in tile_map.iter(){
                percentage_sum += percentage;
            }
        }

        if percentage_sum > 1 as f32{
            return Err("Sum of percentages of required contents and tile types must be at most 1 (100 %)".to_owned());
        }

        Ok(MockWorldGenerator { 
            world_dimension: world_dimension,
            enviromental_conditions_option: enviromental_conditions,
            rng: rng,
            content_upper_bound: content_upper_bound,
            elevation_upper_bound: elevation_upper_bound,
            required_contents:required_contents,
            required_tile_types: required_tile_types,
            tile_types: TileType::iter().collect(),
            content_types: Content::iter().collect()
        })
    }
}

impl Generator for MockWorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,  // I really don't understand why the return type is this shit and not a (World, Coordinate)
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<std::collections::HashMap<Content, f32>>,
    ) {
        // generate robot starting position
        let robot_origin = self.world_dimension /2; // for now place robot to the center of the world

        // generate tiles
        let total_tiles = self.world_dimension* self.world_dimension;
        let mut all_tiles:Vec<Tile> = Vec::with_capacity(total_tiles);
        let mut grass_counter = 0;
        let mut required_tiles_counter = 0;

        // add required contents
        let req_cont =self.required_contents.clone();
        if let Some(content_map) = req_cont{
            for (content, percentage) in content_map.iter(){
                let amount_tiles = ((total_tiles as f32) * percentage).floor() as usize;
                let tile_type = match &content {
                    Content::Fish(_) => TileType::ShallowWater, // fish cannot be placed on the grass
                    _ => {
                        grass_counter += 1; 
                        TileType::Grass // everything else can 
                    } 
                };
                for _ in 0..amount_tiles {
                    let amount = (0..self.content_upper_bound).choose(&mut self.rng).unwrap();
                    let tile = self.create_tile(tile_type, content.to_value(amount));
                    required_tiles_counter+=1;
                    all_tiles.push(tile);
                }
            }
        }

        // generate required tile types
        let req_tiles =self.required_tile_types.clone();
        if let Some(tile_map) = req_tiles {
            for (tile_type, percentage) in tile_map.iter() {
                let mut amount_tiles = ((total_tiles as f32) * *percentage).floor() as usize;
                if let TileType::Grass = tile_type {
                    amount_tiles = usize::max(amount_tiles - grass_counter, 0);
                }
                for _ in 0..amount_tiles {
                    let tile = self.create_tile_by_type(tile_type.clone());
                    required_tiles_counter+=1;
                    all_tiles.push(tile);
                }
            }
        }

        for _ in 0..(total_tiles-required_tiles_counter) {
            let tile = self.generate_tile();
            all_tiles.push(tile);
        }

        all_tiles.shuffle(&mut self.rng);

        let mut world_map:  Vec<Vec<robotics_lib::world::tile::Tile>> = Vec::with_capacity(self.world_dimension);
        for i in 0..self.world_dimension {
            let mut inner_vec: Vec<Tile> = Vec::with_capacity(self.world_dimension);
            for j in 0..self.world_dimension {
                let tile: Tile;
                // place walkable tile on the coordinates where a robot is gonna be generated
                if i == robot_origin && j == robot_origin {
                    tile = Tile {
                        tile_type: TileType::Grass,
                        content: Content::None,
                        elevation: 1
                    }
                }
                else{
                    tile = all_tiles.pop().unwrap()
                }

                inner_vec.push(tile)
            }
            world_map.push(inner_vec);
        }

        // generate environtmental conditions
        let environmental_conditions: EnvironmentalConditions;
        match &self.enviromental_conditions_option {
            Some(env) => {
                environmental_conditions = env.clone()
            } 
            None => {
                environmental_conditions = MockWorldGenerator::generate_env_conditions();
            },
        }

        // we don't care about score
        let max_score = 100 as f32;
        let score_table:Option<HashMap<Content, f32>> = None;

        (world_map, (robot_origin, robot_origin), environmental_conditions, max_score, score_table)
    }
}

impl MockWorldGenerator {
    fn generate_tile(&mut self) -> Tile {
        let tile_type = self.generate_tile_type();
        let tile = self.create_tile_by_type(tile_type);
        tile
    }

    fn create_tile_by_type(&mut self, tile_type: TileType) -> Tile {
        let props = tile_type.properties();
        let content_type = self.generate_content_type();

        let mut content = Content::None;
        if props.can_hold(&content_type){
            let amount = (0..self.content_upper_bound).choose(&mut self.rng).unwrap();
            content = content_type.to_value(amount);
        }
        let tile = self.create_tile(tile_type, content);
        tile
    }

    fn create_tile(&mut self, tile_type: TileType, content: Content) -> Tile {
        Tile {
            tile_type: tile_type,
            content: content,
            elevation: (0..self.elevation_upper_bound).choose(&mut self.rng).unwrap()
        }
    }

    fn generate_tile_type(&mut self) -> TileType {
        self.tile_types.choose(&mut self.rng).unwrap().clone()
    }

    fn generate_content_type(&mut self) -> Content {
        self.content_types.choose(&mut self.rng).unwrap().clone()
    }
    
    fn generate_env_conditions() -> EnvironmentalConditions {
       let weathers = vec![WeatherType::Sunny, WeatherType::Rainy, WeatherType::Foggy,WeatherType::TropicalMonsoon, WeatherType::TrentinoSnow];
       let res = EnvironmentalConditions::new(&weathers, 10, 0);
       res.unwrap() // this should not panic
    }
}
