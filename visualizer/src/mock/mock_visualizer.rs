use robotics_lib::{world::tile::{Tile, TileType, Content}, runner::Runner};
use ggez::{event::{self}, GameError, graphics::{Canvas, TextFragment}, glam};
use ggez::graphics::{self, Color};
use ggez::{Context, GameResult, timer};
use ggez::glam::*;
use std::{sync::mpsc::Receiver, collections::HashMap};
use robotics_lib::event::events::Event as RobotEvent;

// /// Coordinate struct from robotic-lib does not allow for its instances to be created
struct Coord {
    x: usize, 
    y: usize
}

pub enum ChannelItem {
    RobotEventItem(RobotEvent),
    Map(WorldMap)
}

pub struct WorldMap {
    world_map: Vec<Vec<Tile>>,
    robot_position: Coord
}

impl WorldMap {
    pub fn new(world_map: Vec<Vec<Tile>>, (robot_x, roboy_y): (usize, usize)) -> WorldMap {
        WorldMap { world_map: world_map, robot_position: Coord { x: robot_x, y: roboy_y } }
    }
}

pub struct MockVisualizer {
    runner: Runner,
    receiver: Receiver<ChannelItem>,
    total_steps: usize,
    tick_count: usize,
    delay_in_milis: u64,

    world_map: Option<Vec<Vec<Tile>>>,
    robot_position: Option<Coord>,
    backpack: HashMap<Content, usize>
}

impl MockVisualizer {
    pub fn new(runner: Runner, world_receiver: Receiver<ChannelItem>, num_loops: usize, delay_in_milis: u64) -> MockVisualizer {
        MockVisualizer { 
            runner: runner,
            receiver: world_receiver,
            total_steps: num_loops,
            tick_count: 0,
            delay_in_milis: delay_in_milis,
            world_map: None,
            robot_position: None,
            backpack: HashMap::new()
        }
    }
 }

impl event::EventHandler<ggez::GameError> for MockVisualizer {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        println!("UPDATE TICK COUNT: {} (total ticks {})", self.tick_count, self.total_steps);
        if self.tick_count == 0 {
            let res = self.runner.game_tick();
            self.tick_count += 1;
            if let Err(error) = res {
                return Err(GameError::CustomError(format!("{:?}", error)));
            }
        }
        if self.tick_count >= self.total_steps {
            //_ctx.request_quit();
            println!("empty update");
            return Ok(());
        }
        
        //println!("upadte {:?}", self.total_steps);
        //self.total_steps -=1;

        let received_state = self.receiver.try_recv();

        match received_state {
            Ok(channel_item) => {
                timer::sleep(std::time::Duration::from_millis(self.delay_in_milis));
                match channel_item {
                    ChannelItem::RobotEventItem(robot_event) => {
                        match robot_event {
                            // RobotEvent::Ready => todo!(),
                            // RobotEvent::Terminated => todo!(),
                            // RobotEvent::TimeChanged(_) => todo!(),
                            // RobotEvent::DayChanged(_) => todo!(),
                            // RobotEvent::EnergyRecharged(_) => todo!(),
                            // RobotEvent::EnergyConsumed(_) => todo!(),
                            RobotEvent::Moved(_, (robot_x, robot_y)) => {
                                println!("VISUALIZER RECEIVED ROBOT MOVED {:?}", (robot_x, robot_y));
                                self.robot_position = Some(Coord{x:robot_x, y:robot_y });
                                Ok(())
                            }
                            RobotEvent::TileContentUpdated(tile, (tile_x, tile_y)) => {
                                if let Some(world_map) = &mut self.world_map{
                                    world_map[tile_y][tile_x] = tile;
                                }
                                Ok(())
                            }
                            RobotEvent::AddedToBackpack(content, amount) => {
                                *self.backpack.entry(content).or_insert(0) += amount;
                                Ok(())
                            }
                            // RobotEvent::RemovedFromBackpack(_, _) => todo!(),
                            _ => Ok(())
                        }
                    }
                    ChannelItem::Map(world_map) => {
                        println!("VISUALIZER RECEIVED MAP with robot position {:?}", (world_map.robot_position.x, world_map.robot_position.y));
                        self.world_map = Some(world_map.world_map);
                        if self.robot_position.is_none() {
                            self.robot_position = Some(Coord { x: world_map.robot_position.x, y: world_map.robot_position.y });
                        }
                        Ok(())
                    }
                }
            }
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                let res = self.runner.game_tick();
                self.tick_count += 1;
                if let Err(error) = res {
                    return Err(GameError::CustomError(format!("{:?}", error)));
                }
                Ok(())
            }
            Err(error) => Err(GameError::CustomError(format!("{:?}", error)))
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // world map
        if let Some(world_map) = &self.world_map {
            println!("draw");
            let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
            let world_dimension = world_map.len();
            let (x, y) = ctx.gfx.size();
            let size = f32::min(x, y);
            let tile_size = (size / world_dimension as f32) - 10 as f32;

            for y in 0..world_dimension {
                for x in 0..world_dimension {
                    let tile = &world_map[y][x]; 
                    MockVisualizer::draw_tile(tile, ctx, &mut canvas, x, y, tile_size)?;
                    
                }
            }

            // robot
            if let Some(robot_position) = &self.robot_position {
                MockVisualizer::draw_robot(robot_position, ctx, &mut canvas, tile_size)?;
            }
            
            //backpack
            self.draw_backpack(&mut canvas, tile_size, world_dimension);

            let x_tick_count = (tile_size * world_dimension as f32) + (tile_size*3.0);
            let y_tick_count = tile_size;
            let text_size = tile_size * 0.18;
            MockVisualizer::draw_text(&mut  canvas, x_tick_count, y_tick_count, Color::WHITE, text_size, format!("TICK: {}", self.tick_count));

            if self.tick_count >= self.total_steps {
                MockVisualizer::draw_text(&mut  canvas, x_tick_count, y_tick_count + text_size * 1.2, Color::WHITE, text_size, format!("SIMULATION DONE"));
            }

            canvas.finish(ctx)?;
            Ok(())
        }
        else {
            Err(GameError::CustomError(format!("Game state is missing when it should be present")))
        }
    }
}

