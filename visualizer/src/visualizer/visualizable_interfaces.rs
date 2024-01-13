use std::sync::mpsc::Sender;

pub(crate) enum InterfaceChannelItem{
    Action,
    SomethingElse
}

pub struct VisualizableInterfaces {
    sender: Sender<InterfaceChannelItem>
}

impl VisualizableInterfaces {
    pub(crate) fn new(sender: Sender<InterfaceChannelItem>) -> VisualizableInterfaces {
        VisualizableInterfaces {sender}
    }
}
