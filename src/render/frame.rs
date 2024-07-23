use crossterm::{
    cursor,
    style::{self, Print, SetBackgroundColor, SetForegroundColor},
    QueueableCommand,
};
use itertools::{EitherOrBoth::*, Itertools};
use std::io::{self, Stdout, Write};

// ----------------------
/// Point --
#[derive(Debug, Default, Clone, Copy)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    // TODO: Add operators ?
    pub fn new(row: usize, col: usize) -> Point {
        Self { row, col }
    }

    pub fn up(self, row: usize, col: usize) -> Point {
        Point {
            row: self.row + row,
            col: self.col + col,
        }
    }
    pub fn down(self, row: usize, col: usize) -> Point {
        Point {
            row: if row >= self.row { self.row - row } else { row },
            col: if col >= self.col { self.col - col } else { col },
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

#[derive(Debug, Clone)]
pub struct Frame {
    pub current: Vec<Vec<FrameCase>>,
    previous: Vec<Vec<FrameCase>>,
    current_back_color: style::Color,
    current_fore_color: style::Color,
}

impl Frame {
    pub fn new() -> Frame {
        Self {
            current: Vec::new(),
            previous: Vec::new(),
            current_fore_color: style::Color::White,
            current_back_color: style::Color::Reset,
        }
    }

    pub fn resize(mut self, rows: usize, columns: usize) -> Frame {
        self.current.clear();

        self.current = vec![
            vec![
                FrameCase {
                    value: ' ',
                    fore_color: style::Color::White,
                    back_color: style::Color::Reset,
                };
                columns
            ];
            rows
        ];

        self
    }

    pub fn render(mut self) -> Frame {
        let mut stdout = io::stdout();

        // First time, to have the same size
        let force = if self.previous.is_empty() {
            self.previous.clone_from(&self.current);
            true
        } else {
            false
        };

        for (r, row_pair) in self
            .current
            .iter()
            .zip_longest(self.previous.iter())
            .enumerate()
        {
            match row_pair {
                Both(row, row_prev) => {
                    for (c, case_pair) in row.iter().zip_longest(row_prev.iter()).enumerate() {
                        match case_pair {
                            Both(case, case_prev) => {
                                if *case != *case_prev || force {
                                    stdout.queue(cursor::MoveTo(c as u16, r as u16)).unwrap();
                                    stdout.queue(SetBackgroundColor(case.back_color)).unwrap();
                                    stdout.queue(SetForegroundColor(case.fore_color)).unwrap();
                                    stdout.queue(Print(case.value)).unwrap();
                                }
                            }
                            Right(_) => {
                                stdout = clear_case(stdout, c, r);
                            }
                            _ => {}
                        }
                    }
                }
                Right(row_prev) => {
                    for (c, _) in row_prev.iter().enumerate() {
                        stdout = clear_case(stdout, c, r);
                    }
                }
                _ => {}
            }
        }

        stdout.flush().unwrap();
        self.previous = self.current;
        Frame::new()
    }

    pub fn set_current_colors(mut self, fore: style::Color, back: style::Color) -> Frame {
        self.current_fore_color = fore;
        self.current_back_color = back;
        self
    }

    pub fn print_text(mut self, text: &str, point: Point) -> Frame {
        if let Some(row) = self.current.get_mut(point.row) {
            for (i, c) in text.chars().enumerate() {
                if let Some(case) = row.get_mut(point.col + i) {
                    case.value = c;
                    case.fore_color = self.current_fore_color;
                    case.back_color = self.current_back_color;
                }
            }
        }
        self
    }

    pub fn print_square(mut self, point: Point, heigth: usize, width: usize) -> Frame {
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

fn clear_case(mut stdout: Stdout, c: usize, r: usize) -> Stdout {
    stdout.queue(cursor::MoveTo(c as u16, r as u16)).unwrap();
    stdout
        .queue(SetBackgroundColor(style::Color::Reset))
        .unwrap();
    stdout.queue(Print(' ')).unwrap();
    stdout
}
