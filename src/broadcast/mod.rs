use std::thread;

use crossbeam_queue::SegQueue;
use crossbeam_channel::unbounded;


use types::{BroadcastMessage, Services};

let broadcaster_queue = SegQueue::<BroadcastMessage>::new();

let (broadcast_sender, broadcast_receiver) = unbounded::<BroadcastMessage>();

let mut pool_thread: thread::JoinHandle;

pub init_broadcaster() {

}
