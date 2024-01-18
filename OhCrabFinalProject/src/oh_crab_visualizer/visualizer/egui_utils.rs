use std::collections::HashMap;

use egui::Image;
use egui_extras::{TableBuilder, Column};
use ggegui::{GuiContext, egui::{self, Layout}};
use robotics_lib::{world::{tile::Content, environmental_conditions::WeatherType}, interface::Direction};
use rstykrab_cache::{Record, Action};

use super::visualizer::{VisualizationState, WorldTime, MAX_ENERGY_LEVEL};

const COLON_KEY:u8 = 42;
const DIRECTION_UP:u8 = 0;
const DIRECTION_RIGHT:u8 = 1;
const DIRECTION_DOWN:u8 = 2;
const DIRECTION_LEFT:u8 = 3;

pub(super) struct EguiImages<'a> {
    content_images: HashMap<Content, Image<'a>>,
    weather_images: HashMap<WeatherType, (String, Image<'a>)>,
    digit_images: HashMap<u8, Image<'a>>,
    energy: Image<'a>,
    direction_images: HashMap<u8, Image<'a>>,
}

impl<'a> EguiImages<'a> {
    pub(super) fn init() -> EguiImages<'a> {
        let mut content_map:HashMap<Content, Image<'a>> = HashMap::new();
        content_map.insert(Content::Fish(0), egui::Image::new(egui::include_image!("assets\\content\\fish.png")));
        content_map.insert(Content::Water(0), egui::Image::new(egui::include_image!("assets\\content\\water.png")));
        content_map.insert(Content::Rock(0), egui::Image::new(egui::include_image!("assets\\content\\rock.png")));
        content_map.insert(Content::Tree(0), egui::Image::new(egui::include_image!("assets\\content\\tree.png")));
        content_map.insert(Content::Garbage(0), egui::Image::new(egui::include_image!("assets\\content\\garbage.png")));
        content_map.insert(Content::Fire, egui::Image::new(egui::include_image!("assets\\content\\fire.png")));
        content_map.insert(Content::Coin(0), egui::Image::new(egui::include_image!("assets\\content\\coin.png")));
        content_map.insert(Content::Bin(0..10), egui::Image::new(egui::include_image!("assets\\content\\bin.png")));
        content_map.insert(Content::Crate(0..10), egui::Image::new(egui::include_image!("assets\\content\\crate.png")));
        content_map.insert(Content::Bank(0..10), egui::Image::new(egui::include_image!("assets\\content\\bank.png")));
        content_map.insert(Content::Water(0), egui::Image::new(egui::include_image!("assets\\content\\water.png")));
        content_map.insert(Content::Market(0), egui::Image::new(egui::include_image!("assets\\content\\market.png")));
        content_map.insert(Content::Fish(0), egui::Image::new(egui::include_image!("assets\\content\\fish.png")));
        content_map.insert(Content::Building, egui::Image::new(egui::include_image!("assets\\content\\building.png")));
        content_map.insert(Content::Bush(0), egui::Image::new(egui::include_image!("assets\\content\\bush.png")));
        content_map.insert(Content::JollyBlock(0), egui::Image::new(egui::include_image!("assets\\content\\jollyBlock.png")));
        content_map.insert(Content::Scarecrow, egui::Image::new(egui::include_image!("assets\\content\\scarecrow.png")));

