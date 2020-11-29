use std::env;

mod twitch;


fn main() {
    let environment = current_environment();

    let config = load_config();
    let username = Some(config.1.as_ref().map_or("", String::as_str));
    let oauth_token = Some(config.2.as_ref().map_or("", String::as_str));
    println!("Starting {:?} in '{}' environment!", username.to_owned(), environment);
    println!("-----------------------");
    twitch::irc::init(&config.0, username, oauth_token);

    // println!("My config: {:?}", config)

}

fn current_environment() -> String {
    match env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(_e) => String::from("development"),
    }
}

fn load_config() -> (String, Option<String>, Option<String>) {
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

    (channel, username, token)
}
