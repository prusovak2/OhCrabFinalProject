use std::{sync::mpsc::{Receiver, self}, collections::HashMap, default, cmp::min};

use egui_extras::install_image_loaders;
use ggegui::{egui::{self, ScrollArea, Key}, Gui};
use ggez::{event::{EventHandler, self}, timer, graphics::{self, Color, DrawParam}, GameError, Context, glam, input::gamepad::gilrs::GilrsBuilder};
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use rand::distributions::weighted::alias_method;
use robotics_lib::{runner::Runner, utils::LibError as RobotError, event::events::Event as RobotEvent, world::tile::{Tile, Content}};
use rstykrab_cache::Cache;

use crate::{oh_crab_visualizer::visualizer::{draw_utils::{self, GridCanvasProperties}, visualizer_debug, egui_utils}, println_d};

use super::{visualizable_robot::{VisualizableRobot, RobotCreator, MapChannelItem}, Coord, visualizer_event_listener::{VisualizerEventListener, ChannelItem}, egui_utils::EguiImages};

pub(super) const TILE_SIZE_MIN:f32 = 5.0;
pub(super) const TILE_SIZE_MAX:f32 = 120.8;
pub(super) const CONTENT_TILE_SIZE_LIMIT:f32 = 30.0;

pub(super) const DEFAULT_TILE_SIZE:f32 = 60.4;
pub(super) const GRID_FRAME_WIDTH: f32 = 20.0;
pub(super) const GRID_CANVAS_ORIGIN_X: f32 = 200.0 + GRID_FRAME_WIDTH;
pub(super) const GRID_CANVAS_ORIGIN_Y: f32 = 0.0 + GRID_FRAME_WIDTH;

pub struct OhCrabVisualizer {
    runner: Runner,
    robot_receiver: Receiver<ChannelItem>,
    map_receiver: Receiver<MapChannelItem>,
    action_cache: Cache,
    
    gui: Gui,
    egui_images: EguiImages<'static>,

    // configuration
    run_mode: RunMode,
    delay_in_milis: u64,

    // state
    tick_counter: usize,
    world_state: WorldState,
    world_tick_in_progress: bool,
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
pub(super) struct VisualizationState {
    offset_x: f32,
    offset_y: f32,
    should_focus_on_robot: bool,

    pub(super)grid_canvas_properties: GridCanvasProperties
}

impl VisualizationState {
    pub(super) fn get_last_column_to_display(&self) -> usize {
        let columns_to_display = self.grid_canvas_properties.num_columns_to_display();
        usize::min(self.grid_canvas_properties.world_dimension, self.first_column_to_display() + columns_to_display)
    }

    pub(super) fn get_last_row_to_display(&self) -> usize {
        let rows_to_display = self.grid_canvas_properties.num_rows_to_display();
        usize::min(self.grid_canvas_properties.world_dimension, self.first_row_to_display() + rows_to_display)
    }

    pub(super) fn first_column_to_display(&self) -> usize {
        f32::floor(self.offset_x) as usize
    }

    pub(super) fn first_row_to_display(&self) -> usize {
        f32::floor(self.offset_y) as usize
    }

    pub(super) fn robot_should_be_displaied(&self, robot_position: &Coord) -> bool {
        robot_position.x >=  self.first_column_to_display()
            && robot_position.x < self.get_last_column_to_display()
            && robot_position.y >= self.first_row_to_display()
            && robot_position.y < self.get_last_row_to_display() 
    }

    fn get_scroll_limit(&self, world_dimenstion: usize) -> (f32, f32) {
        let scroll_limit_x = (world_dimenstion - usize::min(world_dimenstion, self.grid_canvas_properties.num_columns_to_display())) as f32;
        let scroll_limit_y = (world_dimenstion - usize::min(world_dimenstion, self.grid_canvas_properties.num_rows_to_display())) as f32;
        (scroll_limit_x, scroll_limit_y)
    }
}

#[derive(Debug)]
pub enum OhCrabVisualizerError {
    RobotLibError(RobotError),
    DataError(DataChannelError),
    GraphicsLibraryError(GameError),
    ConfigurationError(String)
}

#[derive(Debug)]
pub enum DataChannelError {
    TryRecvError(std::sync::mpsc::TryRecvError),
    StateMissingError(String)
}

pub enum RunMode {
    Interactive,
    NonInteractive(usize) // simulate `usize` steps
}

pub struct OhCrabVisualizerConfig {
    run_mode: RunMode,
    use_sound: bool,
    delay_in_milis: u64,
}

impl OhCrabVisualizerConfig {
    pub fn new(run_mode: RunMode, use_sound: bool, delay_in_milis: u64) -> Self {
        OhCrabVisualizerConfig {
            run_mode,
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
            run_mode: config.run_mode,
            delay_in_milis: config.delay_in_milis,
            tick_counter: 0,
            world_state: WorldState::empty(),
            visualization_state: VisualizationState::default(),
            world_tick_in_progress: false,
            egui_images: EguiImages::init()
        }
    }

