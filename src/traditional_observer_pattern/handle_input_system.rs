use std::sync::mpsc::Sender;

use eyre::Result;

use super::events::Events;
use super::{Observer, Subject};

#[derive(Default)]
pub struct HandleInputSystem {
    observers: Vec<Sender<Events>>,
}

impl HandleInputSystem {
    pub fn run(&mut self) -> Result<()> {
        let mut events = vec![];

        // space is hit, we are going to fire the gun
        events.push(Events::PlayerFiredBullet);

        // up arrow is hit, time to thrust the ship
        events.push(Events::PlayerThrusting);

        for observer in &self.observers {
            for event in &events {
                observer.send(*event)?;
            }
        }

        Ok(())
    }
}

impl Subject for HandleInputSystem {
    fn register(&mut self, observer: &mut impl Observer) {
        let sender = observer.get_sender();
        self.observers.push(sender);
    }
}
