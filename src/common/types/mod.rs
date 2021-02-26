use crossbeam_channel::Sender;
#[warn(unused_imports)]
use twitch_irc::message::ServerMessage;

// Describe the current available services
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Services {
    Irc,
    Command,
    Broadcaster,
}

#[derive(Clone, Debug)]
pub struct BroadcastMessage {
    pub timestamp: u64,
    pub sender: Services,
    pub raw_message: MessageContent,
    pub to: Option<Services>,
}

// Describes the broadcast message content options
#[derive(Clone, Debug)]
pub enum MessageContent {
    String(String),
    ServerMessage(ServerMessage),
    AddService(ServiceSender),
}

#[derive(Clone, Debug)]
pub struct ServiceSender {
    pub service: Services,
    pub sender: Sender<BroadcastMessage>
}

pub struct Irc;
pub struct Command;
pub struct Broadcaster;

