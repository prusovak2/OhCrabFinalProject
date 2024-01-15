use std::{sync::mpsc::{Receiver, self}, collections::HashMap, f64::consts::E};

use ggez::{event::{EventHandler, self}, timer, graphics::{self, Color}, GameError};
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::{runner::Runner, utils::LibError as RobotError, event::events::Event as RobotEvent, world::tile::{Tile, Content}};
use rstykrab_cache::Cache;

use crate::visualizer::draw_utils;

use super::{visualizable_robot::{VisualizableRobot, RobotCreator, MapChannelItem}, visualizable_interfaces::InterfaceChannelItem, Coord, visualizer_event_listener::{EventChannelItem, VisualizerEventListener}};

pub struct OhCrabVisualizer {
    runner: Runner,
    robot_event_receiver: Receiver<EventChannelItem>,
    robot_map_receiver: Receiver<MapChannelItem>,
    interface_receiver: Receiver<InterfaceChannelItem>,
    action_cache: Cache,

    // configuration
    interactive_mode: bool,
    total_ticks: usize,
    delay_in_milis: u64,

    // state
    tick_counter: usize,
    world_state: WorldState
}

struct WorldState {
    world_map: Option<Vec<Vec<Tile>>>,
    robot_position: Option<Coord>,
    backpack: HashMap<Content, usize>
}

impl WorldState {
    fn empty() -> WorldState {
        WorldState {
            world_map: None,
            robot_position: None,
            backpack: HashMap::new()
        }
    }
}

#[derive(Debug)]
pub enum OhCrabVisualizerError {
    RobotLibError(RobotError),
    DataError(DataChannelError),
    GraphicsLibraryError(GameError)
}

#[derive(Debug)]
pub enum DataChannelError {
    TryRecvError(std::sync::mpsc::TryRecvError),
    StateMissingError(String)
}

pub struct OhCrabVisualizerConfig {
    num_ticks: usize, 
    interactive_mode: bool,
    use_sound: bool,
    delay_in_milis: u64,
}

impl OhCrabVisualizerConfig {
    pub fn new(num_steps: usize, interactive_mode: bool, use_sound: bool, delay_in_milis: u64) -> Self {
        OhCrabVisualizerConfig {
            num_ticks: num_steps,
            interactive_mode,
            use_sound,
            delay_in_milis
        }
    }
}

impl OhCrabVisualizer {
    pub fn new(robot_creator: impl RobotCreator, mut world_generator: OxAgWorldGenerator, config: OhCrabVisualizerConfig) -> OhCrabVisualizer {
        let (interface_sender, interface_receiver) = mpsc::channel::<InterfaceChannelItem>();
        let (event_sender, event_receiver) = mpsc::channel::<EventChannelItem>();
        let (map_sender, map_receiver) = mpsc::channel::<MapChannelItem>();

        let mut visualizer_data_sender = VisualizerEventListener::new(interface_sender, event_sender, config.use_sound);
        let robot = robot_creator.create(visualizer_data_sender);
        let visualizable_robot = VisualizableRobot::new(robot, map_sender);

        let runner = Runner::new(Box::new(visualizable_robot), &mut world_generator).expect("Runner creation failed");

        OhCrabVisualizer {
            runner: runner,
            robot_event_receiver: event_receiver,
            robot_map_receiver: map_receiver,
            interface_receiver, 
            action_cache: Cache::new(10),
            interactive_mode: config.interactive_mode,
            total_ticks:  config.num_ticks,
            delay_in_milis: config.delay_in_milis,
            tick_counter: 0,
            world_state: WorldState::empty()
        }
    }

    pub fn simulate(&mut self) -> Result<(), OhCrabVisualizerError> {
        for _ in 0..self.total_ticks {
            match self.runner.game_tick() {
                Ok(_) => {}
                Err(robot_err) => { return Err(OhCrabVisualizerError::RobotLibError(robot_err)); } 
            } 
        }
        Ok(())
    }

    pub fn run(self) -> Result<(), OhCrabVisualizerError> {
        let context_builder = ggez::ContextBuilder::new("pile-of-shit", "xxx")
            .window_mode(ggez::conf::WindowMode::default()
            .resizable(true)
            .maximized(true));

        match context_builder.build() {
            Ok((ctx, event_loop)) => {
                event::run(ctx, event_loop, self);
            }
            Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error))
        }
    }

    fn do_world_tick(&mut self) -> Result<(), OhCrabVisualizerError> {
        let res = self.runner.game_tick();
        self.tick_counter += 1;
        match res {
            Ok(_) => Ok(()),
            Err(robot_err) => { return Err(OhCrabVisualizerError::RobotLibError(robot_err)); } 
        } 
    }

    fn init_state(&mut self)  -> Result<(), OhCrabVisualizerError> {
        println!("VISUALIZER UPDATE, doing first world tick.");
        self.do_world_tick()?;
        let received_map = self.robot_map_receiver.try_recv();
        match received_map {
            Ok(map_item) => {
                println!("VISUALIZER RECEIVED MAP with robot position {:?}", (map_item.map.robot_position.x, map_item.map.robot_position.y));
                self.world_state.world_map = Some(map_item.map.world_map);
                self.world_state.robot_position = Some( map_item.map.robot_position);
                Ok(())
            }
            Err(_) => todo!(),
        }
    }
}

