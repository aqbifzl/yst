pub mod client_config;
pub mod simple_box;

pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
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
    x: i32,
    y: i32,
}

pub fn make_relative(position: Position, rect: Rect<i32>) -> RelativePosition {
    let x = match position.x {
        PositionUnit::Val(val) => val,
        PositionUnit::Center => rect.w / 2,
    };
    let y = match position.y {
        PositionUnit::Val(val) => val,
        PositionUnit::Center => rect.h / 2,
    };

    RelativePosition { x, y }
}

pub trait Drawable {
    fn draw(&self);
    fn refresh(&self);
    fn destroy(&self);
}

pub trait Focusable {
    fn focus(&self);
    fn unfocus(&self);
}
