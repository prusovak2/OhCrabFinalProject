use robotics_lib::interface::Tools;
use robotics_lib::interface::debug;
use robotics_lib::interface::go;
use robotics_lib::runner::Robot;
use robotics_lib::runner::Runnable;
use std::sync::mpsc::Sender;

use super::mock_visualizer::ChannelItem;
use super::mock_visualizer::WorldMap;
use robotics_lib::event::events::Event;

pub(crate) struct MockRobot{
    properties: Robot,
    tick_counter: usize,
    sender: Sender<ChannelItem>,
}

pub(crate) struct MockTool;
impl Tools for MockTool {}

impl MockRobot {
    pub(crate) fn new(sender: Sender<ChannelItem>) -> MockRobot {
        MockRobot { properties: Robot::new(), tick_counter: 0, sender: sender } 
    }

    fn update_state(&self, world: &mut robotics_lib::world::World) {
        let (map, _, (robot_y, robot_x)) = debug(self, world);
        // println!("{:?}", map);
        // println!("ROBOT SENDING {:?}", (robot_x, robot_y));
        let world_state = WorldMap::new(map, (robot_x, robot_y));
        self.sender.send(ChannelItem::Map(world_state)).expect("Sending state from robot to visualizer failed");
    }
}

impl Runnable for MockRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        self.update_state(world);
        println!("TICK COUNT: {:?}", self.tick_counter);
        self.tick_counter+=1;

        let res = go(self, world, robotics_lib::interface::Direction::Down);
        match res {
            Ok(tile_matrix) => {
                println!("New robot position: {:?}", tile_matrix.1)
            }
            Err(lib_err) => println!("{:?}", lib_err)
        }
        let res = go(self, world, robotics_lib::interface::Direction::Left);
        match res {
            Ok(tile_matrix) => {
                println!("New robot position: {:?}", tile_matrix.1)
            }
            Err(lib_err) => println!("{:?}", lib_err)
        }
        //let _ = CollectTool::collect_content(self, world, &search_content);
        // let one_direction_result = one_direction_view(self, world, robotics_lib::interface::Direction::Up,
        //                                               5);
        // let (map, world_dimension, (robot_x, robot_y)) = debug(self, world);
        // //println!("WORLD: {:?}", map);
    }

    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        match &event {
            // robotics_lib::event::events::Event::Ready => todo!(),
            // robotics_lib::event::events::Event::Terminated => todo!(),
            // robotics_lib::event::events::Event::TimeChanged(_) => todo!(),
            // robotics_lib::event::events::Event::DayChanged(_) => todo!(),
            // robotics_lib::event::events::Event::EnergyRecharged(_) => todo!(),
            // robotics_lib::event::events::Event::EnergyConsumed(_) => todo!(),
            Event::Moved(tile, (robot_y, robot_x)) => {  // BEWARE: library has x and y switched in Move event
                println!("ROBOT SENDING MOVE {:?}", (robot_x, robot_y));
                self.sender.send(ChannelItem::RobotEventItem(Event::Moved(tile.clone(), (*robot_x, *robot_y)))).expect("MOVE send failed") //TODO: remove useless tile clone
            },
            robotics_lib::event::events::Event::TileContentUpdated(tile, (tile_x, tile_y)) => {
                println!("TILE CONTENT UPDATE ({:?}, {:?})", tile_x, tile_y);
                self.sender.send(ChannelItem::RobotEventItem(Event::TileContentUpdated(tile.clone(), (*tile_y, *tile_x)))).expect("CONTENT UPDATE send failed");
            }
            robotics_lib::event::events::Event::AddedToBackpack(content, amount) => {
                println!("ROBOT ADDING TO BACKPACK {:?} items of {:?}", amount, content);
                self.sender.send(ChannelItem::RobotEventItem(Event::AddedToBackpack(content.clone(), *amount))).expect("ADD TO BACKPACK send failed");
            }
            // robotics_lib::event::events::Event::RemovedFromBackpack(content, amount) => todo!(),
            _ => { }
        }
        println!("{:?}", event);
    }

    fn get_energy(&self) -> &robotics_lib::energy::Energy {
        &self.properties.energy
    }

    fn get_energy_mut(&mut self) -> &mut robotics_lib::energy::Energy {
        &mut self.properties.energy
    }

    fn get_coordinate(&self) -> &robotics_lib::world::coordinates::Coordinate {
        & self.properties.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut robotics_lib::world::coordinates::Coordinate {
        &mut self.properties.coordinate
    }

    fn get_backpack(&self) -> &robotics_lib::runner::backpack::BackPack {
        &self.properties.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut robotics_lib::runner::backpack::BackPack {
        &mut self.properties.backpack
    }
}

const DESTROY_VEC : [&str;9] = ["Can i catch you? You look like an exception",
"Baby, if they made you in java you'd be the object of my desire",
"Are you a borrow checker? Because you are making sure there is no ownership conflicts in my heart",
"Are you a wrecking ball? Because you just knocked down all my defenses",
"Are you a demolition expert? Because you just demolished the walls around my heart",
"Did you play Jenga as a kid? Because you just pulled out the last piece, and now I'm falling for you",
"Are you a superhero? Because you just crashed through the walls of my ordinary life",
"Is your name Dynamite? Because you've just blasted your way into my heart",
"Are you a supernova? Because you just exploded into my universe, and now everything revolves around you"];