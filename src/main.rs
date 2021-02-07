use dotenv;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::env;
use std::rc::Rc;
use std::thread;
use std::time;

use crossbeam_channel::unbounded;
// use crossbeam_channel::{Sender, Receiver};
use crossbeam_queue::SegQueue;
use twitch_irc::message::ServerMessage;

mod services;
use services::twitch;
mod common;
use common::types::BroadcastMessage;

// Represents the app configuration
#[derive(Debug)]
struct AppConfig {
    channel: String,
    twitch_username: Option<String>,
    twitch_token: Option<String>,
}

fn main() {
    let history_queue = SegQueue::<ServerMessage>::new();

    let (tx, rx) = unbounded::<ServerMessage>();

    let environment = current_environment();
    let config = load_config(&environment);
    let used_channel = config.channel.clone();
    let username = config.twitch_username;
    let oauth_token = config.twitch_token;
    println!(
        "Starting {:?} in '{}' environment!",
        username.to_owned(),
        environment
    );
    println!("-----------------------");

    let pool_receiver = rx.clone();
    // spawn the pool reader thread
    let pool_thread = thread::spawn(move || loop {
        match pool_receiver.recv() {
            Ok(data) => {
                history_queue.push(data.clone());
                println!("Pool: Received: {:?}", data);
            }
            Err(err) => println!("Pool 1 Error on {:?}", err),
        }
    });

    let pool_receiver2 = rx.clone();
    let pool_thread2 = thread::spawn(move || loop {
        println!("Pool2: Received: {:?}", pool_receiver2.recv().unwrap());
    });

    // The sender endpoint can be copied
    let irc_thread_tx = tx.clone();

    // spawn the irc module on other thread
    let irc_thread = thread::spawn(move || {
        twitch::irc::init(used_channel, username, oauth_token, irc_thread_tx);
    });

    let ten_millis = time::Duration::from_millis(10000);
    let _now = time::Instant::now();
    thread::sleep(ten_millis);

    // let mut received_messages_queue = init_queues();

    println!("--------------------");
    println!("The IRC bot is running in another thread...");

    irc_thread.join();
    pool_thread.join();
    pool_thread2.join();
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

// fn init_queues() -> Rc<RefCell<VecDeque<BroadcastMessage>>> {

//     let mut buffer: VecDeque<BroadcastMessage> = VecDeque::new();
//     let received_messages = Rc::new(RefCell::new(buffer));

//     return received_messages;
// }
