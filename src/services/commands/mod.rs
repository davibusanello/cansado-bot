use std::thread;
use crossbeam_channel::{unbounded, Sender};
use twitch_irc::message::{ServerMessage};

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
                                    let command = prv_message.message_text.split(' ');
                                    println!("Command: {:?}", command);
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

fn first() {}
