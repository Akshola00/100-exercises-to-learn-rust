use std::sync::mpsc::{Receiver, Sender};

use crate::data::TicketDraft;

pub mod data;
pub mod store;

#[derive(Debug)]
pub enum Command {
    Insert(TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    // while 1<2 {
    //     match receiver.try_recv() {
    //         Ok(data) => println!("Got a command {:?}", data),
    //         Err(err) => println!("Encountered error, data didnt reach or tx dropped. details {:?}", err )
    //     }

    // }

    while let Ok(data) = receiver.recv() {
        println!("Got a command {:?}", data);
    }
}
