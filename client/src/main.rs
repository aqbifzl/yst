use std::cmp::max;

use client::client::{init_client, ChangeDay};
use ncurses::{curs_set, endwin, getch, initscr, noecho, setlocale, LcCategory};

fn main() {
    setlocale(LcCategory::all, "").unwrap();
    initscr();
    noecho();
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut drawables = init_client();
    drawables.update_bar_date();

    loop {
        drawables.update_day();
        drawables.refresh();
        drawables.draw();
        let ch = char::from_u32(max(getch(), 0) as u32).unwrap();

        match ch {
            'q' => {
                break;
            }
            'j' => drawables.change_day(ChangeDay::Next),
            'k' => drawables.change_day(ChangeDay::Previous),
            _ => continue,
        }
    }

    drawables.destroy();
    endwin();
}
