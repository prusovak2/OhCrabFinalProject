use std::sync::mpsc::Sender;

use oxagaudiotool::OxAgAudioTool;
use robotics_lib::event::events::Event as RobotEvent;
use rstykrab_cache::Action;

use crate::oh_crab_visualizer::audio::get_configured_audio_tool;

use super::Coord;

#[derive(Debug)]
pub(crate) enum ChannelItem {
    EventChannelItem(RobotEvent),
    InterfaceChannelItem(InterfaceInvocation)
}

#[derive(Debug)]
pub(crate) struct InterfaceInvocation{
    interface_action: Action,
    robot_position: Coord,
    riz_message: Option<String>
}

impl InterfaceInvocation {
    pub(crate) fn new(interface_action: Action, robot_position: Coord, riz_message: Option<String>) -> InterfaceInvocation {
        InterfaceInvocation {
            interface_action, 
            robot_position,
            riz_message
        }
    }
}

pub struct VisualizerEventListener{
    pub(crate) sender: Sender<ChannelItem>,
    audio_tool: Option<OxAgAudioTool>,
}

impl VisualizerEventListener {
    pub(crate) fn new(sender: Sender<ChannelItem>, use_sound: bool) -> VisualizerEventListener {
        VisualizerEventListener{
            sender,
            audio_tool: if use_sound { Some(VisualizerEventListener::get_configured_audio_tool())} else {None},
        }
    }

    pub fn handle_event(&mut self, event: &RobotEvent) {
        self.send_event(event.clone());
        self.play_audio_based_on_event(event);
    }

    fn send_event(&self, event: RobotEvent) {
        println!("DATA SENDER sending event: {:?}", event);
        let channel_item = ChannelItem::EventChannelItem(event.clone());
        self.sender.send(channel_item).expect(&format!("VisualizerDataSender: sending event {} failed.", event));
    }

    fn play_audio_based_on_event(&mut self, event: &RobotEvent) {
        if let Some(audio_tool) = &mut self.audio_tool {
            let audio_res = audio_tool.play_audio_based_on_event(&event);
            match audio_res {
                Ok(_) => {},
                Err(err) => println!("Audio tool error: {}", err),
            }
        }
    }

    fn get_configured_audio_tool() -> OxAgAudioTool {
        let audio_res = get_configured_audio_tool();
        match audio_res {
            Ok(tool) => tool,
            Err(err) => panic!("Audio tool error: {}", err),
        }
    }
}