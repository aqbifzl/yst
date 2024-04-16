use std::collections::HashMap;

use ncurses::{getmaxyx, stdscr};
use shared::utils::{get_specific_date, ms_to_human_readable_time_span};

use crate::{
    client_config::TOP_BAR_SIZE,
    position_utils::{Position, PositionUnit, Rect},
    simple_box::SimpleBox,
    yst_storage::get_content,
    DrawableContentHolder,
};

pub struct ClientContainer {
    pub bar: Box<dyn DrawableContentHolder>,
    pub apps: Box<dyn DrawableContentHolder>,
    pub titles: Box<dyn DrawableContentHolder>,
    day_offset: i32,
}

fn parse_rows(
    data: &HashMap<String, u128>,
    max_len: usize,
    max_row: usize,
) -> Vec<(String, Position)> {
    let mut rows = Vec::new();
    let mut data: Vec<_> = data.iter().collect();
    data.sort_by(|a, b| b.1.cmp(a.1));

    for (i, (key, value)) in data.iter().enumerate() {
        if i > max_row {
            break;
        }
        let name = key;
        let time = ms_to_human_readable_time_span(**value).trim().to_string();
        let time_len: usize = time.chars().map(|ch| ch.len_utf8()).sum();
        const SEPARATOR: &str = " - ";
        let separator_len = SEPARATOR.len();

        let mut truncated_name = String::new();
        let mut counter = 0;
        const ELIPSIS_LEN: usize = 3;
        let mut elipsis_counter = 0;

        for ch in name.chars() {
            let utf8_len = ch.len_utf8();
            if counter + utf8_len + ELIPSIS_LEN + separator_len + time_len <= max_len {
                truncated_name.push(ch);
                counter += utf8_len;
            } else if elipsis_counter < ELIPSIS_LEN {
                truncated_name.push('.');
                elipsis_counter += 1;
            } else {
                break;
            }
        }

        rows.push((
            format!("{}{}{}", truncated_name, SEPARATOR, time),
            Position {
                x: PositionUnit::Val(0),
                y: PositionUnit::Val(i.try_into().unwrap()),
            },
        ));
    }
    rows
}

pub enum ChangeDay {
    Previous,
    Next,
}

impl ClientContainer {
    pub fn update_day(&mut self) {
        self.titles.clear_content();
        self.apps.clear_content();
        self.update_bar_date();
        let data = match get_content(self.day_offset) {
            Ok(content) => content,
            Err(_) => {
                let s = "No data".to_string();
                let p = Position {
                    x: PositionUnit::Val(0),
                    y: PositionUnit::Val(0),
                };
                self.titles.add_content(&s, &p);
                self.apps.add_content(&s, &p);
                return;
            }
        };

        let parsed = parse_rows(
            &data.applications,
            self.apps.max_length(),
            self.apps.max_row(),
        );

        for (s, p) in parsed {
            self.apps.add_content(&s, &p);
        }

        let parsed = parse_rows(&data.titles, self.titles.max_length(), self.apps.max_row());
        for (s, p) in parsed {
            self.titles.add_content(&s, &p);
        }
    }
    pub fn change_day(&mut self, change: ChangeDay) {
        match change {
            ChangeDay::Next => self.day_offset += 1,
            ChangeDay::Previous => self.day_offset -= 1,
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
    pub fn update_bar_date(&mut self) {
        self.bar.clear_content();
        self.bar.add_content(
            &get_specific_date(self.day_offset),
            &Position {
                x: PositionUnit::Center,
                y: PositionUnit::Center,
            },
        );
    }
}

pub fn init_client() -> ClientContainer {
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;
    getmaxyx(stdscr(), &mut rows, &mut cols);

    let half_l = cols / 2;
    let half_r = cols - half_l;

    let bar = Box::new(SimpleBox::new_with_borders(Rect::new(
        0,
        0,
        cols,
        TOP_BAR_SIZE,
    )));

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
