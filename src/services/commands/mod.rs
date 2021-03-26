use crossbeam_channel::{unbounded, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use twitch_irc::message::{PrivmsgMessage, ServerMessage};

use crate::common::helpers::current_timestamp;
use crate::common::types::{BotState, BroadcastMessage, MessageContent, ServiceSender, Services};

pub fn init_commands(
    broadcast_sender: Sender<BroadcastMessage>,
    state: Arc<Mutex<BotState>>,
) -> thread::JoinHandle<()> {
    let (commands_sender, commands_receiver) = unbounded::<BroadcastMessage>();
    let send_sender = broadcast_sender.clone();
    send_sender
        .send(add_service(commands_sender.clone()))
        .unwrap();

    let broadcaster_receiver_thread = thread::spawn(move || loop {
        match commands_receiver.recv() {
            Ok(data) => {
                let raw_message = data.raw_message.clone();
                match raw_message {
                    MessageContent::ServerMessage(server_message) => match server_message {
                        ServerMessage::Privmsg(prv_message) => {
                            let first_char = prv_message.message_text.chars().nth(0).unwrap();
                            if first_char == '!' {
                                let mut string_parts = prv_message.message_text.split_whitespace();
                                let command = string_parts.next();
                                println!("Command: {:?}", command.clone());
                                match command {
                                    Some("!first") => {
                                        first(prv_message, broadcast_sender.clone(), state.clone())
                                    }
                                    Some("!frist") | Some("!fritz")=> {
                                        frist(prv_message, broadcast_sender.clone(), state.clone())
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
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
            sender: sender,
        }),
        to: Some(Services::Broadcaster),
    }
}

fn frist(
    raw_irc_message: PrivmsgMessage,
    broadcast_sender: Sender<BroadcastMessage>,
    state: Arc<Mutex<BotState>>,
) {
    thread::spawn(move || {
        let command_sender = raw_irc_message.sender.login;
        let mutable_state = state.lock().unwrap();
        let login = mutable_state
            .commands
            .first_list
            .iter()
            .find(|login| login == &&command_sender);

        let compliment = "Cumprimentos aos Sr. @freakyfog e Sr. @pokemaobr.";

        let message: String;
        if login.is_some() {
            message = format!("Eita, mas acho que tu ja a @{} fez certo", command_sender);
        } else {
            message = format!(
                "Foi por pouco @{}, acho que voce queria dizer 'first' BloodTrail",
                command_sender
            );
        }
        broadcast_sender
            .send(BroadcastMessage {
                timestamp: current_timestamp(),
                sender: Services::Command,
                raw_message: MessageContent::String(
                    format!("{} {}", message, compliment)
                ),
                to: Some(Services::Irc),
            })
            .unwrap();
    });
}

fn first(
    raw_irc_message: PrivmsgMessage,
    broadcast_sender: Sender<BroadcastMessage>,
    state: Arc<Mutex<BotState>>,
) {
    thread::spawn(move || {
        let command_sender = raw_irc_message.sender.login;
        let mut mutable_state = state.lock().unwrap();
        let login = mutable_state
            .commands
            .first_list
            .iter()
            .find(|login| login == &&command_sender);

        if login.is_some() {
            return;
        }

        let mut message = format!(
            "Foi por pouco @{}, mas você foi o nº {} a chegar PogChamp",
            command_sender,
            mutable_state.commands.first_list.len() + 1
        );
        if mutable_state.commands.first_list.is_empty() {
            message = format!("Valeu por chegar aqui cedo @{}", command_sender);
        }
        broadcast_sender
            .send(BroadcastMessage {
                timestamp: current_timestamp(),
                sender: Services::Command,
                raw_message: MessageContent::String(message),
                to: Some(Services::Irc),
            })
            .unwrap();

        mutable_state.add_visitor(command_sender);
    });
}
