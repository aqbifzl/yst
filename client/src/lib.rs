pub mod client_config;
pub mod position_utils;
pub mod simple_box;
pub mod yst_storage;

pub trait Drawable {
    fn draw(&self);
    fn refresh(&self);
    fn destroy(&self);
}

pub trait Focusable {
    fn focus(&self);
    fn unfocus(&self);
}
