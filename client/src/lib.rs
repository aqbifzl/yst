use position_utils::Position;

pub mod client;
pub mod client_config;
pub mod position_utils;
pub mod simple_box;
pub mod yst_storage;

pub trait Drawable {
    fn draw(&self);
    fn refresh(&self);
    fn destroy(&self);
}

pub trait ContentHolder {
    fn add_content(&mut self, str: String, position: Position);
    fn max_length(&self) -> u32;
    fn max_row(&self) -> u32;
}

pub trait DrawableContentHolder: Drawable + ContentHolder {}

pub trait Focusable {
    fn focus(&self);
    fn unfocus(&self);
}
