pub enum EventType {
    KeyPress,
    KeyRelease,
    Unknown(i32),
}

impl EventType {
    pub fn from_raw(raw: i32) -> Self {
        match raw {
            1 => EventType::KeyPress,
            2 => EventType::KeyRelease,
            other => EventType::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    Hash,
    Star,
    Up,
    Down,
    Left,
    Right,
    Ok,
    Back,
    Call,
    End,
    Unknown(i32),
}

impl KeyCode {
    pub fn from_raw(raw: i32) -> Self {
        match raw {
            48 => KeyCode::Key0,
            49 => KeyCode::Key1,
            50 => KeyCode::Key2,
            51 => KeyCode::Key3,
            52 => KeyCode::Key4,
            53 => KeyCode::Key5,
            54 => KeyCode::Key6,
            55 => KeyCode::Key7,
            56 => KeyCode::Key8,
            57 => KeyCode::Key9,
            35 => KeyCode::Hash,
            42 => KeyCode::Star,
            -1 => KeyCode::Up,
            -2 => KeyCode::Down,
            -3 => KeyCode::Left,
            -4 => KeyCode::Right,
            -5 => KeyCode::Ok,
            -16 => KeyCode::Back,
            other => KeyCode::Unknown(other),
        }
    }
}
