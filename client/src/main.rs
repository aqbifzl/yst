use std::cmp::max;

use client::client::init_client;
use ncurses::{curs_set, endwin, getch, initscr, noecho, setlocale, LcCategory};

fn main() {
    setlocale(LcCategory::all, "").unwrap();
    initscr();
    noecho();
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut drawables = init_client();

    loop {
        drawables.update_day();
        drawables.refresh();
        drawables.draw();
        let ch = char::from_u32(max(getch(), 0) as u32).unwrap();

        match ch {
            'q' => {
                break;
            }
            _ => continue,
        }
    }

    drawables.destroy();
    endwin();
}
