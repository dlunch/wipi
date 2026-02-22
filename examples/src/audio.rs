#![cfg_attr(not(test), no_main)]
#![no_std]
extern crate alloc;

use wipi::{app::App, audio::AudioClip, event::KeyCode, println, resource::Resource, wipi_main};

pub struct AudioApp {
    clip: AudioClip,
    volume: u8,
}

impl AudioApp {
    fn new() -> Self {
        let resource = Resource::new("audio.mmf").unwrap();
        let clip = AudioClip::from_mmf_bytes(resource.read()).unwrap();
        clip.play(false).unwrap();

        Self { clip, volume: 100 }
    }
}

impl App for AudioApp {
    fn on_keydown(&mut self, key_code: KeyCode) {
        match key_code {
            KeyCode::Ok => {
                self.clip.play(false).unwrap();
                println!("play");
            }
            KeyCode::Key1 => {
                self.clip.pause().unwrap();
                println!("pause");
            }
            KeyCode::Key2 => {
                self.clip.resume().unwrap();
                println!("resume");
            }
            KeyCode::Key3 => {
                self.clip.stop().unwrap();
                println!("stop");
            }
            KeyCode::Up => {
                self.volume = self.volume.saturating_add(10).min(100);
                self.clip.set_volume(self.volume).unwrap();
                println!("volume: {}", self.volume);
            }
            KeyCode::Down => {
                self.volume = self.volume.saturating_sub(10);
                self.clip.set_volume(self.volume).unwrap();
                println!("volume: {}", self.volume);
            }
            _ => {}
        }
    }
}

#[wipi_main]
pub fn main() -> AudioApp {
    AudioApp::new()
}
