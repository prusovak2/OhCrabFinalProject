use robotics_lib::{event::events::Event as RobotEvent, world::tile::Tile};


// /// Coordinate struct from robotic-lib does not allow for its instances to be created
struct Coord {
    x: usize, 
    y: usize
}

pub enum ChannelItem {
    RobotEventItem(RobotEvent),
    Map(WorldMap)
}

pub struct WorldMap {
    world_map: Vec<Vec<Tile>>,
    robot_position: Coord
}

impl WorldMap {
    pub fn new(world_map: Vec<Vec<Tile>>, (robot_x, roboy_y): (usize, usize)) -> WorldMap {
        WorldMap { world_map: world_map, robot_position: Coord { x: robot_x, y: roboy_y } }
    }
}