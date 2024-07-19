use crossterm::event::{self, Event, KeyCode};
use std::{io, time::Duration};

use super::Mode;
use crate::render::frame::{Frame, Point, NB_COLS, NB_ROWS};
use crate::widget::WScreen;
use crate::xrandr::XScreens;

#[derive(Debug, Default)]
pub struct ModeWelcome {
    point_prout: Point,
    xscreens: XScreens,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        Self {
            point_prout: Point::new(NB_ROWS / 2, NB_COLS / 2),
            ..Default::default()
        }
    }

    pub fn mode_loop(&mut self, mut frame: Frame) -> io::Result<(Frame, Mode)> {
        let txt = "prout";

        self.xscreens.refresh();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => {
                        return Ok((frame, Mode::Quit));
                    }
                    KeyCode::Up => {
                        if self.point_prout.row > 0 {
                            self.point_prout.row -= 1;
                        } else {
                            self.point_prout.row = NB_ROWS - 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.point_prout.row < NB_ROWS - 1 {
                            self.point_prout.row += 1;
                        } else {
                            self.point_prout.row = 0;
                        }
                    }
                    KeyCode::Left => {
                        if self.point_prout.col > 0 {
                            self.point_prout.col -= 1;
                        } else {
                            self.point_prout.col = NB_COLS - txt.len();
                        }
                    }
                    KeyCode::Right => {
                        if self.point_prout.col < NB_COLS - txt.len() {
                            self.point_prout.col += 1;
                        } else {
                            self.point_prout.col = 0;
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut i = 2;
        for screen in self.xscreens.list.iter() {
            frame = WScreen::new(Point::new(5, i)).draw(frame, screen);
            i += 20;
        }

        frame = frame.print_text(txt, self.point_prout);

        Ok((frame, Mode::Welcome))
    }
}
