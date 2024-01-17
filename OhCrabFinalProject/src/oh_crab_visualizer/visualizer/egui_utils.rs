use std::collections::HashMap;

use egui::Image;
use egui_extras::{TableBuilder, Column};
use ggegui::{GuiContext, egui::{self, Layout}};
use robotics_lib::world::tile::Content;

use super::{visualizer::{VisualizationState, GRID_CANVAS_ORIGIN_X}, draw_utils::get_content_string};

pub(super) struct EguiImages<'a> {
    content_images:HashMap<Content, Image<'a>>,
}

impl<'a> EguiImages<'a> {
    pub(super) fn init() -> EguiImages<'a> {
        let mut map:HashMap<Content, Image<'a>> = HashMap::new();
        map.insert(Content::Fish(0), egui::Image::new(egui::include_image!("assets\\content\\fish.png")));
        map.insert(Content::Water(0), egui::Image::new(egui::include_image!("assets\\content\\water.png")));
        map.insert(Content::Rock(0), egui::Image::new(egui::include_image!("assets\\content\\rock.png")));
        map.insert(Content::Tree(0), egui::Image::new(egui::include_image!("assets\\content\\tree.png")));
        map.insert(Content::Garbage(0), egui::Image::new(egui::include_image!("assets\\content\\garbage.png")));
        map.insert(Content::Fire, egui::Image::new(egui::include_image!("assets\\content\\fire.png")));
        map.insert(Content::Coin(0), egui::Image::new(egui::include_image!("assets\\content\\coin.png")));
        map.insert(Content::Bin(0..10), egui::Image::new(egui::include_image!("assets\\content\\bin.png")));
        map.insert(Content::Crate(0..10), egui::Image::new(egui::include_image!("assets\\content\\crate.png")));
        map.insert(Content::Bank(0..10), egui::Image::new(egui::include_image!("assets\\content\\bank.png")));
        map.insert(Content::Water(0), egui::Image::new(egui::include_image!("assets\\content\\water.png")));
        map.insert(Content::Market(0), egui::Image::new(egui::include_image!("assets\\content\\market.png")));
        map.insert(Content::Fish(0), egui::Image::new(egui::include_image!("assets\\content\\fish.png")));
        map.insert(Content::Building, egui::Image::new(egui::include_image!("assets\\content\\building.png")));
        map.insert(Content::Bush(0), egui::Image::new(egui::include_image!("assets\\content\\bush.png")));
        map.insert(Content::JollyBlock(0), egui::Image::new(egui::include_image!("assets\\content\\jollyBlock.png")));
        map.insert(Content::Scarecrow, egui::Image::new(egui::include_image!("assets\\content\\scarecrow.png")));

        EguiImages { content_images: map }
    }
}

pub(super) fn draw_backpack(gui_ctx: &mut GuiContext, visualizatio_state: &VisualizationState, backpack: &HashMap<Content, usize>, egui_images: &EguiImages) {
    egui::Window::new("Backpack")
        .default_pos((visualizatio_state.grid_canvas_properties.grid_canvas_origin_x + visualizatio_state.grid_canvas_properties.grid_canvas_width + 40.0, 15.0))
        .show(&gui_ctx, |ui| {
            let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .cell_layout(Layout::left_to_right(egui::Align::Center))
            .column(Column::auto())
            .column(Column::auto())
            // .column(Column::initial(100.0).range(40.0..=300.0))
            // .column(Column::initial(100.0).at_least(40.0).clip(true))
            // .column(Column::remainder())
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
                        //row.set_selected(self.selection.contains(&row_index));

                        row.col(|ui| {
                            //ui.add(egui::Image::new(egui::include_image!("assets\\content\\fish.png")));
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