use std::sync::mpsc::{Receiver, Sender};

use eyre::Result;

use super::events::Events;
use super::events_manager::EventManager;

#[derive(Debug)]
pub struct PlayGunSoundSystem {
    event_sender: Sender<Events>,
    gun_fired_event: Receiver<Events>,
}

impl PlayGunSoundSystem {
    pub fn new(event_manager: &mut EventManager) -> Self {
        let event_sender = event_manager.register_as_sender();
        let gun_fired_event = event_manager.subscribe_to_event(Events::PlayerFiredBullet);

        Self {
            event_sender,
            gun_fired_event,
        }
    }
    pub fn run(&mut self) -> Result<()> {
        // if the fired gun event happens, then play the fired gun sound
        let gun_fired = if let Ok(event) = self.gun_fired_event.try_recv() {
            event
        } else {
            return Ok(());
        };

        // play the sound
        dbg!(gun_fired);

        self.event_sender.send(Events::SoundPlayed)?;

        Ok(())
    }
}
