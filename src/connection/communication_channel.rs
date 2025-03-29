use std::sync::mpsc::{Receiver, Sender};

pub struct CommunicationChannel<S, R> {
    pub sender: Option<Sender<S>>,
    pub receiver: Option<Receiver<R>>,
}

impl<S, R> CommunicationChannel<S, R> {
    pub fn new(sender: Option<Sender<S>>, receiver: Option<Receiver<R>>) -> Self {
        Self { sender, receiver }
    }
}
