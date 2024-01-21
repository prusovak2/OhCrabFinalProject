use robotics_lib::{runner::{Robot, Runnable}, world::tile::{Content, Tile}};
use robotics_lib::interface::{destroy, go, robot_map, robot_view, Direction, look_at_sky, get_score,
                              one_direction_view};
use crate::{oh_crab_visualizer::visualizer::{visualizable_interfaces::VisualizableInterfaces, visualizable_robot::{RobotCreator, Visulizable}, visualizer_event_listener::VisualizerEventListener}, println_d};
use crate::robot_veronika::partitioning::PartitioningProblem;
use crate::robot_veronika::content_pick::collect::CollectTool;
use crate::robot_veronika::storage::{StorageInfo, Position};
use robotics_lib::utils::LibError;
use robotics_lib::event::events::Event::EnergyRecharged;
use rust_and_furious_dynamo::dynamo::Dynamo;
use rust_eze_tomtom; //TomTom::get_path_to_coordinates
use strum::IntoEnumIterator;


pub struct DistributorRobot{
    robot: Robot,
    tick_counter: usize,
    desired_content: Vec<usize>,
    /// By default, exploration phase finished will be set to false.
    exploration_finished: bool,
    /// By default, partitioning solved phase will be set to false.
    partitioning_solved: bool,
    /// Robot's discovered size of the robot, by default set to 0.
    world_size: usize,
    /// Robot's discovered targets to collect.
    targets: Vec<StorageInfo>,
    //TODO: tests on robots capacity
    markets: Vec<Position>,
    banks: Vec<Position>,
    visualizer_event_listener: VisualizerEventListener
}

