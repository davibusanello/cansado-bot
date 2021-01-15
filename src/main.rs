use dotenv;
use std::env;
use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;
use std::time;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

mod twitch;
mod types;
use types::MessageReceived;

// Represents the app configuration
#[derive(Debug)]
struct AppConfig {
    channel: String,
    twitch_username: Option<String>,
    twitch_token: Option<String>,
}

fn main() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    let environment = current_environment();
    let config = load_config(&environment);
    let used_channel = config.channel.clone();
    let username = config.twitch_username;
    let oauth_token = config.twitch_token;
    println!("Starting {:?} in '{}' environment!", username.to_owned(), environment);
    println!("-----------------------");

    // The sender endpoint can be copied
    let thread_tx = tx.clone();

    // spawn the irc module on other thread
    let irc_thread = thread::spawn(move || {
        twitch::irc::init(used_channel, username, oauth_token);
    });

    let ten_millis = time::Duration::from_millis(10000);
    let now = time::Instant::now();
    thread::sleep(ten_millis);


    let mut received_messages_queue = init_queues();

    println!("--------------------");
    println!("The IRC bot is running in another thread...");

    irc_thread.join();

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
