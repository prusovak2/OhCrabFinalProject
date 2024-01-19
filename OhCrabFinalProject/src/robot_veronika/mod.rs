pub mod content_pick;
pub mod partitioning;
use robotics_lib::{runner::{Robot, Runnable}, interface::Direction, world::tile::Content};
use crate::{oh_crab_visualizer::visualizer::{visualizable_interfaces::VisualizableInterfaces, visualizable_robot::{RobotCreator, Visulizable}, visualizer_event_listener::VisualizerEventListener}, println_d};
use crate::robot_veronika::partitioning::PartitioningProblem;

pub struct DistributorRobot{
    robot: Robot,
    tick_counter: usize,
    desired_content: Vec<Content>,
    visualizer_event_listener: VisualizerEventListener
}

pub struct DistributorRobotFactory {
    desired_content: Vec<Content>,
}

impl DistributorRobotFactory {
    pub fn new(desired_content: Vec<Content>) -> DistributorRobotFactory {
        DistributorRobotFactory{desired_content}
    }
}

impl RobotCreator for DistributorRobotFactory {
    fn create(&self, data_sender: VisualizerEventListener) -> Box<dyn Runnable> {
        let distributor_robot = DistributorRobot { robot: Robot::new(), tick_counter: 0, desired_content: self.desired_content.clone(), visualizer_event_listener: data_sender };
        Box::new(distributor_robot)
    }
}

impl<'a> Visulizable<'a> for DistributorRobot {
    fn borrow_event_listener(&'a self) -> &'a VisualizerEventListener{
        &self.visualizer_event_listener
    }
}

impl Runnable for DistributorRobot{
    fn process_tick(&mut self, world: &mut robotics_lib::world::World) {
        self.tick_counter+=1;

    }
    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        println_d!("Example robot received event: {}", event);
        // BEWARE - for a visualizer to work it is neccessary to call this method from
        // handle_event method of your robot
        self.visualizer_event_listener.handle_event(&event);
    }

    fn get_energy(&self) -> &robotics_lib::energy::Energy {
        &self.robot.energy
    }

    fn get_energy_mut(&mut self) -> &mut robotics_lib::energy::Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &robotics_lib::world::coordinates::Coordinate {
        & self.robot.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut robotics_lib::world::coordinates::Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &robotics_lib::runner::backpack::BackPack {
        &self.robot.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut robotics_lib::runner::backpack::BackPack {
        &mut self.robot.backpack
    }

}