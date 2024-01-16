use std::collections::HashMap;

use ggez::{graphics::{Canvas, Color, self, TextFragment, Image, ImageFormat, Drawable}, Context, glam};
use robotics_lib::world::tile::{Tile, TileType, Content};

use crate::println_d;

use super::{Coord, visualizer::{OhCrabVisualizerError, self}};

#[derive(Default)]
pub(super) struct GridCanvasProperties {
    pub(super) tile_size: f32,
    pub(super) grid_canvas_width: f32, 
    pub(super) grid_canvas_height: f32,
    pub(super) grid_canvas_origin_x: f32,
    pub(super) grid_canvas_origin_y: f32,
}

impl GridCanvasProperties {
    pub(super) fn num_rows_to_display(&self) -> u32 {
        (self.grid_canvas_height / self.tile_size).floor() as u32
    }

    pub(super) fn num_columns_to_display(&self) -> u32 {
        (self.grid_canvas_width / self.tile_size).floor() as u32
    }

    pub(super) fn build(canvas_total_size: f32) -> GridCanvasProperties {
        GridCanvasProperties {
            tile_size: visualizer::TILE_SIZE,
            grid_canvas_height: canvas_total_size - 150.0,
            grid_canvas_width: canvas_total_size - 150.0,
            grid_canvas_origin_x: visualizer::GRID_CANVAS_ORIGIN_X,
            grid_canvas_origin_y: visualizer::GRID_CANVAS_ORIGIN_Y,
        }
    } 
}

pub(super) fn draw_grid(
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
        canvas_props: &GridCanvasProperties,
        tile_offset: &Coord,
        world_map: &Vec<Vec<Tile>>) 
    -> Result<(), OhCrabVisualizerError> {

    let world_dimension = world_map.len();
    let rows_to_display = canvas_props.num_rows_to_display();
    let columns_to_display = canvas_props.num_columns_to_display();

    let last_column = usize::min(world_dimension, tile_offset.x + (columns_to_display as usize));
    let last_row = usize::min(world_dimension, tile_offset.y + (rows_to_display as usize));

    draw_grid_frame(ctx, canvas, canvas_props)?;

    for y in tile_offset.y..last_row {
        for x in tile_offset.x..last_column {
            let tile: &Tile = &world_map[y][x]; 
            draw_tile(tile, ctx, canvas, x-tile_offset.x, y-tile_offset.y, canvas_props.tile_size, canvas_props.grid_canvas_origin_x, canvas_props.grid_canvas_origin_y)?;
        }
    }
    Ok(())
}

fn draw_grid_frame(ctx: &mut Context, canvas: &mut Canvas, canvas_props: &GridCanvasProperties) -> Result<(), OhCrabVisualizerError> {
    let res = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            canvas_props.grid_canvas_origin_x - visualizer::GRID_FRAME_WIDTH,
            canvas_props.grid_canvas_origin_y - visualizer::GRID_FRAME_WIDTH,
            canvas_props.grid_canvas_width + (visualizer::GRID_FRAME_WIDTH * 2.0),
            canvas_props.grid_canvas_width  + (visualizer::GRID_FRAME_WIDTH * 2.0),
        ),
        Color::from_rgb(128, 128, 128)
    );
    match res {
        Ok(rect) => {
            canvas.draw(&rect, graphics::DrawParam::default());
            Ok(())
        }
        Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error))
    }
}

pub(super) fn draw_tile(tile: &Tile, ctx: &mut Context, canvas: &mut Canvas, x: usize, y:usize, tile_size: f32, grid_canvas_origin_x: f32, grid_canvas_origin_y: f32) -> Result<(), OhCrabVisualizerError> {
    let color = get_tile_color(&tile.tile_type);
    let res = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            (x as f32 * tile_size) + grid_canvas_origin_x,
            (y as f32 * tile_size) + grid_canvas_origin_y,
            tile_size,
            tile_size,
        ),
        color
    );

    match res {
        Ok(rect) => {
            canvas.draw(&rect, graphics::DrawParam::default());

            // TODO: use draw_text fn
            let center_x = ((x as f32 + 0.05) * tile_size) + grid_canvas_origin_x;
            let center_y = ((y as f32 + 0.75) * tile_size) + grid_canvas_origin_y;
            let text_size = tile_size * 0.18;
            let dest_point = ggez::glam::Vec2::new(center_x, center_y);
            let text_color = invert_color(&color);
            let text_fragment = TextFragment{
                color: Some(text_color),
                text: get_content_string(&tile.content),
                font: None,
                scale: None
            };
            canvas.draw(
                graphics::Text::new(text_fragment)
                    .set_scale(text_size),
                dest_point,
            );
            Ok(())
        }
        Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error))
    }
    
}

