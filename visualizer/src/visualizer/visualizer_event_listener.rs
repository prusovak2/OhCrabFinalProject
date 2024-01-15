use std::sync::mpsc::Sender;

use oxagaudiotool::OxAgAudioTool;
use robotics_lib::event::events::Event as RobotEvent;

use crate::audio::get_configured_audio_tool;

use super::visualizable_interfaces::InterfaceChannelItem;


// trait RunnableVisualizable<'a>: Runnable + Visalizable<'a> {
// }

pub trait Visalizable<'a> {
    fn borrow_interface_sender(&'a self) -> &'a VisualizerEventListener;
}

#[derive(Debug)]
pub(crate) struct EventChannelItem {
    pub(crate) event:RobotEvent,
}

pub struct VisualizerEventListener{
    pub(crate) interface_sender: Sender<InterfaceChannelItem>,
    event_sender: Sender<EventChannelItem>,
    audio_tool: Option<OxAgAudioTool>,
}

impl VisualizerEventListener {
    pub(crate) fn new(interface_sender: Sender<InterfaceChannelItem>, event_sender: Sender<EventChannelItem>, use_sound: bool) -> VisualizerEventListener {
        VisualizerEventListener{
            interface_sender, 
            event_sender, 
            audio_tool: if use_sound { Some(VisualizerEventListener::get_configured_audio_tool())} else {None},
        }
    }

    pub fn handle_event(&mut self, event: &RobotEvent) {
        self.send_event(event.clone());
        self.play_audio_based_on_event(event);
    }

    fn send_event(&self, event: RobotEvent) {
        println!("DATA SENDER sending event: {:?}", event);
        let channel_item = EventChannelItem{event:event.clone()};
        self.event_sender.send(channel_item).expect(&format!("VisualizerDataSender: sending event {} failed.", event));
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