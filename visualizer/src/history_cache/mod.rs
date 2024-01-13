use robotics_lib::{interface::{Direction, Tools, where_am_i, go, craft}, runner::Runnable, world::{tile::{Content, Tile}, World}, utils::LibError};
use rstykrab_cache::{
    Cache,
    Action, Record, CacheError
};

impl Tools for CachedActionTool {}

struct CachedActionTool {
    cache: Cache
}

impl CachedActionTool {
    pub fn new(cache_size: usize) -> CachedActionTool {
        CachedActionTool {cache: Cache::new(cache_size)}
    }

    fn log_action(&mut self, action:Action, robot: &mut impl Runnable, world: &World) {
        let (_, (x,y)) = where_am_i(robot, world);
        self.cache.add_record(action, (x, y));
    }

    pub fn go(&mut self, robot: &mut impl Runnable, world: &mut World, direction: Direction) -> Result<(Vec<Vec<Option<Tile>>>, (usize, usize)), LibError> {
        self.log_action(Action::Go(direction.clone()), robot, world);
        go(robot, world, direction)
    }

    pub fn craft(&mut self, robot: &mut impl Runnable, world: &World, content: Content) -> Result<Content, LibError> {
        self.log_action(Action::Craft(content.clone()), robot, world);
        craft(robot, content)
    }

    /// Gets the list of recent action records, up to the specified count. Returns a CacheError if count is bigger than cache size
    /// 
    pub fn get_recent_actions(&self, count: usize) -> Result<Vec<&Record>, CacheError> {
        self.cache.get_recent_actions(count)
    }

    // TODO:
    // craft	Given a content to craft, will attempt to craft it from the contents already present in the backpack
    // debug	Given the world, will return the map, the dimension and the position of the robot It’s used for debug purposed
    // destroy	Given the robot, the world and the direction, will destroy the content of the tile in the given direction
    // discover_tiles	Given a Vec of (x, y) coordinates of the world, the function returns what those tiles are (it discovers them). Discovering each tile costs 3 energy units and it is possible to discover tiles up to 30% of the world’s total dimension
    // get_score	Given the world, will return the amount of score received by the robot.
    // go	Given the robot, the world and the direction, will move the robot in the given direction. If it moves itself to a teleport tile, it will be activated
    // look_at_sky	Given the world, will return the environmental conditions It’s used to see the weather conditions and the time of day
    // one_direction_view	Given the: robot, world, direction and distance will return a 3xdirection matrix of Tile
    // put	Given the world, will try to put a content from the robot backpack into a target tile
    // robot_map	Given the world, will return the map of the robot It’s used as private map for the robot
    // robot_view	Given the world, will return the area around the robot
    // teleport	Given the robot, the world and the coordinate of a teleport tile, will move the robot in the given tile
    // where_am_i	Given the world, will return the area around the robot as a matrix of Option<Tile> with the position of the robot
}


pub fn try_cache() {
    // Create a new Cache with a buffer size of 10
    let mut history = Cache::new(10);

    // Add a action to the cache
    history.add_record(Action::Go(Direction::Up), (0, 0));

    // Retrieve recent actions
    if let Ok(recent_actions) = history.get_recent_actions(5) {
        println!("Recent Action: {:?}", recent_actions);
    } else {
        println!("Error: Invalid count specified");
    }
}