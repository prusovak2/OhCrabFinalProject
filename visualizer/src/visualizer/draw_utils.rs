use std::collections::HashMap;

use ggez::{graphics::{Canvas, Color, self, TextFragment}, Context, glam, GameResult};
use oxagaudiotool::error::error;
use robotics_lib::world::tile::{Tile, TileType, Content};

use super::{Coord, visualizer::OhCrabVisualizerError};

pub(super) fn draw_tile(tile: &Tile, ctx: &mut Context, canvas: &mut Canvas, x: usize, y:usize, tile_size: f32) -> Result<(), OhCrabVisualizerError> {
    let color = get_tile_color(&tile.tile_type);
    let res = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            x as f32 * tile_size,
            y as f32 * tile_size,
            tile_size,
            tile_size,
        ),
        color
    );

    match res {
        Ok(rect) => {
            canvas.draw(&rect, graphics::DrawParam::default());

            // TODO: use draw_text fn
            let center_x = (x as f32 + 0.05) * tile_size;
            let center_y = (y as f32 + 0.75) * tile_size;
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