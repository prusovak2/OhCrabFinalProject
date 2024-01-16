use std::{sync::mpsc::{Receiver, self}, collections::HashMap};

use ggegui::{egui::{self, ScrollArea, Key}, Gui};
use ggez::{event::{EventHandler, self}, timer, graphics::{self, Color, DrawParam}, GameError, Context, glam, input::gamepad::gilrs::GilrsBuilder};
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::{runner::Runner, utils::LibError as RobotError, event::events::Event as RobotEvent, world::tile::{Tile, Content}};
use rstykrab_cache::Cache;

use crate::{oh_crab_visualizer::visualizer::{draw_utils::{self, GridCanvasProperties}, visualizer_debug}, println_d};

use super::{visualizable_robot::{VisualizableRobot, RobotCreator, MapChannelItem}, Coord, visualizer_event_listener::{VisualizerEventListener, ChannelItem}};

const TILE_SIZE:f32 = 120.8;

pub struct OhCrabVisualizer {
    runner: Runner,
    robot_receiver: Receiver<ChannelItem>,
    map_receiver: Receiver<MapChannelItem>,
    action_cache: Cache,
    
    gui: Gui,

    // configuration
    interactive_mode: bool,
    total_ticks: usize,
    delay_in_milis: u64,

    // state
    tick_counter: usize,
    world_state: WorldState,

    visualization_state: VisualizationState
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

#[derive(Default)]
struct VisualizationState {
    offset_x: f32,
    offset_y: f32,
    tile_size: f32
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
        let (robot_sender, robot_receiver) = mpsc::channel::<ChannelItem>();
        let (map_sender, map_receiver) = mpsc::channel::<MapChannelItem>();

        let mut visualizer_data_sender = VisualizerEventListener::new(robot_sender, config.use_sound);
        let robot = robot_creator.create(visualizer_data_sender);
        let visualizable_robot = VisualizableRobot::new(robot, map_sender);

        let runner = Runner::new(Box::new(visualizable_robot), &mut world_generator).expect("Runner creation failed");

