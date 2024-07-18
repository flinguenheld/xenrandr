use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

use super::Mode;
use crate::render::frame::Frame;
use crate::render::frame::{NB_COLS, NB_ROWS};

pub struct ModeWelcome {
    row: usize,
    col: usize,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        Self {
            row: NB_ROWS / 2,
            col: NB_COLS / 2,
        }
    }

    pub fn mode_loop(&mut self, frame: &mut Frame, mode: &mut Mode) -> io::Result<()> {
        let txt = "prout";

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        *mode = Mode::Quit;
                    }
                    KeyCode::Up => {
                        if self.row > 0 {
                            self.row -= 1;
                        } else {
                            self.row = NB_ROWS - 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.row < NB_ROWS - 1 {
                            self.row += 1;
                        } else {
                            self.row = 0;
                        }
                    }
                    KeyCode::Left => {
                        if self.col > 0 {
                            self.col -= 1;
                        } else {
                            self.col = NB_COLS - txt.len();
                        }
                    }
                    KeyCode::Right => {
                        if self.col < NB_COLS - txt.len() {
                            self.col += 1;
                        } else {
                            self.col = 0;
                        }
                    }
                    _ => {}
                }
            }
        }

        for (i, c) in txt.chars().enumerate() {
            // frame.cases[self.row][self.col + i].back_color = style::Color::Red;
            frame.cases[self.row][self.col + i].value = c;
        }
        Ok(())
    }
}