impl DistributorRobot{
    pub fn exploration_phase(&mut self, world: &mut robotics_lib::world::World)-> Result<(), LibError> {
        let _ = robot_view(self, world);
        let robot_world = robot_map(world).unwrap();
        if self.world_size == 0 {
            println!("Map len of the robot is {:?}", robot_world.len());
            println!("Tile at position 0, 0 is {:?}", robot_world[0][0].clone());
            self.world_size = robot_world.len();
        }
        // for dir in Direction::iter() {
        //     println!("\nDirection is {:?}", dir);
        //     let view_output = one_direction_view(self, world, dir, self.world_size)?;
        //     println!("View output len is {:?}", view_output.len());
        //     //println!("\nView is: {:?}\n", view_output);
        //     println!("Robots energy is {}", self.get_energy().get_energy_level());
        // }

        let view_output = one_direction_view(self, world, Direction::Up, self.world_size)?;
        let furthest_top_coordinates = &view_output[view_output.len()-1];

        // check if there exists a path going to any of those top high and top bottom coordinates
        let mut top_coordinates: Option<(usize, usize)> = None;
        let mut up_path_len = 0;
        for i in 0..furthest_top_coordinates.len() {
            let coordinate_robot: (usize, usize) = (self.get_coordinate().get_row(), self.get_coordinate().get_col());
            println!("Robots coordinate {}, {}", coordinate_robot.0, coordinate_robot.1);
            let coordinate_test: (usize, usize) = (coordinate_robot.0  - (view_output.len()), coordinate_robot.1 - i);
            println!("Test coordinates are {}, {}", coordinate_test.0, coordinate_test.1);
            //let tomtom = rust_eze_tomtom::TomTom {};
            //let path = tomtom.get_path_to_coordinates(self, world, false, coordinate_test);
            let path = CollectTool::return_path_to_coordinates(self, world, coordinate_test);
            if path.is_ok(){
                //at this point, we were able to get to the top
                println!("Path to {:?} is {:?}", coordinate_test, path);
                top_coordinates = Some(coordinate_test);
                up_path_len = path.unwrap().len();
                break;
            }
        }

        let view_output = one_direction_view(self, world, Direction::Down, self.world_size)?;
        let furthest_bottom_coordinates = &view_output[view_output.len()-1];
        // check if there exists a path going to any of those top high and top bottom coordinates
        let mut bottom_coordinates: Option<(usize, usize)> = None;
        let mut bottom_path_len = 0;
        for i in 0..furthest_bottom_coordinates.len() {
            let coordinate_robot: (usize, usize) = (self.get_coordinate().get_row(), self.get_coordinate().get_col());
            println!("Robots coordinate {}, {}", coordinate_robot.0, coordinate_robot.1);
            let coordinate_test: (usize, usize) = (coordinate_robot.0  + (view_output.len()), coordinate_robot.1 - i);
            println!("Test coordinates are {}, {}", coordinate_test.0, coordinate_test.1);
            //let tomtom = rust_eze_tomtom::TomTom {};
            //let path = tomtom.get_path_to_coordinates(self, world, false, coordinate_test);
            let path = CollectTool::return_path_to_coordinates(self, world, coordinate_test);
            if path.is_ok(){
                //at this point, we were able to get to the top
                println!("Path to {:?} is {:?}", coordinate_test, path);
                bottom_coordinates = Some(coordinate_test);
                bottom_path_len = path.unwrap().len();
                break;
            }
        }

        if top_coordinates.is_some() && bottom_coordinates.is_some() {
            println!("Top and bottom are connected, we can walk up or down to look around");
            let top_coordinates = top_coordinates.unwrap();
            let bottom_coordinates = bottom_coordinates.unwrap();

            // check also right and left
            let _ = one_direction_view(self, world, Direction::Left, self.world_size)?;
            let _ = one_direction_view(self, world, Direction::Right, self.world_size)?;


            if up_path_len > bottom_path_len{
                let path_to_bottom = CollectTool::return_path_to_coordinates(self,
                                                                             world,
                                                                             bottom_coordinates).unwrap();
                for direction in path_to_bottom {
                    let _ = robot_view(self, world);
                    let _ = VisualizableInterfaces::go(self, world, direction);
                    let _ = one_direction_view(self, world, Direction::Left, self.world_size)?;
                    let _ = one_direction_view(self, world, Direction::Right, self.world_size)?;
                }

                let path_up = CollectTool::return_path_to_coordinates(self,
                                                                      world,
                                                                      top_coordinates).unwrap();

                for direction in path_up{
                    let _ = robot_view(self, world);
                    let _ = VisualizableInterfaces::go(self, world, direction);
                    let _ = one_direction_view(self, world, Direction::Left, self.world_size)?;
                    let _ = one_direction_view(self, world, Direction::Right, self.world_size)?;
                }
            }
            else{
                let path_to_top = CollectTool::return_path_to_coordinates(self,
                                                                          world,
                                                                          top_coordinates).unwrap();
                for direction in path_to_top {
                    let _ = robot_view(self, world);
                    let _ = VisualizableInterfaces::go(self, world, direction);
                    let _ = one_direction_view(self, world, Direction::Left, self.world_size)?;
                    let _ = one_direction_view(self, world, Direction::Right, self.world_size)?;
                }

                let path_bottom = CollectTool::return_path_to_coordinates(self,
                                                                          world,
                                                                          bottom_coordinates).unwrap();
                // if self.get_energy().get_energy_level() < 500 {
                //     *self.get_energy_mut()=Dynamo::update_energy();
                // }

                for direction in path_bottom{
                    let _ = robot_view(self, world);
                    let _ = VisualizableInterfaces::go(self, world, direction);
                    let left = one_direction_view(self, world, Direction::Left, self.world_size)?;
                    let right = one_direction_view(self, world, Direction::Right, self.world_size)?;
                }
            }
        }
        else {
            let view_output = one_direction_view(self, world, Direction::Left, self.world_size)?;
            let furthest_left_coordinates = &view_output[view_output.len() - 1];
            println!("Furthest left coordinates are {:?}", furthest_left_coordinates);

            let view_output = one_direction_view(self, world, Direction::Right, self.world_size)?;
            let height = view_output.len() - 1;
            let width = view_output[0].len() - 1;
            let furthest_right_coordinates = &view_output[view_output.len() - 1];
            println!("Furthest right coordinates are {:?}", furthest_right_coordinates);
        }

        let portion_explored = self.get_quantity_explored_world(world);
        if portion_explored > 0.99{
            self.exploration_finished = true;
        }
        println!("Portion explored is {}", portion_explored);
        println!("Targets are {:?}", self.targets);
        println!("Markets are {:?}", self.markets);
        println!("Banks are {:?}", self.banks);
        Ok(())
    }

