use crossterm::event::{KeyCode, KeyModifiers};

#[derive(Clone)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl Default for Vec2<usize> {
    fn default() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Key {
    Char(char),
    Ctrl(char),
    Esc,
    Enter,
}

impl Key {
    pub fn eq_crossterm(&self, key: (KeyCode, KeyModifiers)) -> bool {
        match self {
            Key::Char(c) => match key {
                (KeyCode::Char(c1), KeyModifiers::NONE) => c == &c1,
                _ => false,
            },
            Key::Ctrl(c) => match key {
                (KeyCode::Char(c1), KeyModifiers::CONTROL) => c == &c1,
                _ => false,
            },
            Key::Esc => match key {
                (KeyCode::Esc, KeyModifiers::NONE) => true,
                _ => false,
            },
            Key::Enter => match key {
                (KeyCode::Enter, KeyModifiers::NONE) => true,
                _ => false,
            },
        }
    }
}

impl From<(KeyCode, KeyModifiers)> for Key {
    fn from(key: (KeyCode, KeyModifiers)) -> Self {
        match key {
            (KeyCode::Char(c), KeyModifiers::NONE | KeyModifiers::SHIFT) => Key::Char(c),
            (KeyCode::Char(c), KeyModifiers::CONTROL) => Key::Ctrl(c),
            (KeyCode::Esc, KeyModifiers::NONE) => Key::Esc,
            (KeyCode::Enter, _) => Key::Enter,
            _ => panic!("Invalid key"),
        }
    }
}
