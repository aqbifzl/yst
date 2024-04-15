use client::{
    client_config::TOP_BAR_SIZE, simple_box::SimpleBox, Drawable, Position, PositionUnit,
};
use ncurses::{curs_set, endwin, getch, getmaxyx, initscr, refresh, setlocale, stdscr, LcCategory};

fn main() {
    setlocale(LcCategory::all, "").unwrap();
    initscr();
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    getmaxyx(stdscr(), &mut rows, &mut cols);

    let half_l = cols / 2;
    let half_r = cols - half_l;

    let mut bar = SimpleBox::new(
        client::Rect {
            x: 0,
            y: 0,
            w: cols,
            h: TOP_BAR_SIZE.into(),
        },
        true,
    );
    bar.set_content(
        "2024-04-15".to_string(),
        Position {
            x: PositionUnit::Val(0),
            y: PositionUnit::Center,
        },
    );

    bar.refresh();
    bar.draw();

    refresh();
    getch();
    bar.destroy();
    endwin();
}
