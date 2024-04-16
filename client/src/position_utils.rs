pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Self { x, y, w, h }
    }
}

pub enum PositionUnit {
    Val(i32),
    Center,
}

pub struct Position {
    pub x: PositionUnit,
    pub y: PositionUnit,
}

pub struct RelativePosition {
    pub x: i32,
    pub y: i32,
}

pub fn make_relative(
    position: Position,
    rect: Rect<i32>,
    border_size: u8,
    xoffset: i32,
) -> RelativePosition {
    let x = match position.x {
        PositionUnit::Val(val) => rect.x + val + border_size as i32,
        PositionUnit::Center => rect.x + (rect.w / 2) - (xoffset / 2),
    };
    let y = match position.y {
        PositionUnit::Val(val) => rect.y + val + border_size as i32,
        PositionUnit::Center => rect.y + rect.h / 2,
    };

    RelativePosition { x, y }
}
