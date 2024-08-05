use super::{Mode, HYPR_BAK, HYPR_CONF};
use crate::bash::hyprland_read;
use crate::render::frame::Frame;
use crate::widget::{focus_next, focus_previous, Focus, WScreen, DISPLAY_SCALE};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::{cmp::max, env, fs, io, path::Path, time::Duration};

#[derive(Debug, Default)]
pub struct ModeWelcome {
    wscreens: Vec<WScreen>,
}

impl ModeWelcome {
    pub fn new() -> ModeWelcome {
        ModeWelcome {
            wscreens: Vec::new(),
        }
    }

    #[rustfmt::skip]
    pub fn mode_loop(&mut self, mut frame: Frame, reload : bool) -> io::Result<(Frame, Mode)> {

        if reload {
            self.wscreens= hyprland_read();
        }

        if self.wscreens.is_empty() {
            return Ok((frame, Mode::Message("No screen found.\nThe 'hyprctl monitors'\
                                             command\nhas failed".to_string())));
        }
        
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {

                match key_event {
                    KeyEvent { code: KeyCode::Esc, .. } |
                    KeyEvent { code: KeyCode::Char('c'), modifiers: KeyModifiers::CONTROL, .. } => {
                        return Ok((frame, Mode::Quit));
                    }

                    KeyEvent { code: KeyCode::Enter, .. } => {

                       let path = Path::new(&env::var_os("HOME").unwrap()).join(HYPR_CONF);
                       let path_bak = Path::new(&env::var_os("HOME").unwrap()).join(HYPR_BAK);

                       // Save current conf
                       if std::fs::copy(&path, path_bak).is_ok() {

                            // Replace lines
                            let mut txt = String::new();
                            for screen in self.wscreens.iter() {

                                if screen.combos[4].current_value() == "true"
                                {
                                    // TODO: Error if it's the current screen or remove the option !
                                    txt= format!("{}monitor={},disable",txt, screen.name);

                                } else {

                                    txt= format!("{}monitor={},{}@{},{}x{},{},transform,{}\n",
                                                    txt,
                                                    screen.name,
                                                    screen.combos[0].current_value(),    // resolution
                                                    screen.combos[1].current_value(),    // frequency
                                                    screen.point.col,                    // position
                                                    screen.point.row,
                                                    screen.combos[3].current_value(),    // scale
                                                    screen.combos[2].current_displayed); // rotation
                                    }
                            }
                            // return Ok((frame, Mode::Message(txt)));

                            // txt = "monitor=,preferred,auto,1\n".to_string();

                            if let Ok(file) = fs::read_to_string(&path) {
                                let mut new_file = String::new();
                                let mut done = false;

                                for line in file.lines() {
                                    if !line.starts_with("monitor=") {
                                        new_file.push_str(line);
                                        new_file.push('\n');
                                    } else if !done {
                                        new_file.push_str(txt.as_str());
                                        done = true;
                                    }
                                }

                                if fs::write(path, new_file).is_ok() {
                                    return Ok((frame, Mode::Confirm(true)));

                                } else {
                                    return Ok((frame, Mode::Message("The attempt to write the \n~/.config/\
                                                      hypr/.hyprland.bak file\nhas failed.".to_string())));
                                }
                            } else {
                                return Ok((frame, Mode::Message("Impossible to read the \n~/.config/\
                                                  hypr/.hyprland.bak file.".to_string())));
                            }
                           
                       } else {
                            return Ok((frame, Mode::Message("The attempt to copy the \n~/.config/\
                                              hypr/hyprland.conf file\nhas failed.".to_string())));
                       }
                    }

                    KeyEvent { code: KeyCode::Tab, .. } |
                    KeyEvent { code: KeyCode::Char('n'), modifiers: KeyModifiers::CONTROL, .. } => {
                        focus_next(&mut self.wscreens);
                    }

                    KeyEvent { code: KeyCode::BackTab, .. } |
                    KeyEvent { code: KeyCode::Char('p'), modifiers: KeyModifiers::CONTROL, .. } => {
                        focus_previous(&mut self.wscreens);
                    }

                    // TODO: USE SCALE !!!!!
                    KeyEvent { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreen.point.col += DISPLAY_SCALE;
                        }
                    }
                    KeyEvent { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            if current_wscreen.point.col > 0 {
                                current_wscreen.point.col -= DISPLAY_SCALE;
                            }
                        }
                    }
                    KeyEvent { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            if current_wscreen.point.row > 0 {
                                current_wscreen.point.row -= DISPLAY_SCALE;
                            }
                        }
                    }
                    KeyEvent { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreen.point.row += DISPLAY_SCALE;
                        }
                    }

                    KeyEvent { code: KeyCode::F(5), .. } => {
                        return Ok((frame, Mode::Welcome(true)))
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
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreen.next_inside_focus();
                        }
                    }

                    KeyEvent { code: KeyCode::Left, .. } => {
                        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
                            current_wscreen.previous_inside_focus();
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
            if !ws.is_focus() {
                frame = ws.draw(frame);
            }
        }

        if let Some(current_wscreen) = self.wscreens.iter_mut().find(|ws| ws.focused) {
            frame = current_wscreen.draw(frame);
        }


        Ok((frame, Mode::Welcome(false)))
    }
}
