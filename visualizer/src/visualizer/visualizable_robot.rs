use std::sync::mpsc::Sender;
use oxagaudiotool::OxAgAudioTool;
use robotics_lib::{runner::Runnable, interface::debug, event::events::Event as RobotEvent, world::tile::Tile};

use crate::audio::get_configured_audio_tool;

use super::Coord;

pub(crate) struct VisualizableRobot {
    real_robot: Box<dyn Runnable>,
    audio_tool: Option<OxAgAudioTool>,
    sender: Sender<RobotChannelItem>,
}

impl VisualizableRobot {
    pub(crate) fn new(real_robot: Box<dyn Runnable>, sender: Sender<RobotChannelItem>, use_sound: bool) -> VisualizableRobot {
        VisualizableRobot {
            real_robot: real_robot,
            audio_tool: if use_sound { Some(VisualizableRobot::get_configured_audio_tool())} else {None},
            sender: sender
        }
    }
}

impl Runnable for VisualizableRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        self.update_state(world); //TODO: can I change this to init_state and only call it once to send map of tiles and solve the rest by sending events and interface invocations via chanels?
        self.real_robot.process_tick(world)
    }

    fn handle_event(&mut self, event: RobotEvent) {
        self.send_event(event.clone());
        self.play_audio_based_on_event(&event);
        self.real_robot.handle_event(event)
    }

    fn get_energy(&self) -> &robotics_lib::energy::Energy {
        self.real_robot.get_energy()
    }

    fn get_energy_mut(&mut self) -> &mut robotics_lib::energy::Energy {
        self.real_robot.get_energy_mut()
    }

    fn get_coordinate(&self) -> &robotics_lib::world::coordinates::Coordinate {
        self.real_robot.get_coordinate()
    }

    fn get_coordinate_mut(&mut self) -> &mut robotics_lib::world::coordinates::Coordinate {
        self.real_robot.get_coordinate_mut()
    }

    fn get_backpack(&self) -> &robotics_lib::runner::backpack::BackPack {
        self.real_robot.get_backpack()
    }

    fn get_backpack_mut(&mut self) -> &mut robotics_lib::runner::backpack::BackPack {
        self.real_robot.get_backpack_mut()
    }
}

impl VisualizableRobot {
    fn play_audio_based_on_event(&mut self, event: &RobotEvent) {
        if let Some(audio_tool) = &mut self.audio_tool {
            let audio_res = audio_tool.play_audio_based_on_event(&event);
            match audio_res {
                Ok(_) => {},
                Err(err) => println!("Audio tool error: {}", err),
            }
        }
    }

    fn get_configured_audio_tool() -> OxAgAudioTool {
        let audio_res = get_configured_audio_tool();
        match audio_res {
            Ok(tool) => tool,
            Err(err) => panic!("Audio tool error: {}", err),
        }
    }

    fn update_state(&self, world: &mut robotics_lib::world::World) {
        let (map, _, (robot_y, robot_x)) = debug(self, world);
        // println!("{:?}", map);
        // println!("ROBOT SENDING {:?}", (robot_x, robot_y));
        let world_state = WorldMap::new(map, (robot_x, robot_y));
        self.sender.send(RobotChannelItem::Map(world_state)).expect("Sending state from robot to visualizer failed");
    }

    fn send_event(&self, event: RobotEvent) {
        println!("Robot event: {:?}", event);
        let channel_item = RobotChannelItem::RobotEventItem(event.clone());
        self.sender.send(channel_item).expect(&format!("VisualizableRobot: sending event {} failed.", event));
    }
}


pub(crate) enum RobotChannelItem {
    RobotEventItem(RobotEvent),
    Map(WorldMap)
}

pub(crate) struct WorldMap {
    world_map: Vec<Vec<Tile>>,
    robot_position: Coord
}

impl WorldMap {
    pub(crate) fn new(world_map: Vec<Vec<Tile>>, (robot_x, roboy_y): (usize, usize)) -> WorldMap {
        WorldMap { world_map: world_map, robot_position: Coord { x: robot_x, y: roboy_y } }
    }
}