use std::cmp::max;

use ncurses::{box_, delwin, mvaddstr, newwin, refresh, wclear, wrefresh, WINDOW};

use crate::{
    position_utils::{make_relative, Position, Rect, RelativePosition},
    ContentHolder, Drawable, DrawableContentHolder,
};

pub struct SimpleBox {
    rect: Rect<i32>,
    win: WINDOW,
    content: Vec<Content>,
    border: bool,
}

pub struct Content {
    pub str: String,
    pub position: RelativePosition,
}

impl Drawable for SimpleBox {
    fn draw(&self) {
        for content in &self.content {
            mvaddstr(content.position.y, content.position.x, &content.str).unwrap();
        }

        refresh();
    }
    fn refresh(&self) {
        wclear(self.win);
        if self.border {
            box_(self.win, 0, 0);
        }
        refresh();
        wrefresh(self.win);
    }
    fn destroy(&self) {
        delwin(self.win);
    }
}

impl DrawableContentHolder for SimpleBox {}

impl SimpleBox {
    pub fn new(rect: Rect<i32>, border: bool) -> Self {
        assert!(rect.w > 1 && rect.h > 1);
        let Rect { x, y, w, h } = rect;
        let win = newwin(h, w, y, x);

        if border {
            box_(win, 0, 0);
        }

        Self {
            rect,
            win,
            content: Vec::new(),
            border,
        }
    }
    pub fn new_with_borders(rect: Rect<i32>) -> Self {
        Self::new(rect, true)
    }
}

impl ContentHolder for SimpleBox {
    fn clear_content(&mut self) {
        self.content.clear();
    }
    fn add_content(&mut self, str: &str, position: &Position) {
        let str = str.to_string();
        let Rect { x, y, w, h } = self.rect;

        let border_size = if self.border { 1 } else { 0 };
        let relative = make_relative(position, Rect { x, y, w, h }, border_size, str.len() as i32);

        self.content.push(Content {
            str,
            position: relative,
        });
    }
    fn max_length(&self) -> usize {
        let border_size = if self.border { 1 } else { 0 };
        max(self.rect.w - (border_size * 2), 0) as usize
    }
    fn max_row(&self) -> usize {
        let border_size = if self.border { 1 } else { 0 };
        max(self.rect.h - (border_size * 2) - 1, 0) as usize
    }
}
