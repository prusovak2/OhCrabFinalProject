use std::collections::HashMap;

use ggez::{graphics::{Canvas, Color, self, TextFragment, Image}, Context, glam, mint::{Point2, Vector2}};
use robotics_lib::world::tile::{Tile, TileType, Content};

use crate::println_d;

use super::{Coord, visualizer::{OhCrabVisualizerError, self, CONTENT_TILE_SIZE_LIMIT, VisualizationState}};

#[derive(Default)]
pub(super) struct GridCanvasProperties {
    pub(super) tile_size: f32,
    pub(super) grid_canvas_width: f32, 
    pub(super) grid_canvas_height: f32,
    pub(super) grid_canvas_origin_x: f32,
    pub(super) grid_canvas_origin_y: f32,
    pub(super) world_dimension: usize
}

impl GridCanvasProperties {
    pub(super) fn num_rows_to_display(&self) -> usize {
        (self.grid_canvas_height / self.tile_size).floor() as usize
    }

    pub(super) fn num_columns_to_display(&self) -> usize {
        (self.grid_canvas_width / self.tile_size).floor() as usize
    }

    pub(super) fn build(canvas_total_size: f32, world_dimension: usize) -> GridCanvasProperties {
        GridCanvasProperties {
            tile_size: visualizer::DEFAULT_TILE_SIZE,
            grid_canvas_height: canvas_total_size - 80.0,
            grid_canvas_width: canvas_total_size - 80.0,
            grid_canvas_origin_x: visualizer::GRID_CANVAS_ORIGIN_X,
            grid_canvas_origin_y: visualizer::GRID_CANVAS_ORIGIN_Y,
            world_dimension
        }
    } 
}

pub(super) struct GgezImages {
    robot_image: Option<Image>,
    tile_images: HashMap<TileType, Image>
}

