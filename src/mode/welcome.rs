use super::Mode;
use crate::bash::hyprland_read;
use crate::render::frame::Frame;
use crate::widget::{focus_next, focus_previous, WScreen};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::cmp::max;
use std::{io, time::Duration};

#[derive(Debug, Default)]
pub struct ModeWelcome {
    wscreens: Vec<WScreen>,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        ModeWelcome {
            wscreens: hyprland_read(),
        }
    }

    #[rustfmt::skip]
    pub fn mode_loop(&mut self, mut frame: Frame) -> io::Result<(Frame, Mode)> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {

                match key_event {
                    KeyEvent { code: KeyCode::Esc, .. } |
                    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. } => {
                        return Ok((frame, Mode::Quit));
                    }

                    KeyEvent { code: KeyCode::Tab, .. } |
                    KeyEvent { code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, .. } => {
                        focus_next(&mut self.wscreens);
                    }

                    KeyEvent { code: KeyCode::BackTab, .. } |
                    KeyEvent { code: KeyCode::Char('p'), modifiers: KeyModifiers::CONTROL, .. } => {
                        focus_previous(&mut self.wscreens);
                    }

                    KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreen.point.col += 1;
                        }
                    }
                    KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            if current_wscreen.point.col > 0 {
                                current_wscreen.point.col -= 1;
                            }
                        }
                    }
                    KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            if current_wscreen.point.row > 0 {
                                current_wscreen.point.row -= 1;
                            }
                        }
                    }
                    KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreen.point.row += 1;
                        }
                    }

                    KeyEvent { code: KeyCode::F(5), .. } => {
                        self.wscreens = hyprland_read();
                    }

                    KeyEvent { code: KeyCode::Up, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            focus_previous(&mut current_wscreen.combos);
                        }
                    }

                    KeyEvent { code: KeyCode::Down, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            focus_next(&mut current_wscreen.combos);
                        }
                    }

                    KeyEvent { code: KeyCode::Right, .. } => {
                        if let Some(current_wscreer) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreer.next_inside_focus();
                        }
                    }

                    KeyEvent { code: KeyCode::Left, .. } => {
                        if let Some(current_wscreer) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreer.previous_inside_focus();
                        }
                    }
                    _ => {}
                }
            }
        }

        // Take the furthest wscreen to resize the terminal.
        let (mut max_row, mut max_col) = (0, 0);
        for ws in self.wscreens.iter() {
            let (r, c) = ws.space_reclaimed();
            max_row = max(max_row, r);
            max_col = max(max_col, c);
        }
        frame = frame.resize(max_row + 1, max_col + 1);

        for ws in self.wscreens.iter() {
            frame = ws.draw(frame);
        }

        Ok((frame, Mode::Welcome))
    }
}
