use std::sync::mpsc::{Receiver, self};

use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::{runner::{Runner, Runnable}, utils::LibError as RobotError};
use rstykrab_cache::Cache;

use super::{visualizable_robot::{VisualizableRobot, RobotChannelItem}, visualizable_interfaces::{VisualizableInterfaces, InterfaceChannelItem, VisualizerDataSender}};

pub trait RobotCreator {
    fn create(&self, interfaces: VisualizerDataSender) -> Box<dyn Runnable>;
}

pub struct OhCrabVisualizer {
    runner: Runner,
    robot_receiver: Receiver<RobotChannelItem>,
    interface_receiver: Receiver<InterfaceChannelItem>,
    action_cache: Cache,

    interactive_mode: bool,
    num_steps: usize
}

#[derive(Debug)]
pub enum OhCrabVisualizerError {
    RobotLibError(RobotError),
    // ...
}

pub struct OhCrabVisualizerConfig {
    num_steps: usize, 
    interactive_mode: bool,
    use_sound: bool
}

impl OhCrabVisualizerConfig {
    pub fn new(num_steps: usize, interactive_mode: bool, use_sound: bool) -> Self {
        OhCrabVisualizerConfig {
            num_steps,
            interactive_mode,
            use_sound,
        }
    }
}

impl OhCrabVisualizer {
    pub fn new(robot_creator: impl RobotCreator, mut world_generator: OxAgWorldGenerator, config: OhCrabVisualizerConfig) -> OhCrabVisualizer {
        let (interface_sender, interface_receiver) = mpsc::channel::<InterfaceChannelItem>();
        let visualizer_data_sender = VisualizerDataSender::new(interface_sender);
        let robot = robot_creator.create(visualizer_data_sender);
        let (robot_sender, robot_receiver) = mpsc::channel::<RobotChannelItem>();
        let visualizable_robot = VisualizableRobot::new(robot, robot_sender, config.use_sound);
        let runner = Runner::new(Box::new(visualizable_robot), &mut world_generator).expect("Runner creation failed");

        OhCrabVisualizer {
            runner: runner,
            robot_receiver,
            interface_receiver, 
            action_cache: Cache::new(10),
            interactive_mode: config.interactive_mode,
            num_steps:  config.num_steps
        }
    }

    pub fn run(&mut self) -> Result<(), OhCrabVisualizerError> {
        for _ in 0..self.num_steps {
            match self.runner.game_tick() {
                Ok(_) => {}
                Err(robot_err) => { return Err(OhCrabVisualizerError::RobotLibError(robot_err)); } 
            } 
        }
        Ok(())
    }
}