use std::{sync::mpsc::{Receiver, self}, collections::HashMap};

use egui::Visuals;
use egui_extras::install_image_loaders;
use ggegui::{egui::{self}, Gui, GuiContext};
use ggez::{event::{EventHandler, self}, graphics::{self, DrawParam}, GameError, glam};
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use rand::{rngs::ThreadRng, seq::SliceRandom};
use robotics_lib::{runner::Runner, utils::LibError as RobotError, event::events::Event as RobotEvent, world::{tile::{Tile, Content}, environmental_conditions::{WeatherType, EnvironmentalConditions}}};
use rstykrab_cache::Cache;

use crate::{oh_crab_visualizer::visualizer::{draw_utils::{self, GridCanvasProperties}, egui_utils}, println_d};

use super::{visualizable_robot::{VisualizableRobot, RobotCreator, InitStateChannelItem}, Coord, visualizer_event_listener::{VisualizerEventListener, ChannelItem, InterfaceInvocation}, egui_utils::EguiImages, draw_utils::GgezImages};

pub(super) const TILE_SIZE_MIN:f32 = 5.0;
pub(super) const TILE_SIZE_MAX:f32 = 120.8;
pub(super) const CONTENT_TILE_SIZE_LIMIT:f32 = 50.0;

pub(super) const DEFAULT_TILE_SIZE:f32 = 60.4;
pub(super) const GRID_FRAME_WIDTH: f32 = 20.0;
pub(super) const GRID_CANVAS_ORIGIN_X: f32 = 200.0 + GRID_FRAME_WIDTH;
pub(super) const GRID_CANVAS_ORIGIN_Y: f32 = 0.0 + GRID_FRAME_WIDTH;

pub(super) const MAX_ENERGY_LEVEL: usize = 1000;

pub struct OhCrabVisualizer {
    runner: Runner,
    robot_receiver: Receiver<ChannelItem>,
    map_receiver: Receiver<InitStateChannelItem>,
    action_cache: Cache,
    rng: ThreadRng,
    
    gui: Gui,
    egui_images: EguiImages<'static>,
    ggez_images: GgezImages,

    // configuration
    run_mode: RunMode,

    // state
    tick_counter: usize,
    world_state: WorldState,
    world_time: WorldTime,
    world_tick_in_progress: bool,
    visualization_state: VisualizationState
}

/// Represents state of robotic lib world as it is known to visualizer
/// 
struct WorldState {
    world_map: Option<Vec<Vec<Tile>>>,
    robot_position: Option<Coord>,
    backpack: HashMap<Content, usize>,
    robot_energy: usize,
    previous_tick_energy_difference: i32,
    current_tick_energy_difference: i32,
    rizler_message: Option<String>,
    rizzler_messages: Vec<String>
}

impl WorldState {
    fn empty() -> WorldState {
        WorldState {
            world_map: None,
            robot_position: None,
            backpack: HashMap::new(),
            robot_energy: 0,
            current_tick_energy_difference: 0,
            previous_tick_energy_difference: 0,
            rizler_message: None,
            rizzler_messages: Vec::new()
        }
    }
}

pub(super) struct WorldTime {
    pub(super) day_counter: u64,
    pub(super) hours: u8,
    pub(super) minutes: u8,
    pub(super) weather: WeatherType
}

impl Default for WorldTime {
    fn default() -> Self {
        Self { day_counter: Default::default(), hours: Default::default(), minutes: Default::default(), weather: WeatherType::Sunny }
    }
}

impl WorldTime {
    pub(crate) fn update_from_env_conditions(&mut self, env_conds: &EnvironmentalConditions) {
        let (hours, minutes) = WorldTime::parse_time(&env_conds.get_time_of_day_string());
        self.hours = hours;
        self.minutes = minutes;
        self.weather = env_conds.get_weather_condition();
    }

    fn parse_time(time_str: &str) -> (u8, u8) {
        let parts: Vec<&str> = time_str.split(':').collect();

        if parts.len() != 2 {
            panic!("Invalid time format. Expected format: hh:mm");
        }

        let hours: u8 = parts[0].parse().expect("Failed to parse hours");
        let minutes: u8 = parts[1].parse().expect("Failed to parse minutes");

        (hours, minutes)
    }
}

/// Represents state given by visualizer gui settings
/// 
#[derive(Default)]
pub(super) struct VisualizationState {
    offset_x: f32,
    offset_y: f32,
    should_focus_on_robot: bool,
    pub(super) content_display_option: ContentDisplayOptions,
    pub(super) grid_canvas_properties: GridCanvasProperties
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
}

impl OhCrabVisualizerConfig {
    pub fn new(run_mode: RunMode, use_sound: bool) -> Self {
        OhCrabVisualizerConfig {
            run_mode,
            use_sound,
        }
    }
}

