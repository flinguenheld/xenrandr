use super::Mode;
use crate::bash::xrandr_read;
use crate::render::frame::{Frame, Point};
use crate::widget::{focus_next, focus_previous, Focus, WScreen, WSCREEN_HEIGHT, WSCREEN_WIDTH};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{io, time::Duration};

#[derive(Debug, Default)]
pub struct ModeWelcome {
    wscreens: Vec<Vec<WScreen>>,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        ModeWelcome {
            wscreens: xrandr_read(),
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
                        if let Some((row, col)) = self.get_focused_position()  {
                            self.wscreens[row][col].set_focus(false);
                            if let Some(next) = self.wscreens[row].get_mut(col + 1) {
                                next.set_focus(true);
                            } else if let Some(next_row) = self.wscreens.get_mut(row + 1) {
                                next_row.first_mut().unwrap().set_focus(true);
                            } else {
                                self.wscreens[0][0].set_focus(true);
                            }
                        }
                    }

                    KeyEvent { code: KeyCode::BackTab, .. } |
                    KeyEvent { code: KeyCode::Char('p'), modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some((row, col)) = self.get_focused_position()  {
                            self.wscreens[row][col].set_focus(false);
                            if col > 0 {
                                self.wscreens[row][col - 1].set_focus(true);
                            } else if row > 0 {
                                self.wscreens[row-1].last_mut().unwrap().set_focus(true);
                            } else {
                                self.wscreens.last_mut().unwrap().last_mut().unwrap().set_focus(true);
                            }
                        }
                    }

                    KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some((row, col)) = self.get_focused_position(){
                            if col < self.wscreens[row].len() - 1 {
                                let ws = self.wscreens[row].remove(col);
                                self.wscreens[row].insert(col + 1, ws)
                            }
                        }
                    }

                    KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some((row, col)) = self.get_focused_position(){
                            if col > 0 {
                                let ws = self.wscreens[row].remove(col);
                                self.wscreens[row].insert(col - 1, ws)
                            }
                        }
                    }

                    KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some((row, col)) = self.get_focused_position() {
                            let ws = self.wscreens[row].remove(col);
                            if row == 0 {
                                self.wscreens.insert(0, vec![ws]);
                            } else {
                                let col = if col > self.wscreens[row-1].len() { self.wscreens[row-1].len() } else { col };
                                self.wscreens[row - 1].insert(col, ws);
                            }
                            self.wscreens.retain(|line| !line.is_empty());
                        }
                    }

                    KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some((row, col)) = self.get_focused_position() {
                            let ws = self.wscreens[row].remove(col);
                            if row == self.wscreens.len() - 1 {
                                self.wscreens.push(vec![ws]);
                            } else {
                                let col = if col > self.wscreens[row+1].len() { self.wscreens[row+1].len() } else { col };
                                self.wscreens[row + 1].insert(col, ws);
                            }
                            self.wscreens.retain(|line| !line.is_empty());
                        }
                    }

                    KeyEvent { code: KeyCode::F(5), .. } => {
                        self.wscreens = xrandr_read();
                    }

                    KeyEvent { code: KeyCode::Up, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().flatten().find(|ws| ws.focused) {
                            focus_next(&mut current_wscreen.combos);
                        }
                    }

                    KeyEvent { code: KeyCode::Down, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().flatten().find(|ws| ws.focused) {
                            focus_previous(&mut current_wscreen.combos);
                        }
                    }

                    KeyEvent { code: KeyCode::Right, .. } => {
                        if let Some(current_wscreer) = self.wscreens.iter_mut().flatten().find(|ws| ws.focused) {
                            current_wscreer.next();
                        }
                    }

                    KeyEvent { code: KeyCode::Left, .. } => {
                        if let Some(current_wscreer) = self.wscreens.iter_mut().flatten().find(|ws| ws.focused) {
                            current_wscreer.previous();
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut pt = Point::new(2, 2);
        let heigth = 4 + (self.wscreens.len() + 1) * WSCREEN_HEIGHT;
        let width = 4 + (self.wscreens.first().unwrap().len() + 2) * WSCREEN_WIDTH;
        frame = frame.resize(heigth, width);

        for line in self.wscreens.iter() {
            for ws in line.iter() {
                frame = ws.draw(frame, pt);
                pt = pt.up(0, WSCREEN_WIDTH + 2);
            }
            pt = pt.up(WSCREEN_HEIGHT + 1, 0);
            pt.col = 2;
        }

        Ok((frame, Mode::Welcome))
    }

    /// Get Some(row, col)
    fn get_focused_position(&self) -> Option<(usize, usize)> {
        for (r, row) in self.wscreens.iter().enumerate() {
            for (c, ws) in row.iter().enumerate() {
                if ws.is_focus() {
                    return Some((r, c));
                }
            }
        }
        None
    }
}
