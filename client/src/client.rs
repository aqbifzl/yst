use std::collections::HashMap;

use ncurses::{getmaxyx, stdscr};
use shared::utils::ms_to_human_readable_time_span;

use crate::{
    client_config::TOP_BAR_SIZE,
    position_utils::{Position, PositionUnit, Rect},
    simple_box::SimpleBox,
    yst_storage::get_content,
    ContentHolder, DrawableContentHolder,
};

pub struct ClientContainer {
    pub bar: Box<dyn DrawableContentHolder>,
    pub apps: Box<dyn DrawableContentHolder>,
    pub titles: Box<dyn DrawableContentHolder>,
    day_offset: i32,
}

fn parse_rows(data: &HashMap<String, u128>, max_len: usize) -> Vec<(String, Position)> {
    let mut rows = Vec::new();
    for (i, (key, value)) in data.iter().enumerate() {
        let mut row = format!(
            "{} - {}",
            key,
            ms_to_human_readable_time_span(*value).trim()
        );
        if row.len() > max_len {
            row = row[0..max_len - 3].to_string();
            row.push_str("...");
        }
        rows.push((
            row,
            Position {
                x: PositionUnit::Val(0),
                y: PositionUnit::Val(i.try_into().unwrap()),
            },
        ));
    }
    rows
}

impl ClientContainer {
    pub fn update_day(&mut self) {
        let data = get_content().unwrap();

        let parsed = parse_rows(&data.applications, self.apps.max_length() as usize);
        for (s, p) in parsed {
            self.apps.add_content(s, p);
        }

        let parsed = parse_rows(&data.titles, self.titles.max_length() as usize);
        for (s, p) in parsed {
            self.titles.add_content(s, p);
        }
    }
    pub fn refresh(&self) {
        self.bar.refresh();
        self.apps.refresh();
        self.titles.refresh();
    }
    pub fn draw(&self) {
        self.bar.draw();
        self.apps.draw();
        self.titles.draw();
    }
    pub fn destroy(&self) {
        self.bar.destroy();
        self.apps.destroy();
        self.titles.destroy();
    }
}

pub fn init_client() -> ClientContainer {
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    getmaxyx(stdscr(), &mut rows, &mut cols);

    let half_l = cols / 2;
    let half_r = cols - half_l;

    let mut bar = Box::new(SimpleBox::new_with_borders(Rect::new(
        0,
        0,
        cols,
        TOP_BAR_SIZE,
    )));

    bar.add_content(
        "today".to_string(),
        Position {
            x: PositionUnit::Center,
            y: PositionUnit::Center,
        },
    );

    let apps = Box::new(SimpleBox::new_with_borders(Rect::new(
        0,
        TOP_BAR_SIZE,
        half_l,
        rows - TOP_BAR_SIZE,
    )));

    let titles = Box::new(SimpleBox::new_with_borders(Rect::new(
        half_l,
        TOP_BAR_SIZE,
        half_r,
        rows - TOP_BAR_SIZE,
    )));

    ClientContainer {
        bar,
        apps,
        titles,
        day_offset: 0,
    }
}