#[derive(PartialEq, Default, Debug)]
pub(super) enum ContentDisplayOptions { 
    #[default] Images,
    Lables,
    No 
}

impl OhCrabVisualizer {
    pub fn new(robot_creator: impl RobotCreator, mut world_generator: OxAgWorldGenerator, config: OhCrabVisualizerConfig) -> OhCrabVisualizer {
        let (robot_sender, robot_receiver) = mpsc::channel::<ChannelItem>();
        let (map_sender, map_receiver) = mpsc::channel::<InitStateChannelItem>();

        let visualizer_data_sender = VisualizerEventListener::new(robot_sender, config.use_sound);
        let robot = robot_creator.create(visualizer_data_sender);
        let visualizable_robot = VisualizableRobot::new(robot, map_sender);

        let runner = Runner::new(Box::new(visualizable_robot), &mut world_generator).expect("Runner creation failed");

        OhCrabVisualizer {
            runner: runner,
            robot_receiver: robot_receiver,
            map_receiver,
            action_cache: Cache::new(50),
            gui: Gui::default(),
            run_mode: config.run_mode,
            tick_counter: 0,
            world_state: WorldState::empty(),
            world_time: WorldTime::default(),
            visualization_state: VisualizationState::default(),
            world_tick_in_progress: false,
            egui_images: EguiImages::init(),
            ggez_images: GgezImages::empty(),
            rng: rand::thread_rng()
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
            .add_resource_path("./assets")
            .window_mode(ggez::conf::WindowMode::default()
            .resizable(true)
            .maximized(true));

        match context_builder.build() {
            Ok((ctx, event_loop)) => {
                self.gui = Gui::new(&ctx);
                self.ggez_images = GgezImages::init(&ctx);
                event::run(ctx, event_loop, self);
            }
            Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error))
        }
    }

    fn update_riz_messages_for_tick(&mut self) {
        match self.world_state.rizzler_messages.choose(&mut self.rng) {
            Some(message) => {self.world_state.rizler_message = Some(message.clone())},
            None => {},
        }
        self.world_state.rizzler_messages.clear();
    }

    fn update_energy_difference_for_tick(&mut self) {
        self.world_state.previous_tick_energy_difference = self.world_state.current_tick_energy_difference;
        self.world_state.current_tick_energy_difference = 0;
    }

    fn do_world_tick(&mut self) -> Result<(), OhCrabVisualizerError> {
        self.update_energy_difference_for_tick();
        self.update_riz_messages_for_tick();
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

        // self.world_state.backpack.insert(Content::Rock(0), 5);
        // self.world_state.backpack.insert(Content::Tree(0), 3);
        // self.world_state.backpack.insert(Content::Garbage(0), 10);
        // self.world_state.backpack.insert(Content::Fire, 1);
        // self.world_state.backpack.insert(Content::Coin(0), 50);
        // self.world_state.backpack.insert(Content::Bin(0..10), 2);
        // self.world_state.backpack.insert(Content::Crate(0..10), 3);
        // self.world_state.backpack.insert(Content::Bank(0..10), 1);
        // self.world_state.backpack.insert(Content::Water(0), 8);
        // self.world_state.backpack.insert(Content::Market(0), 1);
        // self.world_state.backpack.insert(Content::Fish(0), 15);
        // self.world_state.backpack.insert(Content::Building, 2);
        // self.world_state.backpack.insert(Content::Bush(0), 5);
        // self.world_state.backpack.insert(Content::JollyBlock(0), 2);
        // self.world_state.backpack.insert(Content::Scarecrow, 1);
        match received_map {
            Ok(item) => {
                self.visualization_state.grid_canvas_properties = GridCanvasProperties::build(canvas_size, item.state.world_map.len());
                let robot_pos = item.state.robot_position;
                println_d!("VISUALIZER RECEIVED MAP with robot position {:?} and robot energy {:?}", (robot_pos.x, robot_pos.y), item.state.robot_energy);
                self.world_state.world_map = Some(item.state.world_map);
                self.world_state.robot_position = Some(robot_pos);
                self.world_state.robot_energy = item.state.robot_energy;
                self.focus_on_robot();
                Ok(())
            }
            Err(_) => todo!(),
        }
    }

    fn focus_on_robot(&mut self) {
        if let Some(robot_pos) = &self.world_state.robot_position{
            println_d!("Focusing on robot on position {:?}", robot_pos);
            let world_dimension = self.visualization_state.grid_canvas_properties.world_dimension;
            // x
            let half_of_columns_to_display = self.visualization_state.grid_canvas_properties.num_columns_to_display() / 2;
            if robot_pos.x <= (world_dimension - half_of_columns_to_display) && robot_pos.x >= half_of_columns_to_display { // prevent moving camera when robot is close to edge 
                self.visualization_state.offset_x = f32::max(0.0, robot_pos.x as f32  - half_of_columns_to_display as f32);
            }
            // y
            let half_of_rows_to_display =  self.visualization_state.grid_canvas_properties.num_rows_to_display() /2 ;
            if robot_pos.y <= ( world_dimension - half_of_rows_to_display) && robot_pos.y >= half_of_rows_to_display { // prevent moving camera when robot is close to edge 
                self.visualization_state.offset_y = f32::max(0.0, robot_pos.y as f32 - half_of_rows_to_display as f32) ;
            }

            println_d!("Focused");
        }
    }

    fn zoom_on_robot(&mut self) {
        if let Some(_) = &self.world_state.robot_position{
            self.visualization_state.grid_canvas_properties.tile_size = DEFAULT_TILE_SIZE;
            self.focus_on_robot();
        }
    }

    fn move_camera_if_world_is_zoomed_out(&mut self) {
        // move camera if the world is zoomed out
        if let Some(world_map) = &self.world_state.world_map {
            let (scroll_limit_x, scroll_limit_y) = self.visualization_state.get_scroll_limit(world_map.len());
            self.visualization_state.offset_x = f32::min(self.visualization_state.offset_x, scroll_limit_x);
            self.visualization_state.offset_y = f32::min(self.visualization_state.offset_y, scroll_limit_y);
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

    fn add_control_panel(&mut self, gui_ctx: &mut GuiContext) -> Result<(), OhCrabVisualizerError> {
        let mut res: Result<(), OhCrabVisualizerError> = Ok(());
        egui::Window::new("Scroll world")
        .default_pos((5.0, 10.0))
        .show(&gui_ctx, |ui: &mut egui::Ui| {
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
            ui.label("Content: ");
            ui.horizontal(|ui| {
                ui.radio_value(&mut self.visualization_state.content_display_option, ContentDisplayOptions::Images, "Images");
                ui.radio_value(&mut self.visualization_state.content_display_option, ContentDisplayOptions::Lables, "Labels");
                ui.radio_value(&mut self.visualization_state.content_display_option, ContentDisplayOptions::No, "None"); 
            });
        });

        if res.is_err() {
            return res;
        }
        Ok(())
    }

    fn register_egui_windows(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        let gui_ctx = &mut self.gui.ctx();
        gui_ctx.set_visuals(Visuals::dark());
       
        self.add_control_panel(gui_ctx)?;
        egui_utils::draw_backpack(gui_ctx, &self.visualization_state, &self.world_state.backpack, &self.egui_images);
        egui_utils::draw_time(gui_ctx, &self.visualization_state, &self.world_time, self.tick_counter, self.simulation_should_end(), &self.egui_images);
        egui_utils::draw_energy_bar(gui_ctx, &self.visualization_state, self.world_state.robot_energy, self.world_state.previous_tick_energy_difference, &self.egui_images);
        let cached_actions = self.action_cache.get_recent_actions(self.action_cache.get_size()).unwrap();
        egui_utils::draw_history_cache(gui_ctx, &self.visualization_state, &cached_actions, &self.egui_images);
        egui_utils::draw_rizler_message(gui_ctx, &self.visualization_state, &self.world_state.rizler_message);
        
        self.gui.update(ctx);
        Ok(())
    }

    #[inline]
    fn process_time_changed_event(&mut self, env_conditions: EnvironmentalConditions) {
        println_d!("VISUALIZER: received EVENT time changed {:?}", (env_conditions));
        self.world_time.update_from_env_conditions(&env_conditions);
    }

    #[inline]
    fn process_day_changed_event(&mut self, env_conditions: EnvironmentalConditions) {
        println_d!("VISUALIZER: received EVENT day changed {:?}", (env_conditions));
        self.world_time.update_from_env_conditions(&env_conditions);
        self.world_time.day_counter +=1;
    }

    #[inline]
    fn process_energy_regarged_event(&mut self, amount: usize) {
        println_d!("VISUALIZER: received EVENT energy recharged {:?}", (amount));
        self.world_state.robot_energy += amount;
        self.world_state.current_tick_energy_difference += amount as i32;
    }

    #[inline]
    fn process_energy_consumed_event(&mut self, amount: usize) {
        println_d!("VISUALIZER: received EVENT energy consumed {:?}", (amount));
        let to_subtract = usize::min(amount, self.world_state.robot_energy); // to prevent subtract with overflow
        self.world_state.robot_energy -= to_subtract;
        self.world_state.current_tick_energy_difference -= to_subtract as i32;
    }

    #[inline]
    fn process_moved_event(&mut self, robot_x: usize, robot_y :usize) {
        println_d!("VISUALIZER: received robot moved {:?}", (robot_x, robot_y));
        self.world_state.robot_position = Some(Coord{x:robot_x, y:robot_y });
    }

    #[inline]
    fn process_tile_content_update_event(&mut self, tile: Tile, tile_x: usize, tile_y: usize) {
        println_d!("VISUALIZER: received tile content update.");
        if let Some(world_map) = &mut self.world_state.world_map{
            world_map[tile_y][tile_x] = tile;
        }
    }

    #[inline]
    fn process_added_to_backpack_event(&mut self, content: Content, amount: usize) {
        println_d!("VISUALIZER: added to backpack: {:?}, {:?}.", content, amount);
        *self.world_state.backpack.entry(content.clone()).or_insert(0) += amount;
        println_d!("   current amount {:?} of {:?} after add", self.world_state.backpack.get(&content), content.to_string());
    }

    #[inline]
    fn process_removed_from_backpack_event(&mut self,  content: Content, amount: usize) {
        println!("VISUALIZER: removed from backpack: {:?}, {:?}.", content, amount);
        if let Some(current_amount) = self.world_state.backpack.get_mut(&content) {
            if *current_amount > amount {
                *current_amount -= amount;
                println_d!("   current amount {:?} of {:?} after remove", *current_amount, content.to_string());
            } else {
                self.world_state.backpack.remove(&content);
            }
        }
    }

    #[inline]
    fn process_interface_invocation_record(&mut self, interface_invocation: InterfaceInvocation) {
        println_d!("VISULAZER: received interface invocation: {:?}", interface_invocation);

        // history cache
        self.action_cache.add_record(interface_invocation.interface_action, (interface_invocation.robot_position.x, interface_invocation.robot_position.y));
       
        //rizzler
        if let Some(meesage) = interface_invocation.riz_message {
            self.world_state.rizzler_messages.push(meesage);
        }
    }

    fn process_robotic_lib_event(&mut self) -> Result<(), OhCrabVisualizerError> {
                let received_state = self.robot_receiver.try_recv();

                match received_state {
                    Ok(channel_item) => {
                        match  channel_item {
                            ChannelItem::EventChannelItem(event) => {
                                match event {
                                    // RobotEvent::Ready => todo!(),
                                    // RobotEvent::Terminated => todo!(),
                                    RobotEvent::TimeChanged(env_conditions) => {
                                        self.process_time_changed_event(env_conditions);
                                    }
                                    RobotEvent::DayChanged(env_conditions) => {
                                        self.process_day_changed_event(env_conditions);
                                    }
                                    RobotEvent::EnergyRecharged(amount) => {
                                        self.process_energy_regarged_event(amount);
                                    },
                                    RobotEvent::EnergyConsumed(amount) => {
                                        self.process_energy_consumed_event(amount);
                                    },
                                    RobotEvent::Moved(_, (robot_y, robot_x)) => { // BEWARE: library has x and y switched in Move event
                                        self.process_moved_event(robot_x, robot_y);
                                    }
                                    RobotEvent::TileContentUpdated(tile, (tile_y, tile_x)) => {
                                        self.process_tile_content_update_event(tile, tile_x, tile_y);
                                    }
                                    RobotEvent::AddedToBackpack(content, amount) => {
                                        self.process_added_to_backpack_event(content, amount);
                                    }
                                    RobotEvent::RemovedFromBackpack(content, amount) => {
                                        self.process_removed_from_backpack_event(content, amount);
                                    }
                                    _ => {
                                        println_d!("VISUALIZER: {:?}", event);
                                    }
                                }
                            }
                            ChannelItem::InterfaceChannelItem(interface_invocation) => {
                                self.process_interface_invocation_record(interface_invocation);
                            }
                        }
                    }
                    Err(std::sync::mpsc::TryRecvError::Empty) => {
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
}

impl EventHandler<OhCrabVisualizerError> for OhCrabVisualizer {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        if self.tick_counter == 0 {
            let (x, y) = ctx.gfx.size();
            let size = f32::min(x, y);
            self.init_state(size)?;
        }
        
        self.register_egui_windows(ctx)?;
        self.move_camera_if_world_is_zoomed_out();

        if self.simulation_should_end() {
            //_ctx.request_quit();
            println_d!("empty update");
            return Ok(());
        }

        self.process_robotic_lib_event()?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), OhCrabVisualizerError> {
        if let Some(world_map) = &self.world_state.world_map {
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

            // draw grid
            draw_utils::draw_grid(ctx, &mut canvas, &self.visualization_state, world_map, &self.world_state.robot_position, &self.ggez_images)?;

            // draw gui
            canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::new(400.0, 400.0)));
            
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