    pub fn simulate(&mut self) -> Result<(), OhCrabVisualizerError> {
        match self.run_mode {
            RunMode::Interactive => Err(OhCrabVisualizerError::ConfigurationError("Cannot run simulation on interactively configured visualizer. To run simulation, set run_mode to RunMode::Noninteractive(total_ticks)".to_string())),
            RunMode::NonInteractive(total_ticks) => {
                for _ in 0..total_ticks {
                    match self.runner.game_tick() {
                        Ok(_) => {}
                        Err(robot_err) => { return Err(OhCrabVisualizerError::RobotLibError(robot_err)); } 
                    } 
                }
                Ok(())
            }
        }
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
            Ok(_) => {
                if self.visualization_state.should_focus_on_robot {
                    self.focus_on_robot();
                }
                self.world_tick_in_progress = true;
                Ok(())
            },
            Err(robot_err) => { return Err(OhCrabVisualizerError::RobotLibError(robot_err)); } 
        } 
    }

    fn init_state(&mut self, canvas_size: f32)  -> Result<(), OhCrabVisualizerError> {
        println_d!("VISUALIZER UPDATE, doing first world tick.");
        self.do_world_tick()?;
        install_image_loaders(&self.gui.ctx());
        let received_map = self.map_receiver.try_recv();

        self.world_state.backpack.insert(Content::Rock(0), 5);
        self.world_state.backpack.insert(Content::Tree(0), 3);
        self.world_state.backpack.insert(Content::Garbage(0), 10);
        self.world_state.backpack.insert(Content::Fire, 1);
        self.world_state.backpack.insert(Content::Coin(0), 50);
        self.world_state.backpack.insert(Content::Bin(0..10), 2);
        self.world_state.backpack.insert(Content::Crate(0..10), 3);
        self.world_state.backpack.insert(Content::Bank(0..10), 1);
        self.world_state.backpack.insert(Content::Water(0), 8);
        self.world_state.backpack.insert(Content::Market(0), 1);
        self.world_state.backpack.insert(Content::Fish(0), 15);
        self.world_state.backpack.insert(Content::Building, 2);
        self.world_state.backpack.insert(Content::Bush(0), 5);
        self.world_state.backpack.insert(Content::JollyBlock(0), 2);
        self.world_state.backpack.insert(Content::Scarecrow, 1);
        match received_map {
            Ok(map_item) => {
                self.visualization_state.grid_canvas_properties = GridCanvasProperties::build(canvas_size, map_item.map.world_map.len());
                let robot_pos = map_item.map.robot_position;
                println_d!("VISUALIZER RECEIVED MAP with robot position {:?}", (robot_pos.x, robot_pos.y));
                self.world_state.world_map = Some(map_item.map.world_map);
                self.world_state.robot_position = Some(robot_pos);
                self.focus_on_robot();
                Ok(())
            }
            Err(_) => todo!(),
        }
    }

    fn focus_on_robot(&mut self) {
        if let Some(robot_pos) = &self.world_state.robot_position{
            println_d!("Focusing on robot on position {:?}", robot_pos);
            println_d!("rows to display: {} columns to display: {}", self.visualization_state.grid_canvas_properties.num_rows_to_display(), self.visualization_state.grid_canvas_properties.num_columns_to_display() );
            self.visualization_state.offset_x = f32::max(0.0, robot_pos.x as f32  - (self.visualization_state.grid_canvas_properties.num_columns_to_display() / 2 ) as f32);
            self.visualization_state.offset_y = f32::max(0.0, robot_pos.y as f32 - (self.visualization_state.grid_canvas_properties.num_rows_to_display() / 2 ) as f32) ;
            println_d!("Focused");
        }
    }

    fn zoom_on_robot(&mut self) {
        if let Some(_) = &self.world_state.robot_position{
            self.visualization_state.grid_canvas_properties.tile_size = DEFAULT_TILE_SIZE;
            self.focus_on_robot();
        }
    }

    fn simulation_should_end(&self) -> bool {
        match self.run_mode {
            RunMode::Interactive => false,
            RunMode::NonInteractive(total_ticks) => self.tick_counter >= total_ticks,
        }
    }

    fn is_interactive(&self) -> bool {
        match self.run_mode {
            RunMode::Interactive => true,
            RunMode::NonInteractive(_) => false,
        }
    }
}

