pub mod visualizable_robot;
pub mod visualizable_interfaces;
pub mod visualizer;
mod draw_utils;
mod egui_utils;
pub mod visualizer_event_listener;
mod visualizer_debug;

// Coordinate struct from robotic-lib does not allow for its instances to be created
#[derive(Debug)]
pub(crate) struct Coord {
    pub(crate) x: usize, 
    pub(crate) y: usize
}

impl Coord {
    pub(crate) fn new(x: usize, y: usize) -> Coord {
        Coord { x: x, y: y} 
    }
}