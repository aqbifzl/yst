use ncurses::{box_, delwin, mvaddstr, newwin, refresh, wrefresh, WINDOW};

use crate::{make_relative, Drawable, Position, Rect, RelativePosition};

pub struct SimpleBox {
    rect: Rect<i32>,
    win: WINDOW,
    content: Option<Content>,
    border: bool,
}

pub struct Content {
    pub str: String,
    pub position: RelativePosition,
}

impl Drawable for SimpleBox {
    fn draw(&self) {
        let content = match &self.content {
            Some(content) => content,
            None => return,
        };

        mvaddstr(content.position.y, content.position.y, &content.str).unwrap();
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
            content: None,
            border,
        }
    }
    pub fn set_content(&mut self, str: String, position: Position) {
        let Rect { x, y, w, h } = self.rect;

        let border_size = if self.border { 1 } else { 0 };
        let relative = make_relative(
            position,
            Rect {
                x: x + border_size,
                y: y + border_size,
                w: w - border_size,
                h: h - border_size,
            },
        );

        self.content = Some(Content {
            str,
            position: relative,
        });
    }
}
