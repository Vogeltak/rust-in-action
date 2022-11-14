#[macro_use]
extern crate crossbeam;

use std::thread;
use crossbeam::channel::unbounded;

use crate::ConnectivityCheck::*;

#[derive(Debug)]
enum ConnectivityCheck {
    Pief,
    Paf,
    Poef,
}

fn main() {
    let n_message = 3;
    let (requests_tx, requests_rx) = unbounded();
    let (responses_tx, responses_rx) = unbounded();

    thread::spawn(move || loop {
        match requests_rx.recv().unwrap() {
            Paf => eprintln!("unexpected paf response"),
            Pief => responses_tx.send(Paf).unwrap(),
            Poef => return,
        }
    });

    for _ in 0..n_message {
        requests_tx.send(Pief).unwrap();
    }
    requests_tx.send(Poef).unwrap();

    for _ in 0..n_message {
        select!{
            recv(responses_rx) -> msg => println!("{:?}", msg),
        }
    }
}
