use std::collections::HashMap;

use rizzler_rust_and_furious::rizzler::Rizzler;
use robotics_lib::{interface::{Direction, where_am_i, go, get_score, one_direction_view, robot_map, robot_view}, runner::Runnable, world::{tile::{Content, Tile}, World, environmental_conditions::EnvironmentalConditions}, utils::LibError};
use rstykrab_cache::Action; 

use super::{Coord, visualizer_event_listener::{ChannelItem, InterfaceInvocation}, visualizable_robot::Visulizable};

pub struct VisualizableInterfaces {
}

impl VisualizableInterfaces {
    fn send_action<'a>(action:Action, robot: &'a(impl Runnable + Visulizable<'a>), world: &World, riz_message: Option<String>) {
        let (_, (x,y)) = where_am_i(robot, world);
        let channel_item = ChannelItem::InterfaceChannelItem(InterfaceInvocation::new(action, Coord::new(x, y), riz_message));
        robot.borrow_event_listener().sender.send(channel_item).expect("VisualizableInterfaces: sending action failed.");
    }

    /// Given a content to craft, will attempt to craft it from the contents already present in the backpack
    pub fn craft<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &World, content: Content) -> Result<Content, LibError> {
        let (riz_message, res) = Rizzler::craft_with_rizz(robot, content.clone());
        VisualizableInterfaces::send_action(Action::Craft(content), robot, world, Some(riz_message));
        res
    }

    /// destroy	Given the robot, the world and the direction, will destroy the content of the tile in the given direction
    pub fn destroy<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &mut World, direction: Direction) -> Result<usize, LibError> {
        let (riz_message, res) = Rizzler::destroy_with_rizz(robot, world, direction.clone());
        VisualizableInterfaces::send_action(Action::Destroy(direction), robot, world, Some(riz_message));
        res
    }

    // Given a Vec of (x, y) coordinates of the world, the function returns what those tiles are (it discovers them). Discovering each tile costs 3 energy units and it is possible to discover tiles up to 30% of the world’s total dimension
    pub fn discover_tiles<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &mut World, to_discover: &[(usize, usize)]) -> Result<HashMap<(usize, usize), Option<Tile>>, LibError> {
        let (riz_message, res) = Rizzler::discover_tiles_with_rizz(robot, world, to_discover);
        VisualizableInterfaces::send_action(Action::DiscoverTiles(to_discover.to_vec()), robot, world, Some(riz_message));
        res
    }
    
    // Given the world, will return the amount of score received by the robot.
    pub fn get_score<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &World) -> f32 {
        let res = get_score(world);
        VisualizableInterfaces::send_action(Action::GetScore(), robot, world, None);
        res 
    }

    /// go	Given the robot, the world and the direction, will move the robot in the given direction. If it moves itself to a teleport tile, it will be activated
    pub fn go<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &mut World, direction: Direction) -> Result<(Vec<Vec<Option<Tile>>>, (usize, usize)), LibError> {
        let res = go(robot, world, direction.clone());
        VisualizableInterfaces::send_action(Action::Go(direction), robot, world, None);
        res
    }

    /// look_at_sky	Given the world, will return the environmental conditions It’s used to see the weather conditions and the time of day
    pub fn look_at_sky<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &World) -> EnvironmentalConditions {
        let (riz_message, res) = Rizzler::look_at_sky_with_rizz(world);
        VisualizableInterfaces::send_action(Action::LookAtSky(), robot, world, Some(riz_message));
        res
    }

    /// one_direction_view	Given the: robot, world, direction and distance will return a 3xdirection matrix of Tile
    pub fn one_direction_view<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &World, direction: Direction, distance: usize) -> Result<Vec<Vec<Tile>>, LibError> {
        let res = one_direction_view(robot, world, direction.clone(), distance);
        VisualizableInterfaces::send_action(Action::OneDirectionView(direction, distance), robot, world, None);
        res
    }

    /// put	Given the world, will try to put a content from the robot backpack into a target tile
    pub fn put<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &mut World, content_in: Content, quantity: usize, direction: Direction) -> Result<usize, LibError> {
        let (riz_message, res) = Rizzler::put_with_rizz(robot, world, content_in.clone(), quantity, direction.clone());
        VisualizableInterfaces::send_action(Action::Put(content_in, quantity, direction), robot, world, Some(riz_message));
        res
    }

    /// Given the world, will return the map of the robot It’s used as private map for the robot
    pub fn robot_map<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &World) -> Option<Vec<Vec<Option<Tile>>>> {
        let res = robot_map(world);
        VisualizableInterfaces::send_action(Action::RobotMap(), robot, world, None);
        res
    }

    /// Given the world, will return the area around the robot
    pub fn robot_view<'a>(robot: &'a (impl Runnable + Visulizable<'a>), world: &World) -> Vec<Vec<Option<Tile>>> {
        let res = robot_view(robot, world);
        VisualizableInterfaces::send_action(Action::RobotView(), robot, world, None);
        res
    }

    /// Given the robot, the world and the coordinate of a teleport tile, will move the robot in the given tile
    pub fn teleport<'a>(robot: &'a mut (impl Runnable + Visulizable<'a>), world: &mut World, coordinates: (usize, usize)) -> Result<(Vec<Vec<Option<Tile>>>, (usize, usize)), LibError> {
        let (riz_message, res) = Rizzler::teleport_with_rizz(robot, world, coordinates);
        VisualizableInterfaces::send_action(Action::Teleport(coordinates), robot, world, Some(riz_message));
        res
    }

    /// Given the world, will return the area around the robot as a matrix of Option<Tile> with the position of the robot
    pub fn where_am_i<'a>(robot: &'a (impl Runnable + Visulizable<'a>), world: &World) -> (Vec<Vec<Option<Tile>>>, (usize, usize)) {
        let res = where_am_i(robot, world);
        VisualizableInterfaces::send_action(Action::WhereAmI(), robot, world, None);
        res
    }
}
