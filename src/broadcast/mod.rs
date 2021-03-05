use crate::common::types::{BroadcastMessage, MessageContent, ServiceSender, Services};
use crossbeam_channel::{unbounded, Sender};
use crossbeam_queue::SegQueue;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn init_broadcaster() -> (thread::JoinHandle<()>, Sender<BroadcastMessage>) {
    // Using Arc/Mutex to be able to share between threads
    let broadcaster_queue = Arc::new(Mutex::new(SegQueue::<BroadcastMessage>::new()));
    let services_list: Arc<Mutex<Vec<ServiceSender>>> = Arc::new(Mutex::new(Vec::new()));

    let receiver_service_list = services_list.clone();
    let receiver_queue = broadcaster_queue.clone();
    let handler_queue = broadcaster_queue.clone();
    let (broadcast_sender, broadcast_receiver) = unbounded::<BroadcastMessage>();

    let broadcaster_receiver_thread = thread::spawn(move || loop {
        let queue_data = receiver_queue.lock().unwrap();
        match broadcast_receiver.recv() {
            Ok(data) => {
                let raw_message = data.raw_message.clone();
                match raw_message {
                    MessageContent::AddService(sender_data) => {
                        let mut services = receiver_service_list.lock().unwrap();
                        println!("service {:?} added", sender_data.service);
                        services.push(sender_data.clone());
                    },
                    _ => {
                        queue_data.push(data.clone());
                        let services = receiver_service_list.lock().unwrap();

                        for service in services.iter() {
                            if !(service.service == data.sender) {
                                println!("sending message to {:?}", service.service);
                                service.sender.send(data.clone()).unwrap();
                            }
                        }
                    }
                }
                // queue_sender.send(data.clone());
                println!("Broadcaster received: {:?} \n", data);
            }
            Err(err) => println!("Broadcaster received error: {:?}", err),
        }
    });

    return (broadcaster_receiver_thread, broadcast_sender.clone());
}
