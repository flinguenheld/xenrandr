use crossterm::{
    cursor,
    style::{self, Print, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use std::io::{self, Write};

pub const NB_ROWS: usize = 20;
pub const NB_COLS: usize = 70;

// ----------------------
/// Point --
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Point {
        Self { row, col }
    }

    pub fn up(self, row: usize, col: usize) -> Point {
        Point {
            row: self.row + row,
            col: self.col + col,
        }
    }
}
// ----------------------
/// Frame --
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameCase {
    pub value: char,
    pub fore_color: style::Color,
    pub back_color: style::Color,
}

#[derive(Debug, Clone, Copy)]
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

    pub fn print_text(mut self, text: &str, point: Point) -> Self {
        if let Some(row) = self.cases.get_mut(point.row) {
            for (i, c) in text.chars().enumerate() {
                if let Some(case) = row.get_mut(point.col + i) {
                    case.value = c;
                }
            }
        }
        self
    }

    pub fn print_square(mut self, point: Point, heigth: usize, width: usize) -> Self {
        self = self
            .print_text("─".repeat(width).as_str(), point)
            .print_text("─".repeat(width).as_str(), point.up(heigth, 0))
            .print_text("┌", point)
            .print_text("┐", point.up(0, width))
            .print_text("┘", point.up(heigth, width))
            .print_text("└", point.up(heigth, 0));

        for i in 1..heigth {
            self = self
                .print_text("│", Point::new(point.row + i, point.col))
                .print_text("│", Point::new(point.row + i, point.col + width))
        }

        self
    }
}
