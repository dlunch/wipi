use crate::event::{EventType, KeyCode};

pub trait App {
    fn on_paint(&mut self) {}
    fn on_pause(&mut self) {}
    fn on_resume(&mut self) {}
    fn on_keydown(&mut self, _key_code: KeyCode) {}
    fn on_keyup(&mut self, _key_code: KeyCode) {}

    fn on_raw_event(&mut self, r#type: i32, param1: i32, _param2: i32) {
        #[cfg(feature = "lgt")]
        let r#type = r#type - 501; // TODO should be in wipic_sys

        let event_type = EventType::from_raw(r#type);
        match event_type {
            EventType::KeyPress => {
                self.on_keydown(KeyCode::from_raw(param1));
            }
            EventType::KeyRelease => {
                self.on_keyup(KeyCode::from_raw(param1));
            }
            EventType::Unknown(_) => {}
        }
    }
}