impl EventHandler<OhCrabVisualizerError> for OhCrabVisualizer {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        println_d!("VISUALIZER UPDATE, TICK COUNT: {}", self.tick_counter);

        if self.tick_counter == 0 {
            let (x, y) = ctx.gfx.size();
            let size = f32::min(x, y);
            self.init_state(size)?;
        }
        
        let mut res: Result<(), OhCrabVisualizerError> = Ok(());
        let gui_ctx = &mut self.gui.ctx();
        egui::Window::new("Scroll world").show(&gui_ctx, |ui: &mut egui::Ui| {
            if let Some(world_map) = &self.world_state.world_map {
                let (scroll_limit_x, scroll_limit_y) = self.visualization_state.get_scroll_limit(world_map.len());
                ui.add(egui::Slider::new(&mut self.visualization_state.offset_x, 0.0..=scroll_limit_x));
                ui.add(egui::Slider::new(&mut self.visualization_state.offset_y, scroll_limit_y..=0.0).orientation(egui::SliderOrientation::Vertical));
                ui.add(egui::Slider::new(&mut self.visualization_state.grid_canvas_properties.tile_size, TILE_SIZE_MIN..=TILE_SIZE_MAX));
                ui.add(egui::Checkbox::new(&mut self.visualization_state.should_focus_on_robot, "Focus on robot"));
            }
            if ui.add(egui::Button::new("Center on robot")).clicked() {
                self.focus_on_robot();
            }
            if ui.add(egui::Button::new("Zoom on robot")).clicked() {
                self.zoom_on_robot();
            }

            if self.is_interactive() {
                if self.world_tick_in_progress {
                    if ui.add_enabled(false, egui::Button::new("Tick in progress")).clicked() {
                        unreachable!();
                    }
                }
                else {
                    if ui.add(egui::Button::new("Do tick")).clicked() {
                        res = self.do_world_tick();
                    }
                }
            }
            // if gui_ctx.input(|i| i.key_pressed(Key::ArrowLeft)) {
            //     println!("Left pressed");
            // }
            // if gui_ctx.input(|i| i.key_down(Key::ArrowLeft)) {
            //     println!("Left is down");
            //     //ui.ctx().request_repaint(); // make sure we note the holding.
            // }
            // if gui_ctx.input(|i| i.key_released(Key::ArrowLeft)) {
            //     println!("Left is released");
            // }
        });

        egui_utils::draw_backpack(gui_ctx, &self.visualization_state,&self.world_state.backpack, &self.egui_images);

        self.gui.update(ctx);
        if res.is_err() {
            return res;
        }

        // move camera if the world is zoomed out
        if let Some(world_map) = &self.world_state.world_map {
            let (scroll_limit_x, scroll_limit_y) = self.visualization_state.get_scroll_limit(world_map.len());
            self.visualization_state.offset_x = f32::min(self.visualization_state.offset_x, scroll_limit_x);
            self.visualization_state.offset_y = f32::min(self.visualization_state.offset_y, scroll_limit_y);
        }
        
        if self.simulation_should_end() {
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
                self.world_tick_in_progress = false;
                if !self.is_interactive() {
                    self.do_world_tick()?;
                }
            }
            Err(error) => {
                println_d!("VISUALIZER: try receive error: {:?}.", error);
                return Err(OhCrabVisualizerError::DataError(DataChannelError::TryRecvError(error)));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        // world map
        if let Some(world_map) = &self.world_state.world_map {
            println_d!("draw");
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            
            let world_dimension = world_map.len();
            let (x, y) = ctx.gfx.size();
            let size = f32::min(x, y);
            let tile_size = 64 as f32; //(size / world_dimension as f32) - 10 as f32;
            println_d!("TILE SIZE: {}", tile_size);

            let tile_offset = Coord { x: f32::floor(self.visualization_state.offset_x) as usize, y: f32::floor(self.visualization_state.offset_y) as usize };

            draw_utils::draw_grid(ctx, &mut canvas, &self.visualization_state, world_map, &self.world_state.robot_position)?;
            canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::new(400.0, 400.0)));
            
            //backpack
            //draw_utils::draw_backpack(&self.world_state.backpack, &mut canvas, tile_size, world_dimension);

            let x_tick_count = (tile_size * world_dimension as f32) + (tile_size*3.0);
            let y_tick_count = tile_size;
            let text_size = tile_size * 0.18;
            draw_utils::draw_text(&mut  canvas, x_tick_count, y_tick_count, Color::WHITE, text_size, format!("TICK: {}", self.tick_counter));

            if self.simulation_should_end() {
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