use OhCrab_collection::collection::CollectTool;
use robotics_lib::{runner::{Robot, Runnable}, interface::Direction, world::tile::Content};

use crate::{oh_crab_visualizer::visualizer::{visualizable_interfaces::VisualizableInterfaces, visualizable_robot::{RobotCreator, Visulizable}, visualizer_event_listener::VisualizerEventListener}, println_d};

pub struct ExampleRobot{
    properties: Robot,
    tick_counter: usize,
    some_param: i32, // to show how to pass parameter to your robot
    visualizer_event_listener: VisualizerEventListener
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
    fn create(&self, data_sender: VisualizerEventListener) -> Box<dyn Runnable> {
        let example_robot = ExampleRobot {properties: Robot::new(), tick_counter: 0, some_param:self.some_param, visualizer_event_listener: data_sender };
        Box::new(example_robot)
    }
}

impl<'a> Visulizable<'a> for ExampleRobot {
    fn borrow_event_listener(&'a self) -> &'a VisualizerEventListener{
        &self.visualizer_event_listener
    }
}

impl ExampleRobot {
    fn int_to_direction(number: i32) -> Direction {
        let modulo =  number % 4;
        match modulo {
            0 => Direction::Left,
            1 => Direction::Down,
            2 => Direction::Up,
            3 => Direction::Right,
            _ => panic!("Logic error: modulo 4")
        }
    }

    fn get_direction(&self) -> Direction {
        ExampleRobot::int_to_direction(self.some_param)
    }

    fn change_direction(&mut self) {
        self.some_param +=1;
    }
}

impl Runnable for ExampleRobot {
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        println_d!("TICK COUNT: {:?}", self.tick_counter);
        self.tick_counter+=1;

        match VisualizableInterfaces::put(self, world, Content::Water(0), 2, Direction::Right) {
            Ok(_) => {}
            Err(error) => println!("Example robot: put error {:?}", error)
        } 

        match VisualizableInterfaces::put(self, world, Content::Coin(20), 2, Direction::Right) {
            Ok(_) => {}
            Err(error) => println!("Example robot: put error {:?}", error)
        } 

        let _: robotics_lib::world::environmental_conditions::EnvironmentalConditions = VisualizableInterfaces::look_at_sky(self, world);

        match VisualizableInterfaces::go(self, world, self.get_direction()) {
            Ok((_, (_, _))) => { }
            Err(_) => {
                self.change_direction();
                println!("Example robot: changing direction: {:?}", self.get_direction())
            }
        }

        // match CollectTool::collect_instantly_reachable(self, world, &Content::JollyBlock(0)){
        //     Ok(_) => {}
        //     Err(error) => println!("Example robot: collect error {:?}", error)
        // }

        // match CollectTool::collect_instantly_reachable(self, world, &Content::Fish(1)){
        //     Ok(_) => {}
        //     Err(error) => println!("Example robot: collect error {:?}", error)
        // }

        match CollectTool::collect_instantly_reachable(self, world, &Content::Coin(5)){
            Ok(_) => {}
            Err(error) => println!("Example robot: collect error {:?}", error)
        }

        match CollectTool::collect_instantly_reachable(self, world, &Content::Water(1)){
            Ok(_) => {}
            Err(error) => println!("Example robot: collect error {:?}", error)
        }

        // match CollectTool::collect_content(self, world, &Content::Water(0), 42, 40){
        //     Ok(_) => {}
        //     Err(error) => println!("Example robot: collect error {:?}", error)
        // }
    }

    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        println_d!("Example robot received event: {}", event);
        // BEWARE - for a visualizer to work it is neccessary to call this method from 
        // handle_event method of your robot
        self.visualizer_event_listener.handle_event(&event); 
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