        OhCrabVisualizer {
            runner: runner,
            robot_receiver: robot_receiver,
            map_receiver,
            action_cache: Cache::new(10),
            gui: Gui::default(),
            interactive_mode: config.interactive_mode,
            total_ticks:  config.num_ticks,
            delay_in_milis: config.delay_in_milis,
            tick_counter: 0,
            world_state: WorldState::empty(),
            visualization_state: VisualizationState::default()
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

    pub fn run(mut self) -> Result<(), OhCrabVisualizerError> {
        let context_builder = ggez::ContextBuilder::new("OhCrabWorld", "OhCrab")
            .window_mode(ggez::conf::WindowMode::default()
            .resizable(true)
            .maximized(true));

        match context_builder.build() {
            Ok((ctx, event_loop)) => {
                self.gui = Gui::new(&ctx);
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
        println_d!("MY awesome macro {} {}.", 42, 73);
        println_d!("VISUALIZER UPDATE, doing first world tick.");
        self.do_world_tick()?;
        let received_map = self.map_receiver.try_recv();
        match received_map {
            Ok(map_item) => {
                println_d!("VISUALIZER RECEIVED MAP with robot position {:?}", (map_item.map.robot_position.x, map_item.map.robot_position.y));
                self.world_state.world_map = Some(map_item.map.world_map);
                self.world_state.robot_position = Some( map_item.map.robot_position);
                Ok(())
            }
            Err(_) => todo!(),
        }
    }
}

impl EventHandler<OhCrabVisualizerError> for OhCrabVisualizer {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        println_d!("VISUALIZER UPDATE, TICK COUNT: {} (total ticks {})", self.tick_counter, self.total_ticks);

        if self.tick_counter == 0 {
            self.init_state()?;
        }

        let gui = &mut self.gui;
        let gui_ctx = &gui.ctx();
        // egui::Window::new("UI").show(&gui_ctx, |ui| {
        //     ui.label("a very nice gui :3");
        //     if ui.button("print \"hello world\"").clicked() {
        //         println!("hello world");
        //     }
        //     if ui.button("quit").clicked() {
        //         ctx.request_quit();
        //     }
        // });
        // gui.update(ctx);
        egui::Window::new("Scroll world").show(&gui_ctx, |ui| {
            if let Some(world_map) = &self.world_state.world_map {
                ui.add(egui::Slider::new(&mut self.visualization_state.offset_x, 0.0..=((world_map.len() - 1) as f32)));
                ui.add(egui::Slider::new(&mut self.visualization_state.offset_y, ((world_map.len() - 1) as f32)..=0.0).orientation(egui::SliderOrientation::Vertical));
            }
            
            //ui.add(egui::DragValue::new(&mut self.offset_x));
            //ui.heading("Press/Hold/Release example. Press A to test.");
            // ScrollArea::vertical()
            //     .auto_shrink([false, false])
            //     .stick_to_bottom(true)
            //     .show(ui, |ui| {
            //         ui.label("abraka dabra");
            //     });

            if gui_ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
                println!("Left pressed");
            }
            if gui_ctx.input(|i| i.key_down(Key::ArrowLeft)) {
                println!("Left is down");
                //ui.ctx().request_repaint(); // make sure we note the holding.
            }
            if gui_ctx.input(|i| i.key_released(Key::ArrowLeft)) {
                println!("Left is released");
            }
        });
        gui.update(ctx);

        if self.tick_counter >= self.total_ticks {
            //_ctx.request_quit();
            println_d!("empty update");
            return Ok(());
        }

        println_d!("VISUALIZER UPDATE, receiving from robot channel.");
        let received_state = self.robot_receiver.try_recv();

        match received_state {
            Ok(channel_item) => {
                println_d!("VISUALIZER UPDATE, received item {:?}.", channel_item);
                //timer::sleep(std::time::Duration::from_millis(self.delay_in_milis)); // TODO: why is this sleep there and not somewhere else?
                match  channel_item {
                    ChannelItem::EventChannelItem(event) => {
                        match event {
                            // RobotEvent::Ready => todo!(),
                            // RobotEvent::Terminated => todo!(),
                            // RobotEvent::TimeChanged(_) => todo!(),
                            // RobotEvent::DayChanged(_) => todo!(),
                            // RobotEvent::EnergyRecharged(_) => todo!(),
                            // RobotEvent::EnergyConsumed(_) => todo!(),
                            RobotEvent::Moved(_, (robot_y, robot_x)) => { // BEWARE: library has x and y switched in Move event
                                println_d!("VISUALIZER: received robot moved {:?}", (robot_x, robot_y));
                                self.world_state.robot_position = Some(Coord{x:robot_x, y:robot_y });
                            }
                            RobotEvent::TileContentUpdated(tile, (tile_y, tile_x)) => {
                                println_d!("VISUALIZER: received tile content update.");
                                if let Some(world_map) = &mut self.world_state.world_map{
                                    world_map[tile_y][tile_x] = tile;
                                }
                            }
                            RobotEvent::AddedToBackpack(content, amount) => {
                                println_d!("VISUALIZER: added to backpack.");
                                *self.world_state.backpack.entry(content).or_insert(0) += amount;
                            }
                            // RobotEvent::RemovedFromBackpack(_, _) => todo!(),
                            _ => {
                                println_d!("VISUALIZER: {:?}", event);
                            }
                        }
                    }
                    ChannelItem::InterfaceChannelItem(interface_invocation) => {
                        println_d!();
                        println_d!("VISULAZER: received interface invocation: {:?}", interface_invocation);
                    }
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                println_d!("VISUALIZER: channel empty, execution another world tick.");
                self.do_world_tick()?;
            }
            Err(error) => {
                println_d!("VISUALIZER: try receive error: {:?}.", error);
                return Err(OhCrabVisualizerError::DataError(DataChannelError::TryRecvError(error)));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        println!("ofset x {}" , self.visualization_state.offset_x);
        // world map
        if let Some(world_map) = &self.world_state.world_map {
            println_d!("draw");
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            
            let world_dimension = world_map.len();
            let (x, y) = ctx.gfx.size();
            let size = f32::min(x, y);
            let tile_size = 64 as f32; //(size / world_dimension as f32) - 10 as f32;
            println_d!("TILE SIZE: {}", tile_size);

            let grid_canvas_props = GridCanvasProperties {
                tile_size: TILE_SIZE,
                grid_canvas_height: size - 150.0,
                grid_canvas_width: size - 150.0,
                grid_canvas_origin_x: 200.0,
                grid_canvas_origin_y: 0.0,
            };
            let tile_offset = Coord { x: f32::floor(self.visualization_state.offset_x) as usize, y: f32::floor(self.visualization_state.offset_y) as usize };

            draw_utils::draw_grid(ctx, &mut canvas, &grid_canvas_props, &tile_offset, world_map)?;
            canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::new(400.0, 400.0)));

            //canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::new(grid_canvas_props.grid_canvas_width + 20.0, grid_canvas_props.grid_canvas_height - 20.0)));

            // let world_grid_canvas = draw_utils::draw_grid(ctx, size as u32, size as u32, world_map, tile_size)?;
            // canvas.draw(&world_grid_canvas, graphics::DrawParam::default());

            // for y in 0..world_dimension {
            //     for x in 0..world_dimension {
            //         let tile: &Tile = &world_map[y][x]; 
            //         draw_utils::draw_tile(tile, ctx, &mut canvas, x, y, tile_size)?;
            //     }
            // }

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