pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl Default for Vec2<usize> {
    fn default() -> Self {
        Vec2 { x: 0, y: 0 }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