        let mut weather_map: HashMap<WeatherType, (String, Image<'a>)> = HashMap::new();
        weather_map.insert(WeatherType::Sunny, ("Sunny".to_owned(), egui::Image::new(egui::include_image!("assets\\weather\\sunny.png"))));
        weather_map.insert(WeatherType::Rainy, ("Rainy".to_owned(), egui::Image::new(egui::include_image!("assets\\weather\\rainy.png"))));
        weather_map.insert(WeatherType::Foggy, ("Foggy".to_owned(), egui::Image::new(egui::include_image!("assets\\weather\\foggy.png"))));
        weather_map.insert(WeatherType::TropicalMonsoon, ("Tropical monsoon".to_owned(), egui::Image::new(egui::include_image!("assets\\weather\\tropical_monsoon.png"))));
        weather_map.insert(WeatherType::TrentinoSnow, ("Trentino snow".to_owned(), egui::Image::new(egui::include_image!("assets\\weather\\trentino_snow.png"))));

        let mut digit_map:  HashMap<u8, Image<'a>> = HashMap::new();
        digit_map.insert(0,  egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_10.png")));
        digit_map.insert(1,  egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_01.png")));
        digit_map.insert(2,  egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_02.png")));
        digit_map.insert(3, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_03.png")));
        digit_map.insert(4, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_04.png")));
        digit_map.insert(5, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_05.png")));
        digit_map.insert(6, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_06.png")));
        digit_map.insert(7, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_07.png")));
        digit_map.insert(8, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_08.png")));
        digit_map.insert(9, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_09.png")));
        digit_map.insert(COLON_KEY, egui::Image::new(egui::include_image!("assets\\digits\\white\\digits_11.png")));

        let energy= egui::Image::new(egui::include_image!("assets\\energy.png"));

        let mut direction_map: HashMap<u8, Image<'a>> = HashMap::new();
        direction_map.insert(DIRECTION_UP, egui::Image::new(egui::include_image!("assets\\direction\\up.png")));
        direction_map.insert(DIRECTION_RIGHT, egui::Image::new(egui::include_image!("assets\\direction\\right.png")));
        direction_map.insert(DIRECTION_DOWN, egui::Image::new(egui::include_image!("assets\\direction\\down.png")));
        direction_map.insert(DIRECTION_LEFT, egui::Image::new(egui::include_image!("assets\\direction\\left.png")));

        EguiImages { content_images: content_map, weather_images: weather_map, digit_images: digit_map, energy:energy, direction_images: direction_map }
    }

    fn get_image_for_direction(&self, direction: &Direction) -> Image<'a> {
        match direction {
            Direction::Up => self.direction_images.get(&DIRECTION_UP).unwrap().clone(),
            Direction::Down => self.direction_images.get(&DIRECTION_DOWN).unwrap().clone(),
            Direction::Left => self.direction_images.get(&DIRECTION_LEFT).unwrap().clone(),
            Direction::Right => self.direction_images.get(&DIRECTION_RIGHT).unwrap().clone(),
        }
    }

    fn get_image_for_content(&self, content: &Content) -> Image<'a> {
        self.content_images.get(content).unwrap().clone()
    }
}


pub(super) fn draw_backpack(gui_ctx: &mut GuiContext, visualizatio_state: &VisualizationState, backpack: &HashMap<Content, usize>, egui_images: &EguiImages) {
    egui::Window::new("Backpack")
        .default_pos((visualizatio_state.grid_canvas_properties.grid_canvas_origin_x + visualizatio_state.grid_canvas_properties.grid_canvas_width + 40.0, 110.0))
        .show(&gui_ctx, |ui| {
            let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0);

            table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Content type");
                });
                header.col(|ui| {
                    ui.strong("Amount");
                });
            })
            .body(|mut body|
                for (content, amount) in backpack.iter() {
                    let row_height = 30.0 ;
                    body.row(row_height, |mut row| {
                        row.col(|ui| {
                            let image = egui_images.content_images.get(content).unwrap();
                            ui.add(image.clone());
                            ui.label(content.to_string());
                        });
                        row.col(|ui| {
                            ui.label(amount.to_string());
                        });
                    });
        });
    });
}

pub(super) fn draw_time(gui_ctx: &mut GuiContext, visualizatio_state: &VisualizationState, world_time: &WorldTime, num_ticks: usize, simulation_finished: bool, egui_images: &EguiImages){
    egui::Window::new("Time")
        .default_pos((visualizatio_state.grid_canvas_properties.grid_canvas_origin_x + visualizatio_state.grid_canvas_properties.grid_canvas_width + 215.0, 110.0))
        .show(&gui_ctx, |ui| {
            //ui.label(format!("{}:{}", world_time.hours, world_time.minutes));
            ui.horizontal(|ui| {
                let (first, second) = get_digits(world_time.hours);
                let image = egui_images.digit_images.get(&first).unwrap();
                ui.add(image.clone());
                let image = egui_images.digit_images.get(&second).unwrap();
                ui.add(image.clone());
                let colon_image = egui_images.digit_images.get(&COLON_KEY).unwrap();
                ui.add(colon_image.clone());
                let (first, second) = get_digits(world_time.minutes);
                let image = egui_images.digit_images.get(&first).unwrap();
                ui.add(image.clone());
                let image = egui_images.digit_images.get(&second).unwrap();
                ui.add(image.clone());
            });
            ui.horizontal(|ui| {
                let (weather_name, weather_image) = egui_images.weather_images.get(&world_time.weather).unwrap();
                ui.strong("Weather: ");
                ui.label(format!("{}", weather_name));
                ui.add(weather_image.clone());
            });
            ui.horizontal(|ui| {
                ui.strong("Day number: ");
                ui.label(format!("{}", world_time.day_counter));
            });
            ui.horizontal(|ui| {
                ui.strong("Tick number: ");
                ui.label(format!("{}", num_ticks));
            });
            if simulation_finished {
                ui.strong("SIMULATION FINISHED");
            }
        });
}

fn get_digits(number: u8) -> (u8, u8) {
    let second = number % 10;
    let first = (number / 10) % 10;
    (first, second)
}

pub(super) fn draw_energy_bar(ctx: &egui::Context, visualizatio_state: &VisualizationState, robot_energy: usize, energy_difference: i32, egui_images: &EguiImages) {
    let energy_percentage = robot_energy as f32 / MAX_ENERGY_LEVEL as f32; // MAX_ENERGY is the maximum energy value

    egui::Window::new("Robot energy")
        .default_pos((visualizatio_state.grid_canvas_properties.grid_canvas_origin_x + visualizatio_state.grid_canvas_properties.grid_canvas_width + 40.0, 15.0))
        .collapsible(false)
        //.default_width(500.0)
        .show(ctx, |ui| {
            let energy_bar = egui::ProgressBar::new(energy_percentage)
            .fill(egui::Color32::from_rgb(255, 51, 0))
            .text(format!("{} / {}", robot_energy, MAX_ENERGY_LEVEL));
            //.show_percentage();
            ui.add(energy_bar);

            ui.horizontal(|ui| {
                ui.label("Energy bilance per tick:");
                let plus_or_not = if energy_difference > 0 {"+"} else {""};
                ui.strong(format!("{plus_or_not}{energy_difference}"));
                ui.add(egui_images.energy.clone());
            });
        });
}

pub(super) fn draw_rizler_message(ctx: &egui::Context, visualizatio_state: &VisualizationState, riz_message: &Option<String>) {
    egui::Window::new("Rizzler")
        .default_pos((visualizatio_state.grid_canvas_properties.grid_canvas_origin_x + visualizatio_state.grid_canvas_properties.grid_canvas_width +  280.0, 500.0))
        //.min_width(500.0)
        .collapsible(false)
        .show(ctx, |ui| {
            ui.strong("Robot say: ");
            match riz_message {
                Some(message) => {ui.label(message);},
                None => {ui.label("Nothing much.");},
            }
        });
}

pub(super) fn draw_history_cache(gui_ctx: &mut GuiContext, visualizatio_state: &VisualizationState, cached_actions: &Vec<&Record>, egui_images: &EguiImages) {
    egui::Window::new("Robot action history")
        .default_pos((visualizatio_state.grid_canvas_properties.grid_canvas_origin_x + visualizatio_state.grid_canvas_properties.grid_canvas_width + 40.0, 500.0))
        .show(gui_ctx, |ui| {
            let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::remainder())
            .column(Column::auto())
            .min_scrolled_height(0.0);

            table
            .header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("Position");
                });
                header.col(|ui| {
                    ui.strong("Action");
                });
                header.col(|ui| {
                    ui.label("        ");
                });
            })
            .body(|mut body|
                for action_record in cached_actions.iter() {
                    let row_height = 20.0 ;
                    body.row(row_height, |mut row| {
                        let (action_label, image) = cache_action_to_visualization(action_record.action(), &egui_images);
                        row.col(|ui| {
                            ui.label(format!("{},{}", action_record.position.0, action_record.position.1));
                        });
                        row.col(|ui| {
                            ui.label(action_label);
                        });
                        row.col(|ui| {
                            if let Some(image) = image {
                                ui.add(image);
                            }
                        });
                    });
        });
    });
}

