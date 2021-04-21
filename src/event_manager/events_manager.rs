use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use eyre::Result;

use super::events::Events;

#[derive(Debug)]
pub struct EventManager {
    event_receiver: Receiver<Events>,
    event_sender: Sender<Events>,
    subscribers: HashMap<Events, Vec<Sender<Events>>>,
}

impl EventManager {
    pub fn new() -> Self {
        let (event_sender, event_receiver) = channel();

        Self {
            event_receiver,
            event_sender,
            subscribers: HashMap::new(),
        }
    }

    pub fn register_as_sender(&mut self) -> Sender<Events> {
        self.event_sender.clone()
    }

    pub fn subscribe_to_event(&mut self, event: Events) -> Receiver<Events> {
        let (subscribe_sender, subscribe_receiver) = channel();

        let subscriber = self.subscribers.entry(event).or_default();
        subscriber.push(subscribe_sender);

        subscribe_receiver
    }

    pub fn handle_events(&mut self) -> Result<()> {
        while let Ok(event) = &self.event_receiver.try_recv() {
            let subscribers = if let Some(subscribers) = self.subscribers.get(event) {
                subscribers
            } else {
                continue;
            };

            for subscriber in subscribers {
                subscriber.send(*event)?;
            }
        }

        Ok(())
    }
}
