use std::sync::mpsc::Sender;
use robotics_lib::{runner::Runnable, interface::debug, event::events::Event as RobotEvent};


use super::{visualizer::WorldMap, visualizer_event_listener::VisualizerEventListener};

// trait RunnableVisualizable<'a>: Runnable + Visalizable<'a> {
// }

pub trait Visulizable<'a> {
    fn borrow_event_listener(&'a self) -> &'a VisualizerEventListener;
}

pub trait RobotCreator {
    fn create(&self, event_listener: VisualizerEventListener) -> Box<dyn Runnable>;
}

pub(crate) struct VisualizableRobot {
    real_robot: Box<dyn Runnable>,
    map_sender: Sender<MapChannelItem>,
    is_initialized: bool
}

impl VisualizableRobot {
    pub(crate) fn new(real_robot: Box<dyn Runnable>, map_sender: Sender<MapChannelItem>) -> VisualizableRobot {
        VisualizableRobot {
            real_robot: real_robot,
            map_sender,
            is_initialized: false
        }
    }
}

impl Runnable for VisualizableRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        self.init_state(world);
        self.real_robot.process_tick(world)
    }

    fn handle_event(&mut self, event: RobotEvent) {
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
    fn init_state(&mut self, world: &mut robotics_lib::world::World) {
        if !self.is_initialized {
            let (map, _, (robot_y, robot_x)) = debug(self, world);
            println!("VISUALIZABLE ROBOT SENDING ON POSITION {:?} SENDING WORLD MAP", (robot_x, robot_y));
            println!("{:?}", map);
            let world_state = WorldMap::new(map, (robot_x, robot_y));
            self.map_sender.send(MapChannelItem { map: world_state }).expect("Sending state from robot to visualizer failed");
            self.is_initialized = true
        }
    }
}

pub(crate) struct MapChannelItem {
    pub(crate) map: WorldMap
}