impl MockVisualizer {
    fn draw_tile(tile: &Tile, ctx: &mut Context, canvas: &mut Canvas, x: usize, y:usize, tile_size: f32) -> GameResult {
        let color = MockVisualizer::get_tile_color(&tile.tile_type);
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(
                x as f32 * tile_size,
                y as f32 * tile_size,
                tile_size,
                tile_size,
            ),
            color
        )?;
        canvas.draw(&rect, graphics::DrawParam::default());

        // TODO: use draw_text fn
        let center_x = (x as f32 + 0.05) * tile_size;
        let center_y = (y as f32 + 0.75) * tile_size;
        let text_size = tile_size * 0.18;
        let dest_point = ggez::glam::Vec2::new(center_x, center_y);
        let text_color = MockVisualizer::invert_color(&color);
        let text_fragment = TextFragment{
            color: Some(text_color),
            text: MockVisualizer::get_content_string(&tile.content),
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

    fn draw_backpack(&self, canvas: &mut Canvas, tile_size: f32, world_dimension:usize) {
        let x_backpack = (tile_size * world_dimension as f32) + (tile_size);
        let mut y_backpack = tile_size;
        let text_size = tile_size * 0.2;
        let text_color = Color::WHITE;

        MockVisualizer::draw_text(canvas, x_backpack, y_backpack - text_size * 1.1, text_color, text_size, "BACKPACK:".to_owned());
        for (content, amount) in &self.backpack {
            MockVisualizer::draw_text(canvas, x_backpack, y_backpack, text_color, text_size, format!("{}({})", content.to_string(), amount));
            y_backpack += text_size * 1.1;
        }
    }

    fn draw_robot(robot_position: &Coord, ctx: &mut Context, canvas: &mut Canvas, tile_size: f32) -> GameResult {
        let x = robot_position.x;
        let y = robot_position.y;
        let center_x = (x as f32 + 0.25) * tile_size;
        let center_y = (y as f32 + 0.25) * tile_size;

        let circle_radius = tile_size * 0.2;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            glam::Vec2::new(center_x, center_y),
            circle_radius,
            0.4, // Segments (adjust based on your needs)
            Color::BLACK,
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());
        Ok(())
    }

    fn draw_text(canvas: &mut Canvas, x: f32, y:f32, color: Color, size:f32, text: String ) {
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

    fn invert_color(color: &Color) -> Color {
        if MockVisualizer::is_close_to_gray(color){
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
}

const GRAY_TRESHOLD:f32 = 0.15;
const COLOR_MAX:f32 = 1.0;
// let color = Color::new(
                    //     j as f32 / world_state.world_dimension as f32,
                    //     i as f32 / world_state.world_dimension as f32,
                    //     0.5,
                    //     1.0,
                    // );