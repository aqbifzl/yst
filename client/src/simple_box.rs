use ncurses::{box_, delwin, mvaddstr, newwin, refresh, wrefresh, WINDOW};

use crate::{
    position_utils::{make_relative, Position, Rect, RelativePosition},
    Drawable,
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
        refresh();
        wrefresh(self.win);
    }
    fn destroy(&self) {
        delwin(self.win);
    }
}

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
    pub fn add_content(&mut self, str: String, position: Position) {
        let Rect { x, y, w, h } = self.rect;

        let border_size = if self.border { 1 } else { 0 };
        let relative = make_relative(position, Rect { x, y, w, h }, border_size, str.len() as i32);

        self.content.push(Content {
            str,
            position: relative,
        });
    }
}
