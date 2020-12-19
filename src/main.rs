use dotenv;
use std::env;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;

mod twitch;
mod types;
use types::MessageReceived;

struct AppConfig {
    channel: String,
    twitch_username: Option<String>,
    twitch_token: Option<String>,
}

fn main() {
    let environment = current_environment();
    let config = load_config(&environment);
    let username = Some(config.twitch_username.as_ref().map_or("", String::as_str));
    let oauth_token = Some(config.twitch_token.as_ref().map_or("", String::as_str));
    println!("Starting {:?} in '{}' environment!", username.to_owned(), environment);
    println!("-----------------------");

    let mut received_messages_queue = init_queues();

    twitch::irc::init(&config.channel, username, oauth_token);
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
        twitch_token: token
    }
}

fn init_queues() -> Rc<RefCell<VecDeque<MessageReceived>>> {

    let mut buffer: VecDeque<MessageReceived> = VecDeque::new();
    let received_messages = Rc::new(RefCell::new(buffer));

    return received_messages;
}
