use self::events_manager::EventManager;
use self::handle_input_system::HandleInputSystem;
use self::play_gun_sound_system::PlayGunSoundSystem;

mod events;
mod events_manager;
mod handle_input_system;
mod play_gun_sound_system;

pub fn run() {
    let mut event_manager = EventManager::new();
    let mut handle_input_system = HandleInputSystem::new(&mut event_manager);
    let mut play_gun_sound_system = PlayGunSoundSystem::new(&mut event_manager);

    handle_input_system.run().unwrap();
    play_gun_sound_system.run().unwrap();
    event_manager.handle_events().unwrap();
    handle_input_system.run().unwrap();
    play_gun_sound_system.run().unwrap();
    handle_input_system.run().unwrap();
    play_gun_sound_system.run().unwrap();
}
