use std::thread;
use crossbeam_channel::{unbounded, Sender};
use twitch_irc::message::{ServerMessage, PrivmsgMessage};

use crate::common::helpers::current_timestamp;
use crate::common::types::{BroadcastMessage, Services, MessageContent, ServiceSender};

pub fn init_commands(broadcast_sender: Sender<BroadcastMessage>) -> thread::JoinHandle<()> {
    let (commands_sender, commands_receiver) = unbounded::<BroadcastMessage>();
    let send_sender = broadcast_sender.clone();
    send_sender.send(add_service(commands_sender.clone())).unwrap();

    let broadcaster_receiver_thread = thread::spawn(move || loop {
        match commands_receiver.recv() {
            Ok(data) => {
                let raw_message = data.raw_message.clone();
                match raw_message {
                    MessageContent::ServerMessage(server_message) => {
                        match server_message {
                            ServerMessage::Privmsg(prv_message) => {
                                let first_char = prv_message.message_text.chars().nth(0).unwrap();
                                if first_char == '!' {
                                    println!("Is a command");
                                    let mut string_parts = prv_message.message_text.split_whitespace();
                                    let command = string_parts.next();
                                    println!("Command: {:?}", command.clone());
                                    if command == Some("!first") { first(prv_message, broadcast_sender.clone()) }
                                }
                            },
                            _ => (),
                        }
                    },
                    _ => (),
                }
            },
            _ => (),
        }
    });

    broadcaster_receiver_thread
}

fn add_service(sender: Sender<BroadcastMessage>) -> BroadcastMessage {
    BroadcastMessage {
        timestamp: current_timestamp(),
        sender: Services::Command,
        raw_message: MessageContent::AddService(ServiceSender {
            service: Services::Command,
            sender: sender
        }),
        to: Some(Services::Broadcaster)
    }
}

fn first(raw_irc_message: PrivmsgMessage, broadcast_sender: Sender<BroadcastMessage>) {
    thread::spawn(move || {
        let message = format!("Valeu por chegar aqui cedo @{}", raw_irc_message.sender.login);
        broadcast_sender.send(BroadcastMessage {
            timestamp: current_timestamp(),
            sender: Services::Command,
            raw_message: MessageContent::String(message),
            to: Some(Services::Irc)
        }).unwrap()
    });
}
