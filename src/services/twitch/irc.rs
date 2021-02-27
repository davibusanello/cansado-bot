use std::fs::OpenOptions;
use std::io::Write;

use crossbeam_channel::{unbounded, Sender};
use futures::prelude::*;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage;
use twitch_irc::ClientConfig;
use twitch_irc::TCPTransport;
use twitch_irc::TwitchIRCClient;

use crate::common::helpers::current_timestamp;
use crate::common::types::{BroadcastMessage, MessageContent, ServiceSender, Services};

#[tokio::main]
pub async fn init(
    channel: String,
    username: Option<String>,
    token: Option<String>,
    broadcast_sender: Sender<BroadcastMessage>,
) {
    let file_name = format!("{}_{}_irc.log", current_timestamp(), channel);

    let mut log_file = OpenOptions::new()
        .create_new(true)
        .append(true)
        .open(file_name)
        .expect("Can't open logs file to write.");

    let (irc_sender, irc_receiver) = unbounded::<BroadcastMessage>();
    let send_sender = broadcast_sender.clone();

    send_sender.send(add_service(irc_sender.clone())).unwrap();

    let config = get_auth_credentials(username, token);
    let (mut incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let irc_thread_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.next().await {
            let copy_message = message.clone();
            match message {
                ServerMessage::Privmsg(private_message) => {
                    let log = format!(
                        "{} {} {} \n",
                        private_message.server_timestamp,
                        private_message.sender.login,
                        private_message.message_text
                    );
                    log_file.write_all(log.as_bytes()).expect("Write failed.");
                }
                _ => (),
            }

            let message = build_broadcast_message(copy_message, None).clone();

            // send messages to broadcaster
            broadcast_sender.send(message).unwrap();
        }
    });

    let handler_client = client.clone();
    let handler_channel = channel.clone();
    let irc_sender_thread = tokio::spawn(async move {
        loop {
            match irc_receiver.recv() {
                Ok(data) => {
                    println!("IRC sending.. {:?} \n", data.clone());
                    match data.raw_message {
                        MessageContent::String(message) => handler_client
                            .say(handler_channel.clone(), message)
                            .await
                            .unwrap(),
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    });

    // join a channel
    client.join(channel.to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    irc_thread_handle.await.unwrap();
    irc_sender_thread.await.unwrap();
}

fn build_broadcast_message(message: ServerMessage, to: Option<Services>) -> BroadcastMessage {
    BroadcastMessage {
        timestamp: current_timestamp(),
        sender: Services::Irc,
        raw_message: MessageContent::ServerMessage(message),
        to: to,
    }
}

fn add_service(sender: Sender<BroadcastMessage>) -> BroadcastMessage {
    BroadcastMessage {
        timestamp: current_timestamp(),
        sender: Services::Irc,
        raw_message: MessageContent::AddService(ServiceSender {
            service: Services::Irc,
            sender: sender,
        }),
        to: Some(Services::Broadcaster),
    }
}

fn get_auth_credentials(
    username: Option<String>,
    token: Option<String>,
) -> ClientConfig<StaticLoginCredentials> {
    if username.is_none() || token.is_none() {
        // default configuration is to join chat as anonymous.
        return ClientConfig::default();
    }

    let custom_credentials = StaticLoginCredentials::new(
        username.unwrap().to_string(),
        Some(token.unwrap().to_string()),
    );
    ClientConfig::new_simple(custom_credentials)
}
