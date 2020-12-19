use futures::prelude::*;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TCPTransport;
use twitch_irc::TwitchIRCClient;

use twitch_irc::message::{ServerMessage};
use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
pub async fn init(channel: &str, username: Option<&str>, token: Option<&str>) {
    let mut log_file = OpenOptions::new().append(true).open("irc_logs.log").expect("Can't open logs file to write.");

    let config = get_auth_credentials(username, token);
    let (mut incoming_messages, client) =
        TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    // first thing you should do: start consuming incoming messages,
    // otherwise they will back up.
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.next().await {
            println!("Received message: {:?}", message);

            match message {
                ServerMessage::Privmsg(private_message) => {
                    let log = format!("{} {} {} \n", private_message.server_timestamp, private_message.sender.login, private_message.message_text);
                    log_file.write_all(log.as_bytes()).expect("Write failed.");
                },
                _ => (),
            }
        }
    });

    // join a channel
    client.join(channel.to_owned());

    // keep the tokio executor alive.
    // If you return instead of waiting the background task will exit.
    join_handle.await.unwrap();
}

fn get_auth_credentials(username: Option<&str>, token: Option<&str>) -> ClientConfig<StaticLoginCredentials> {
    if username.is_none() || token.is_none() {
        // default configuration is to join chat as anonymous.
        return ClientConfig::default();
    }

    let custom_credentials = StaticLoginCredentials::new(username.unwrap().to_string(), Some(token.unwrap().to_string()));
    ClientConfig::new_simple(custom_credentials)
}
