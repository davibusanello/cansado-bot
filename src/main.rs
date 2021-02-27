use dotenv;
use std::env;
use std::thread;
use std::time;
// Internals
mod services;
use services::twitch;
use services::commands;
mod common;
mod broadcast;
mod state;

// Represents the app configuration
#[derive(Debug)]
struct AppConfig {
    channel: String,
    twitch_username: Option<String>,
    twitch_token: Option<String>,
}

fn main() {
    let environment = current_environment();
    let config = load_config(&environment);
    let used_channel = config.channel.clone();
    let username = config.twitch_username;
    let oauth_token = config.twitch_token;

    let mut state = state::init_state().unwrap();
    let mut thread_list: Vec<thread::JoinHandle<()>> = Vec::new();
    println!(
        "Starting {:?} in '{}' environment!",
        username.to_owned(),
        environment
    );
    println!("-----------------------");

    let (broadcast_th, broadcaster_sender) = broadcast::init_broadcaster();

    thread_list.push(broadcast_th);
    // 1.5 seconds delay
    let ten_millis = time::Duration::from_millis(1500);
    let _now = time::Instant::now();
    thread::sleep(ten_millis);

    // The sender endpoint can be copied
    let irc_broadcast_sender = broadcaster_sender.clone();

    // spawn the irc module on other thread
    let irc_thread = thread::spawn(move || {
        twitch::irc::init(used_channel, username, oauth_token, irc_broadcast_sender);
    });

    let commands_broadcast_sender = broadcaster_sender.clone();
    let commands_thread = commands::init_commands(commands_broadcast_sender, state);

    println!("--------------------");

    irc_thread.join().unwrap();
    commands_thread.join().unwrap();
}

fn current_environment() -> String {
    match env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(_e) => String::from("development"),
    }
}

fn load_config(environment: &String) -> AppConfig {
    dotenv::from_filename(environment.to_owned() + ".env").ok();

    let channel = match env::var("TWITCH_IRC_CHANNEL") {
        Ok(val) => val,
        Err(_e) => String::from("test_channel"),
    };
    let username = match env::var("TWITCH_IRC_USERNAME") {
        Ok(val) => Some(val),
        Err(_e) => None,
    };
    let token = match env::var("TWITCH_IRC_OAUTH_TOKEN") {
        Ok(val) => Some(val),
        Err(_e) => None,
    };

    AppConfig {
        channel: channel,
        twitch_username: username,
        twitch_token: token,
    }
}
