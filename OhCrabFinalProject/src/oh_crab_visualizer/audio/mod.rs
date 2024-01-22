use std::collections::HashMap;
use robotics_lib::event::events::Event;
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::{Content, TileType};
use oxagaudiotool::sound_config::OxAgSoundConfig;
use oxagaudiotool::OxAgAudioTool;
use oxagaudiotool::error::error::OxAgAudioToolError;

pub fn get_configured_audio_tool() -> Result<OxAgAudioTool, OxAgAudioToolError>  {
    println!("Loading sounds...");

    let mut events = HashMap::new();
    events.insert(Event::Ready, OxAgSoundConfig::new("assets/audio/event/event_ready.ogg"));
    events.insert(Event::Terminated, OxAgSoundConfig::new("assets/audio/event/event_terminated.ogg"));
    events.insert(Event::EnergyRecharged(0), OxAgSoundConfig::new_with_volume("assets/audio/event/event_energy_recharged.ogg", 0.1));
    events.insert(Event::AddedToBackpack(Content::None, 0), OxAgSoundConfig::new("assets/audio/event/event_add_to_backpack.ogg"));
    events.insert(Event::RemovedFromBackpack(Content::None, 0), OxAgSoundConfig::new("assets/audio/event/event_remove_from_backpack.ogg"));

    let mut tiles = HashMap::new();
    tiles.insert(TileType::DeepWater, OxAgSoundConfig::new("assets/audio/tile/tile_water.ogg"));
    tiles.insert(TileType::ShallowWater, OxAgSoundConfig::new("assets/audio/tile/tile_water.ogg"));
    tiles.insert(TileType::Sand, OxAgSoundConfig::new("assets/audio/tile/tile_sand.ogg"));
    tiles.insert(TileType::Grass, OxAgSoundConfig::new("assets/audio/tile/tile_grass.ogg"));
    tiles.insert(TileType::Hill, OxAgSoundConfig::new("assets/audio/tile/tile_grass.ogg"));
    tiles.insert(TileType::Mountain, OxAgSoundConfig::new("assets/audio/tile/tile_mountain.ogg"));
    tiles.insert(TileType::Snow, OxAgSoundConfig::new("assets/audio/tile/tile_snow.ogg"));
    tiles.insert(TileType::Lava, OxAgSoundConfig::new("assets/audio/tile/tile_lava.ogg"));
    tiles.insert(TileType::Teleport(false), OxAgSoundConfig::new("assets/audio/tile/tile_teleport.ogg"));
    tiles.insert(TileType::Street, OxAgSoundConfig::new("assets/audio/tile/tile_street.ogg"));

    let mut weather = HashMap::new();
    weather.insert(WeatherType::Rainy, OxAgSoundConfig::new("assets/audio/weather/weather_rainy.ogg"));
    weather.insert(WeatherType::Foggy, OxAgSoundConfig::new("assets/audio/weather/weather_foggy.ogg"));
    weather.insert(WeatherType::Sunny, OxAgSoundConfig::new("assets/audio/weather/weather_sunny.ogg"));
    weather.insert(WeatherType::TrentinoSnow, OxAgSoundConfig::new("assets/audio/weather/weather_winter.ogg"));
    weather.insert(WeatherType::TropicalMonsoon, OxAgSoundConfig::new("assets/audio/weather/weather_tropical.ogg"));

    // Create the audio tool
    OxAgAudioTool::new(events, tiles, weather)
}

// pub fn load_background_music() -> OxAgSoundConfig {
//     let background_music = OxAgSoundConfig::new_looped_with_volume("assets/audio/music.ogg", 2.0);
//     background_music
// }