use std::sync::mpsc::Sender;

use eyre::Result;

use super::events::Events;
use super::events_manager::EventManager;

#[derive(Debug)]
pub struct HandleInputSystem {
    event_sender: Sender<Events>,
}

impl HandleInputSystem {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_sender = event_manager.register_as_sender();

        Self { event_sender }
    }

    pub fn run(&mut self) -> Result<()> {
        // player activates thruster for ship
        self.event_sender.send(Events::PlayerThrusting)?;

        // player fires gun
        self.event_sender.send(Events::PlayerFiredBullet)?;

        Ok(())
    }
}
