use robotics_lib::{runner::{Robot, Runnable}, world::tile::{Content, Tile}};
use robotics_lib::interface::{destroy, go, robot_map, robot_view, Direction, look_at_sky, get_score,
                              one_direction_view};
use crate::{oh_crab_visualizer::visualizer::{visualizable_interfaces::VisualizableInterfaces, visualizable_robot::{RobotCreator, Visulizable}, visualizer_event_listener::VisualizerEventListener}, println_d};
use crate::robot_veronika::partitioning::PartitioningProblem;
use crate::robot_veronika::storage::{StorageInfo, Position};
use robotics_lib::utils::LibError;
use rust_and_furious_dynamo;
use rust_eze_tomtom; //TomTom::get_path_to_coordinates
use strum::IntoEnumIterator;


pub struct DistributorRobot{
    robot: Robot,
    tick_counter: usize,
    desired_content: Vec<Content>,
    /// By default, exploration phase finished will be set to false.
    exploration_finished: bool,
    /// Robot's discovered size of the robot, by default set to 0.
    world_size: usize,
    /// Robot's discovered targets to collect.
    targets: Vec<StorageInfo>,
    //TODO: tests on robots capacity
    markets: Vec<Position>,
    visualizer_event_listener: VisualizerEventListener
}

impl DistributorRobot{
    pub fn exploration_phase(&mut self, world: &mut robotics_lib::world::World)-> Result<(), LibError> {
        let robot_world = robot_map(world).unwrap();
        if self.world_size == 0 {
            println!("Map len of the robot is {:?}", robot_world.len());
            self.world_size = robot_world.len();
        }
        for dir in Direction::iter() {
            println!("\nDirection is {:?}", dir);
            let view_output = one_direction_view(self, world, dir, self.world_size)?;
            println!("View output len is {:?}", view_output.len());
            println!("\nView is: {:?}\n", view_output);
            println!("Robots energy is {}", self.get_energy().get_energy_level());
        }
        let portion_explored = self.get_quantity_explored_world(world);
        Ok(())
    }

    pub fn get_quantity_explored_world(&self, world: &mut robotics_lib::world::World) -> f32 {
        let robot_world = robot_map(world).unwrap();
        let number_of_tiles: usize = robot_world.len() * robot_world.len();
        let mut non_none_tiles_counter: u32 = 0;
        for row in robot_world {
            for col in row {
                if col.is_none() {
                    continue;
                }
                non_none_tiles_counter += 1;
            }
        }
        //println!("Counter finished at : {}", non_none_tiles_counter);
        return (non_none_tiles_counter as f32) / (number_of_tiles as f32);
    }
    fn print_nicer_known_world_map(&self, known_world: &Vec<Vec<Option<Tile>>>) {
        for row in known_world {
            for col in row {
                if col.is_none() {
                    print!("None\t");
                    continue;
                }
                let tile = col.clone().unwrap();
                print!("Type {:?},", tile.tile_type);
                print!("{:?} ", tile.content);
                //print!("(cost {:?})\t", tile.tile_type.properties().cost());
            }
            println!();
        }
    }
}

pub struct DistributorRobotFactory {
    desired_content: Vec<Content>,
    exploration_finished: bool,
    world_size: usize,
    targets: Vec<StorageInfo>,
    markets: Vec<Position>,
}

impl DistributorRobotFactory {
    pub fn new(desired_content: Vec<Content>) -> DistributorRobotFactory {
        DistributorRobotFactory{desired_content, exploration_finished: false, world_size: 0,
        targets: Vec::new(), markets: Vec::new()}
    }
}

impl RobotCreator for DistributorRobotFactory {
    fn create(&self, data_sender: VisualizerEventListener) -> Box<dyn Runnable> {
        let distributor_robot = DistributorRobot { robot: Robot::new(), tick_counter: 0,
            desired_content: self.desired_content.clone(),
            exploration_finished: self.exploration_finished,
            world_size: self.world_size,
            targets: self.targets.clone(),
            markets: self.markets.clone(),
            visualizer_event_listener: data_sender };
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
        println!("CURRENT SCORE IS {}", get_score(world));
        println!("Robot's initial position {:?}", self.robot.coordinate);
        if self.exploration_finished == false {
            self.exploration_phase(world);
        }

    }
    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        println_d!("Example robot received event: {}", event);
        // BEWARE - for a visualizer to work it is necessary to call this method from
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