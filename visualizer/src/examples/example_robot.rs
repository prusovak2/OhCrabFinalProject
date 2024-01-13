use robotics_lib::{runner::{Robot, Runnable}, interface::go};

use crate::visualizer::{visualizable_interfaces::VisualizableInterfaces, visualizer::RobotCreator};

pub struct ExampleRobot{
    properties: Robot,
    tick_counter: usize,
    some_param: i32, // to show how to pass parameter to your robot
    interfaces: VisualizableInterfaces
}

pub struct ExampleRobotFactory {
    some_param: i32,
}

impl ExampleRobotFactory {
    pub fn new(some_param: i32) -> ExampleRobotFactory {
        ExampleRobotFactory{some_param}
    }
}

impl RobotCreator for ExampleRobotFactory {
    fn create(&self, interfaces: VisualizableInterfaces) -> Box<dyn Runnable> {
        let example_robot = ExampleRobot {properties: Robot::new(), tick_counter: 0, some_param:self.some_param, interfaces: interfaces };
        Box::new(example_robot)
    }
}

impl Runnable for ExampleRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        println!("TICK COUNT: {:?}", self.tick_counter);
        self.tick_counter+=1;

        let res = go(self, world, robotics_lib::interface::Direction::Down);
        let res = go(self, world, robotics_lib::interface::Direction::Left);
    }

    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        println!("Random robot received event: {}", event);
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