impl EventHandler<OhCrabVisualizerError> for OhCrabVisualizer {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        println!("VISUALIZER UPDATE, TICK COUNT: {} (total ticks {})", self.tick_counter, self.total_ticks);
        if self.tick_counter >= self.total_ticks {
            //_ctx.request_quit();
            println!("empty update");
            return Ok(());
        }
        if self.tick_counter == 0 {
            self.init_state()?;
        }

        println!("VISUALIZER UPDATE, receiving from robot channel.");
        let received_state = self.robot_event_receiver.try_recv();

        match received_state {
            Ok(channel_item) => {
                println!("VISUALIZER UPDATE, received item {:?}.", channel_item);
                timer::sleep(std::time::Duration::from_millis(self.delay_in_milis)); // TODO: why is this sleep there and not somewhere else?
                match  channel_item.event {
                    // RobotEvent::Ready => todo!(),
                    // RobotEvent::Terminated => todo!(),
                    // RobotEvent::TimeChanged(_) => todo!(),
                    // RobotEvent::DayChanged(_) => todo!(),
                    // RobotEvent::EnergyRecharged(_) => todo!(),
                    // RobotEvent::EnergyConsumed(_) => todo!(),
                    RobotEvent::Moved(_, (robot_y, robot_x)) => { // BEWARE: library has x and y switched in Move event
                        println!("VISUALIZER: received robot moved {:?}", (robot_x, robot_y));
                        self.world_state.robot_position = Some(Coord{x:robot_x, y:robot_y });
                        Ok(())
                    }
                    RobotEvent::TileContentUpdated(tile, (tile_x, tile_y)) => {
                        println!("VISUALIZER: received tile content update.");
                        if let Some(world_map) = &mut self.world_state.world_map{
                            world_map[tile_y][tile_x] = tile;
                        }
                        Ok(())
                    }
                    RobotEvent::AddedToBackpack(content, amount) => {
                        println!("VISUALIZER: added to backpack.");
                        *self.world_state.backpack.entry(content).or_insert(0) += amount;
                        Ok(())
                    }
                    // RobotEvent::RemovedFromBackpack(_, _) => todo!(),
                    _ => Ok(())
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                println!("VISUALIZER: channel empty, execution another world tick.");
                self.do_world_tick()
            }
            Err(error) => {
                println!("VISUALIZER: try receive error: {:?}.", error);
                Err(OhCrabVisualizerError::DataError(DataChannelError::TryRecvError(error)))
            }
        }
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        // world map
        if let Some(world_map) = &self.world_state.world_map {
            println!("draw");
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            let world_dimension = world_map.len();
            let (x, y) = ctx.gfx.size();
            let size = f32::min(x, y);
            let tile_size = (size / world_dimension as f32) - 10 as f32;

            for y in 0..world_dimension {
                for x in 0..world_dimension {
                    let tile = &world_map[y][x]; 
                    draw_utils::draw_tile(tile, ctx, &mut canvas, x, y, tile_size)?;
                }
            }

            // robot
            if let Some(robot_position) = &self.world_state.robot_position {
                draw_utils::draw_robot(robot_position, ctx, &mut canvas, tile_size)?;
            }
            
            //backpack
            draw_utils::draw_backpack(&self.world_state.backpack, &mut canvas, tile_size, world_dimension);

            let x_tick_count = (tile_size * world_dimension as f32) + (tile_size*3.0);
            let y_tick_count = tile_size;
            let text_size = tile_size * 0.18;
            draw_utils::draw_text(&mut  canvas, x_tick_count, y_tick_count, Color::WHITE, text_size, format!("TICK: {}", self.tick_counter));

            if self.tick_counter >= self.total_ticks {
                draw_utils::draw_text(&mut  canvas, x_tick_count, y_tick_count + text_size * 1.2, Color::WHITE, text_size, format!("SIMULATION DONE"));
            }

            match canvas.finish(ctx) {
                Ok(_) => Ok(()),
                Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error)),
            }
        }
        else {
            Err(OhCrabVisualizerError::DataError(DataChannelError::StateMissingError(format!("Game state is missing when it should be present"))))
        }
    }
}

#[derive(Debug)]
pub(crate) struct WorldMap {
    world_map: Vec<Vec<Tile>>,
    robot_position: Coord
}

impl WorldMap {
    pub(crate) fn new(world_map: Vec<Vec<Tile>>, (robot_x, roboy_y): (usize, usize)) -> WorldMap {
        WorldMap { world_map: world_map, robot_position: Coord { x: robot_x, y: roboy_y } }
    }
}