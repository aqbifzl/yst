use client::{
    client_config::TOP_BAR_SIZE,
    position_utils::{Position, PositionUnit, Rect},
    simple_box::SimpleBox,
    Drawable,
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
        Rect {
            x: 0,
            y: 0,
            w: cols,
            h: TOP_BAR_SIZE.into(),
        },
        true,
    );
    bar.add_content(
        "2024-04-16".to_string(),
        Position {
            x: PositionUnit::Center,
            y: PositionUnit::Center,
        },
    );

    bar.refresh();
    bar.draw();

    let mut applications_box = SimpleBox::new(
        Rect {
            x: 0,
            y: TOP_BAR_SIZE as i32,
            w: half_l,
            h: rows - TOP_BAR_SIZE as i32,
        },
        true,
    );
    applications_box.add_content(
        "1".to_string(),
        Position {
            x: PositionUnit::Val(0),
            y: PositionUnit::Val(0),
        },
    );
    applications_box.refresh();
    applications_box.draw();

    let mut titles_box = SimpleBox::new(
        Rect {
            x: half_l,
            y: TOP_BAR_SIZE as i32,
            w: half_r,
            h: rows - TOP_BAR_SIZE as i32,
        },
        true,
    );
    titles_box.add_content(
        "2".to_string(),
        Position {
            x: PositionUnit::Val(0),
            y: PositionUnit::Val(0),
        },
    );
    titles_box.refresh();
    titles_box.draw();

    refresh();
    getch();
    bar.destroy();
    applications_box.destroy();
    endwin();
}
