use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{io, time::Duration};

use super::Mode;
use crate::bash::xrandr_read;
use crate::render::frame::{Frame, Point, NB_COLS, NB_ROWS};
use crate::widget::{focus_next, focus_previous, WScreen};

#[derive(Debug, Default)]
pub struct ModeWelcome {
    wscreens: Vec<WScreen>,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        ModeWelcome {
            wscreens: xrandr_read(),
            ..Default::default()
        }
    }

    #[rustfmt::skip]
    pub fn mode_loop(&mut self, mut frame: Frame) -> io::Result<(Frame, Mode)> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {

                match key_event {
                    KeyEvent { code: KeyCode::Esc, .. } |
                    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. }
                     => {
                        return Ok((frame, Mode::Quit));
                    }

                    KeyEvent { code: KeyCode::Tab, .. } |
                    KeyEvent { code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, .. } => {
                        focus_next(&mut self.wscreens);
                    }

                    KeyEvent { code: KeyCode::BackTab, .. } |
                    KeyEvent { code: KeyCode::Char('p'), modifiers: KeyModifiers::CONTROL, .. } => {
                        focus_previous(&mut self.wscreens)
                    }

                    KeyEvent { code: KeyCode::F(5), .. } => {
                        self.wscreens = xrandr_read();
                    }

                    KeyEvent { code: KeyCode::Up, .. } => {
                        if let Some(current_wscreer) =
                            self.wscreens.iter_mut().find(|ws| ws.focused)
                        {
                            focus_next(&mut current_wscreer.combos);
                        }
                    }

                    KeyEvent { code: KeyCode::Down, .. } => {
                        if let Some(current_wscreer) =
                            self.wscreens.iter_mut().find(|ws| ws.focused)
                        {
                            focus_previous(&mut current_wscreer.combos);
                        }
                    }

                    KeyEvent { code: KeyCode::Right, .. } => {
                        if let Some(current_wscreer) = self.wscreens.iter_mut().find(|ws| ws.focused)
                        {
                            current_wscreer.next();
                        }
                    }

                    KeyEvent { code: KeyCode::Left, .. } => {
                        if let Some(current_wscreer) = self.wscreens.iter_mut().find(|ws| ws.focused)
                        {
                            current_wscreer.previous();
                        }
                    }
                    _ => {}
                }
            }
        }

        for ws in self.wscreens.iter() {
            frame = ws.draw(frame);
        }

        Ok((frame, Mode::Welcome))
    }
}