pub(super) fn draw_backpack(backpack: &HashMap<Content, usize>, canvas: &mut Canvas, tile_size: f32, world_dimension:usize) {
    let x_backpack = (tile_size * world_dimension as f32) + (tile_size);
    let mut y_backpack = tile_size;
    let text_size = tile_size * 0.2;
    let text_color = Color::WHITE;

    draw_text(canvas, x_backpack, y_backpack - text_size * 1.1, text_color, text_size, "BACKPACK:".to_owned());
    for (content, amount) in backpack {
        draw_text(canvas, x_backpack, y_backpack, text_color, text_size, format!("{}({})", content.to_string(), amount));
        y_backpack += text_size * 1.1;
    }
}

pub(super) fn draw_robot(robot_position: &Coord, ctx: &mut Context, canvas: &mut Canvas, tile_size: f32) -> Result<(), OhCrabVisualizerError> {
    let x = robot_position.x;
    let y = robot_position.y;
    let center_x = (x as f32 + 0.25) * tile_size;
    let center_y = (y as f32 + 0.25) * tile_size;

    let circle_radius = tile_size * 0.2;

    let res = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::fill(),
        glam::Vec2::new(center_x, center_y),
        circle_radius,
        0.4, // Segments (adjust based on your needs)
        Color::BLACK,
    );
    match res {
        Ok(circle) => {
            canvas.draw(&circle, graphics::DrawParam::default());
            Ok(())
        }
        Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error))
    }
}

pub(super) fn draw_text(canvas: &mut Canvas, x: f32, y:f32, color: Color, size:f32, text: String ) {
    let dest_point = ggez::glam::Vec2::new(x, y);
    let text_fragment = TextFragment{
        color: Some(color),
        text: text,
        font: None,
        scale: None
    };
    canvas.draw(
    graphics::Text::new(text_fragment)
        .set_scale(size),
    dest_point,
    );
}

fn get_tile_color(tile_type: &TileType) -> Color {
    match tile_type {
        TileType::DeepWater => Color::from_rgb(20, 21, 123),      
        TileType::ShallowWater => Color::from_rgb(125, 245, 234),     
        TileType::Sand => Color::from_rgb(246, 213, 111),            
        TileType::Grass => Color::from_rgb(126, 208, 64),            
        TileType::Street => Color::from_rgb(128, 128, 128),           
        TileType::Hill => Color::from_rgb(18, 171, 67),             
        TileType::Mountain => Color::from_rgb(123, 62, 20),       
        TileType::Snow => Color::from_rgb(255, 255, 255),          
        TileType::Lava => Color::from_rgb(241, 56, 22),            
        TileType::Teleport(_) => Color::from_rgb(147, 35, 238),  
        TileType::Wall => Color::from_rgb(248, 199, 237)
    }
}

fn get_content_string(content: &Content) -> String {
    match content {
        Content::Rock(val) => format!("Rock({})", val),
        Content::Tree(val) => format!("Tree({})", val),
        Content::Garbage(val) => format!("Garbage({})", val),
        Content::Fire => String::from("Fire"),
        Content::Coin(val) => format!("Coin({})", val),
        Content::Bin(val) => format!("Bin({},{})", val.start, val.end),
        Content::Crate(val) => format!("Crate({},{})", val.start, val.end),
        Content::Bank(val) => format!("Bank({},{})", val.start, val.end),
        Content::Water(val) => format!("Water({})", val),
        Content::Market(val) => format!("Market({})", val),
        Content::Fish(val) => format!("Fish({})", val),
        Content::Building => String::from("Building"),
        Content::Bush(val) => format!("Bush({})", val),
        Content::JollyBlock(val) => format!("Jolly({})", val),
        Content::Scarecrow => String::from("Scare"),
        Content::None => String::from(""),
    }
}

const GRAY_TRESHOLD:f32 = 0.15;
const COLOR_MAX:f32 = 1.0;

fn invert_color(color: &Color) -> Color {
    if is_close_to_gray(color){
        return Color::BLACK;
    }
    let inverse = Color { r: COLOR_MAX - color.r, g:  COLOR_MAX - color.g, b: COLOR_MAX- color.b, a: 1.0 };
    inverse
}

fn is_close_to_gray(color: &Color) -> bool {
    let average = (color.r + color.g + color.b) / 3.0;

    (color.r - average).abs() <= GRAY_TRESHOLD
        && (color.g - average).abs() <= GRAY_TRESHOLD
        && (color.b - average).abs() <= GRAY_TRESHOLD
}