use std::sync::mpsc::{channel, Receiver, Sender};

use eyre::Result;

use super::events::Events;
use super::Observer;

pub struct PlayGunSoundSystem {
    subjects: Vec<Receiver<Events>>,
}

impl PlayGunSoundSystem {
    pub fn new() -> Self {
        Self { subjects: vec![] }
    }

    pub fn run(&mut self) -> Result<()> {
        for subject in &self.subjects {
            while let Ok(event) = subject.try_recv() {
                match event {
                    Events::PlayerFiredBullet => {
                        dbg!("playing gun sound");
                    }
                    Events::PlayerThrusting => (),
                };
            }
        }

        Ok(())
    }
}

impl Observer for PlayGunSoundSystem {
    fn get_sender(&mut self) -> Sender<Events> {
        let (sender, receiver) = channel::<Events>();
        self.subjects.push(receiver);
        sender
    }
}