fn cache_action_to_visualization<'a>(action: &Action, images: &EguiImages<'a>) -> (String, Option<Image<'a>>) {
    match action {
        Action::Craft(content) => (format!("Craft: {}", content), Some(images.get_image_for_content(content))),
        Action::Destroy(direction) => (format!("Destroy: {}", direction_to_string(direction)), Some(images.get_image_for_direction(direction))),
        Action::DiscoverTiles(tiles) => (format!("Discover tiles: {:?}", tiles), None),
        Action::GetScore() => (format!("Get score"), None),
        Action::Go(direction) => (format!("Go: {:}", direction_to_string(direction)), Some(images.get_image_for_direction(direction))),
        Action::LookAtSky() => (format!("Look at sky"), None),
        Action::OneDirectionView(direction, distance) => (format!("One directional view: {:}, distance {}", direction_to_string(direction), distance), Some(images.get_image_for_direction(direction)) ),
        Action::Put(content, amount, direction) => (format!("Put {} of {}: {}", amount, content, direction_to_string(direction)), Some(images.get_image_for_content(content))),
        Action::RobotMap() => (format!("Robot map"), None),
        Action::RobotView() => (format!("Robot view"), None),
        Action::Teleport((x,y)) => (format!("Teleport ({},{})", x, y), None),
        Action::WhereAmI() => (format!("Where am I"), None)
    }        
}

fn direction_to_string(direction: &Direction) -> String {
    let string = match direction {
        Direction::Up => "Up",
        Direction::Down => "Down",
        Direction::Left => "Left",
        Direction::Right => "Right",
    };
    string.to_owned()
}
