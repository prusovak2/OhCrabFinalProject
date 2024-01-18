use core::num;
use std::{collections::HashMap, fmt::format};

use egui::{Image, ahash::HashMapExt, Ui, Context, Rect, TextStyle};
use egui_extras::{TableBuilder, Column};
use ggegui::{GuiContext, egui::{self, Layout}};
use robotics_lib::world::{tile::Content, environmental_conditions::WeatherType};

use super::visualizer::{VisualizationState, WorldTime, MAX_ENERGY_LEVEL};

const COLON_KEY:u8 = 42;

pub(super) struct EguiImages<'a> {
    content_images: HashMap<Content, Image<'a>>,
    weather_images: HashMap<WeatherType, (String, Image<'a>)>,
    digit_images: HashMap<u8, Image<'a>>,
    energy: Image<'a>
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

        EguiImages { content_images: content_map, weather_images: weather_map, digit_images: digit_map, energy:energy }
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
    let energy_bar_width = 200.0;
    let energy_bar_height = 20.0;
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
                ui.add(egui_images.energy.clone());
                let plus_or_not = if energy_difference > 0 {"+"} else {""};
                ui.strong(format!("{plus_or_not}{energy_difference}"));
            });
        });
}