use crossbeam_channel::Sender;
#[warn(unused_imports)]
use twitch_irc::message::ServerMessage;

// Describe the current available services
#[derive(Copy, Clone, Debug)]
pub enum Services {
    Irc,
    Command,
    Broadcaster,
}

#[derive(Clone, Debug)]
pub struct BroadcastMessage {
    timestamp: u64,
    sender: Services,
    raw_message: MessageContent,
    to: Option<Services>,
}

// Describes the broadcast message content options
#[derive(Copy, Clone, Debug)]
pub enum MessageContent {
    String,
    ServerMessage,
    AddService,
}

/// Describes the Message to load a new service in the broadcaster
pub struct AddService<T> {
    sender: Sender<T>,
}
