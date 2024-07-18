use crossterm::{
    cursor,
    style::{self, Print, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use std::io::{self, Write};

pub const NB_ROWS: usize = 20;
pub const NB_COLS: usize = 50;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameCase {
    pub value: char,
    pub fore_color: style::Color,
    pub back_color: style::Color,
}

pub struct Frame {
    pub cases: [[FrameCase; NB_COLS]; NB_ROWS],
    base: [[FrameCase; NB_COLS]; NB_ROWS],
}

impl Frame {
    pub fn new_frame() -> Frame {
        Self {
            cases: [[FrameCase {
                value: ' ',
                fore_color: style::Color::White,
                back_color: style::Color::Black,
            }; NB_COLS]; NB_ROWS],

            base: [[FrameCase {
                value: ' ',
                fore_color: style::Color::White,
                back_color: style::Color::Blue,
            }; NB_COLS]; NB_ROWS],
        }
    }

    pub fn render(&self, force: bool) {
        let mut stdout = io::stdout();

        for (r, (row, base_row)) in self.cases.iter().zip(self.base).enumerate() {
            for (c, (case, base_case)) in row.iter().zip(base_row).enumerate() {
                if force || *case != base_case {
                    stdout.queue(cursor::MoveTo(c as u16, r as u16)).unwrap();
                    stdout.queue(SetBackgroundColor(case.back_color)).unwrap();
                    stdout.queue(SetForegroundColor(case.fore_color)).unwrap();
                    stdout.queue(Print(case.value)).unwrap();
                }
            }
        }

        stdout.flush().unwrap();
    }
}
