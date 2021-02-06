use std::time::SystemTime;

pub fn current_timestamp() -> u64 {
    return match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs() * 1000,
        _ => {
            println!("Error getting the timestamp");
            0
        }
    };
}
