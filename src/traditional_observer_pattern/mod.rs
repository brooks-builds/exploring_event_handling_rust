use std::sync::mpsc::Sender;

use eyre::Result;

use self::events::Events;
use self::handle_input_system::HandleInputSystem;
use self::play_gun_sound_system::PlayGunSoundSystem;

mod events;
mod handle_input_system;
mod play_gun_sound_system;

pub trait Subject {
    fn register(&mut self, observer: &mut impl Observer);
}

pub trait Observer {
    fn get_sender(&mut self) -> Sender<Events>;
}

pub fn run() -> Result<()> {
    let mut handle_input_system = HandleInputSystem::default();
    let mut play_gun_sound_system = PlayGunSoundSystem::new();

    handle_input_system.register(&mut play_gun_sound_system);

    handle_input_system.run()?;

    play_gun_sound_system.run()?;

    Ok(())
}