impl GgezImages {
    pub(super) fn init(ctx: & Context) -> GgezImages {
        let robot_image = Image::from_path(&ctx.gfx, "/images/robot/robot_2.png").ok();
        let mut tile_images: HashMap<TileType, Image> = HashMap::new();
        tile_images.insert(TileType::DeepWater, Image::from_path(&ctx.gfx, "/images/tiles/deep_water.png").expect("failed to load tile image"));
        tile_images.insert(TileType::ShallowWater, Image::from_path(&ctx.gfx, "/images/tiles/shallow_water.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Sand, Image::from_path(&ctx.gfx, "/images/tiles/sand.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Grass, Image::from_path(&ctx.gfx, "/images/tiles/grass.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Street, Image::from_path(&ctx.gfx, "/images/tiles/street.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Hill, Image::from_path(&ctx.gfx, "/images/tiles/hill.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Mountain, Image::from_path(&ctx.gfx, "/images/tiles/mountain.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Snow, Image::from_path(&ctx.gfx, "/images/tiles/snow.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Lava, Image::from_path(&ctx.gfx, "/images/tiles/lava.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Teleport(true), Image::from_path(&ctx.gfx, "/images/tiles/teleport.png").expect("failed to load tile image"));
        tile_images.insert(TileType::Wall, Image::from_path(&ctx.gfx, "/images/tiles/wall.png").expect("failed to load tile image"));

        GgezImages { robot_image: robot_image, tile_images: tile_images }
    }

    pub(super) fn empty() -> GgezImages {
        GgezImages { robot_image: None, tile_images: HashMap::new() }
    }
}

pub(super) fn draw_grid(
        ctx: &mut ggez::Context,
        canvas: &mut Canvas,
        visualization_state: &VisualizationState,
        world_map: &Vec<Vec<Tile>>,
        robot_position: &Option<Coord>,
        images: &GgezImages
    ) 
    -> Result<(), OhCrabVisualizerError> {

    let last_column = visualization_state.get_last_column_to_display();
    let last_row = visualization_state.get_last_row_to_display();
    let tile_offset_x = visualization_state.first_column_to_display();
    let tile_offset_y = visualization_state.first_row_to_display();
    let tile_size = visualization_state.grid_canvas_properties.tile_size;
    let canvas_origin_x = visualization_state.grid_canvas_properties.grid_canvas_origin_x;
    let canvas_origin_y = visualization_state.grid_canvas_properties.grid_canvas_origin_y;

    // let rows_to_display = canvas_props.num_rows_to_display();
    // let columns_to_display = canvas_props.num_columns_to_display();

    // let last_column = usize::min(world_dimension, tile_offset.x + (columns_to_display as usize));
    // let last_row = usize::min(world_dimension, tile_offset.y + (rows_to_display as usize));

    draw_grid_frame(ctx, canvas, &visualization_state.grid_canvas_properties)?;

    for y in tile_offset_y..last_row {
        for x in tile_offset_x..last_column {
            let tile: &Tile = &world_map[y][x]; 
            draw_tile(tile, ctx, canvas, (x-tile_offset_x) as f32, (y-tile_offset_y) as f32,  tile_size, canvas_origin_x, canvas_origin_y, images)?;
        }
    }

    // robot
    if let Some(robot_position) = robot_position {
        if visualization_state.robot_should_be_displaied(robot_position) {
            let robot_position_on_canvas = Coord {x: robot_position.x - tile_offset_x, y: robot_position.y - tile_offset_y };
            draw_robot(&robot_position_on_canvas, ctx, canvas, tile_size, canvas_origin_x, canvas_origin_y, images)?;
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

fn draw_tile(tile: &Tile, ctx: &mut Context, canvas: &mut Canvas, x: f32, y :f32, tile_size: f32, grid_canvas_origin_x: f32, grid_canvas_origin_y: f32, images: &GgezImages) -> Result<(), OhCrabVisualizerError> {    
    let tile_x = (x * tile_size) + grid_canvas_origin_x;
    let tile_y = (y * tile_size) + grid_canvas_origin_y;
    
    let tile_image = images.tile_images.get(&tile.tile_type);
    if  /*tile_size >= CONTENT_TILE_SIZE_LIMIT + 10 as f32 &&*/ tile_image.is_some() {
        let tile_image = tile_image.unwrap();
        let x_scale = 1.0 / (tile_image.width() as f32 / tile_size);
        let y_scale = 1.0 / (tile_image.height() as f32 / tile_size);
        let draw_param = graphics::DrawParam::new()
            .dest(Point2 { x: tile_x, y:tile_y})
            .scale(Vector2 {x: x_scale, y: y_scale});

        canvas.draw(tile_image, draw_param);
        if tile_size >= CONTENT_TILE_SIZE_LIMIT {
            draw_tile_content(canvas, &tile.content, x, y, tile_size, grid_canvas_origin_x, grid_canvas_origin_y, Color::BLACK);
        }
        Ok(())
    }
    else {
        let color = get_tile_color(&tile.tile_type);
        let res = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                tile_x,
                tile_y,
                tile_size,
                tile_size,
            ),
            color
        );
        match res {
            Ok(rect) => {
                canvas.draw(&rect, graphics::DrawParam::default());
                if tile_size >= CONTENT_TILE_SIZE_LIMIT {
                    draw_tile_content(canvas, &tile.content, x, y, tile_size, grid_canvas_origin_x, grid_canvas_origin_y, color);
                }
                Ok(())
            }
            Err(error) => Err(OhCrabVisualizerError::GraphicsLibraryError(error))
        }
    }
}

pub fn draw_tile_content(canvas: &mut Canvas, content: &Content, x: f32, y:f32, tile_size: f32, grid_canvas_origin_x: f32, grid_canvas_origin_y: f32, color: Color) {
    let text_x = ((x + 0.05) * tile_size) + grid_canvas_origin_x;
    let text_y: f32 = ((y + 0.75) * tile_size) + grid_canvas_origin_y;
    let text_size = tile_size * 0.18;
    let text_color = invert_color(&color);

    draw_content(canvas, content, text_x, text_y, text_size, text_color);
}

pub(crate) fn draw_content(canvas: &mut Canvas, content: &Content, x: f32, y:f32, size: f32, color: Color) {
    let text = get_content_string(content);
    draw_text(canvas, x, y, color, size, text)
}

// pub(super) fn draw_backpack(backpack: &HashMap<Content, usize>, canvas: &mut Canvas, tile_size: f32, world_dimension:usize) {
//     let x_backpack = (tile_size * world_dimension as f32) + (tile_size) + 200.0;
//     let mut y_backpack = tile_size;
//     let text_size = tile_size * 0.2;
//     let text_color = Color::WHITE;

//     draw_text(canvas, x_backpack, y_backpack - text_size * 1.1, text_color, text_size, "BACKPACK:".to_owned());
//     for (content, amount) in backpack {
//         draw_text(canvas, x_backpack, y_backpack, text_color, text_size, format!("{}({})", content.to_string(), amount));
//         y_backpack += text_size * 1.1;
//     }
// }

fn draw_robot(robot_position: &Coord, ctx: &mut Context, canvas: &mut Canvas, tile_size: f32, grid_canvas_origin_x: f32, grid_canvas_origin_y: f32, images: &GgezImages) -> Result<(), OhCrabVisualizerError> {
    let x = robot_position.x;
    let y = robot_position.y;
    if  tile_size >= CONTENT_TILE_SIZE_LIMIT + 10 as f32 && images.robot_image.is_some() {
        let center_x = ((x as f32 + 0.1) * tile_size) + grid_canvas_origin_x;
        let center_y = ((y as f32 + 0.1) * tile_size) + grid_canvas_origin_y;
        let robot_image: &Image = images.robot_image.as_ref().unwrap();
        let x_scale = (1.0 / (robot_image.width() as f32 / tile_size)) * 0.6;
        let y_scale = (1.0 / (robot_image.height() as f32 / tile_size)) * 0.6;

        let draw_param = graphics::DrawParam::new()
            .dest(Point2 { x: center_x, y:center_y})
            .scale(Vector2 {x:x_scale, y:y_scale});

        canvas.draw(robot_image, draw_param);
        Ok(())
    }
    else{
        let circle_radius = tile_size * 0.2;
        let center_x = ((x as f32 + 0.25) * tile_size) + grid_canvas_origin_x;
        let center_y = (y as f32 + 0.25) * tile_size + grid_canvas_origin_y;
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

pub(super) fn get_content_string(content: &Content) -> String {
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