    pub fn solve_packaging_problem(&mut self, world: &mut robotics_lib::world::World){
        let weights: Vec<u32> = self.extract_storage_into_weights();
        let evolutionary_algo = PartitioningProblem::new(
            weights,
            self.markets.len(),
            100,
            1000,
            0.8,
            0.22,
            0.085,
            5
        );
        let best_solution: Vec<usize> = evolutionary_algo.main_exec("logs/market_distribution.log");
        println!("Best solution is {:?}", best_solution);
        self.partitioning_solved = true;
    }

    pub fn get_quantity_explored_world(&mut self, world: &mut robotics_lib::world::World) -> f32 {
        let robot_world = robot_map(world).unwrap();
        let number_of_tiles: usize = robot_world.len() * robot_world.len();
        let mut non_none_tiles_counter: u32 = 0;
        for (i,row) in robot_world.iter().enumerate() {
            for (j,col) in row.iter().enumerate() {
                if col.is_none() {
                    continue;
                }
                non_none_tiles_counter += 1;
                let tile = col.clone().unwrap();
                if self.desired_content.contains(&tile.content.index()) {
                    let position = Position::new(i, j);
                    let storage_info = StorageInfo::new(position,
                                                        tile.content.index(),
                                                        tile.content.get_value().0.unwrap());
                    self.targets.push(storage_info);
                }
                else if tile.content.index() == Content::Market(0).index() {
                    let position = Position::new(i, j);
                    if !self.markets.contains(&position) {
                        self.markets.push(position);
                    }
                }
                else if tile.content.index() ==Content::Bank(0..0).index(){
                    let position=Position::new(i, j);
                    if !self.banks.contains(&position){
                        self.banks.push(position);
                    }
                }
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

    fn extract_storage_into_weights(&self) -> Vec<u32>{
        let mut weights = Vec::with_capacity(self.targets.len());
        for target in self.targets.iter() {
            weights.push((target.get_quantity() as u32) * target.get_coefficient());
        }
        weights
    }
}

pub struct DistributorRobotFactory {
    desired_content: Vec<usize>,
    exploration_finished: bool,
    partitioning_solved: bool,
    world_size: usize,
    targets: Vec<StorageInfo>,
    markets: Vec<Position>,
    banks: Vec<Position>
}

impl DistributorRobotFactory {
    pub fn new(desired_content: Vec<usize>) -> DistributorRobotFactory {
        DistributorRobotFactory{desired_content, exploration_finished: false,
            partitioning_solved: false, world_size: 0,
            targets: Vec::new(), markets: Vec::new(), banks: Vec::new()}
    }
}

impl RobotCreator for DistributorRobotFactory {
    fn create(&self, data_sender: VisualizerEventListener) -> Box<dyn Runnable> {
        let distributor_robot = DistributorRobot { robot: Robot::new(), tick_counter: 0,
            desired_content: self.desired_content.clone(),
            exploration_finished: self.exploration_finished,
            partitioning_solved: self.partitioning_solved,
            world_size: self.world_size,
            targets: self.targets.clone(),
            markets: self.markets.clone(),
            banks: self.banks.clone(),
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
        println!("CURRENT TICK is {}", self.tick_counter);
        println!("CURRENT SCORE IS {}", get_score(world));
        println!("Robot's position {:?}", self.robot.coordinate);
        if self.exploration_finished == false {
            let _ = self.exploration_phase(world);
        }
        else if self.partitioning_solved == false{
            let _ = self.solve_packaging_problem(world);
        }
        else{
            println!("Nothing to do!");
        }
        // packing problem solution phase
    }
    fn handle_event(&mut self, event: robotics_lib::event::events::Event) {
        println_d!("Example robot received event: {}", event);
        // BEWARE - for a visualizer to work it is necessary to call this method from
        // handle_event method of your robot
        self.visualizer_event_listener.handle_event(&event);
        if self.get_energy().get_energy_level() < 300 {
            let previous_energy = self.get_energy().get_energy_level();
            *self.get_energy_mut()=Dynamo::update_energy();
            self.handle_event(EnergyRecharged(1000-previous_energy));
        }
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