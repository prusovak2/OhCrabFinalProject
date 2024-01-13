use std::sync::mpsc::{Receiver, self};

use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::runner::{Runner, Runnable, self};
use rstykrab_cache::Cache;

use super::{visualizable_robot::{VisualizableRobot, RobotChannelItem}, visualizable_interfaces::{VisualizableInterfaces, InterfaceChannelItem}};

pub trait RobotCreator {
    fn create(&self, interfaces: VisualizableInterfaces) -> Box<dyn Runnable>;
}

pub struct OhCrabVisualizer {
    runner: Runner,
    robot_receiver: Receiver<RobotChannelItem>,
    interface_receiver: Receiver<InterfaceChannelItem>,
    action_cache: Cache
}

impl OhCrabVisualizer {
    pub fn new(robot_creator: impl RobotCreator /*robot_creator: impl FnOnce(VisualizableInterfaces) -> Box<dyn Runnable>*/  /*robot: Box<dyn Runnable>*/, mut world_generator: OxAgWorldGenerator) -> OhCrabVisualizer {
        let (interface_sender, interface_receiver) = mpsc::channel::<InterfaceChannelItem>();
        let interfaces = VisualizableInterfaces::new(interface_sender);
        let robot = robot_creator.create(interfaces);
        let (robot_sender, robot_receiver) = mpsc::channel::<RobotChannelItem>();
        let visualizable_robot = VisualizableRobot::new(robot, robot_sender);
        let runner = Runner::new(Box::new(visualizable_robot), &mut world_generator).expect("Runner creation failed");

        OhCrabVisualizer {
            runner: runner,
            robot_receiver,
            interface_receiver, 
            action_cache: Cache::new(10)
        }
    }
}