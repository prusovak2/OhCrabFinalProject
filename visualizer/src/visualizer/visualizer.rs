use std::sync::mpsc::{Receiver, self};

use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::runner::{Runner, Runnable, self};

use super::{dtos::ChannelItem, visualizable_robot::VisualizableRobot, visualizable_interfaces::VisualizableInterfaces};

pub trait RobotCreator {
    fn create(&self, interfaces: VisualizableInterfaces) -> Box<dyn Runnable>;
}

pub struct OhCrabVisualizer {
    runner: Runner,
    receiver: Receiver<ChannelItem>,
}

impl OhCrabVisualizer {
    pub fn new(robot_creator: impl RobotCreator /*robot_creator: impl FnOnce(VisualizableInterfaces) -> Box<dyn Runnable>*/  /*robot: Box<dyn Runnable>*/, mut world_generator: OxAgWorldGenerator) -> OhCrabVisualizer {
        let ifaces = VisualizableInterfaces{};
        let robot = robot_creator.create(ifaces);
        let (sender, receiver) = mpsc::channel::<ChannelItem>();
        let visualizable_robot = VisualizableRobot::new(robot, sender);
        let runner = Runner::new(Box::new(visualizable_robot), &mut world_generator).expect("Runner creation failed");

        OhCrabVisualizer {
            runner: runner,
            receiver: receiver
        }
    }
}