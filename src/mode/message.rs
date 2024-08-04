use super::Mode;
use crate::render::frame::{Frame, Point};
use crossterm::{
    event::{self, Event, KeyCode},
    style,
};
use std::{io, time::Duration};

pub struct ModeMessage {}

impl ModeMessage {
    pub fn new() -> ModeMessage {
        ModeMessage {}
    }

    pub fn mode_loop(&mut self, mode: Mode, mut frame: Frame) -> io::Result<(Frame, Mode)> {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc {} => {
                        return Ok((frame, Mode::Quit));
                    }
                    KeyCode::Enter {} => {
                        return Ok((frame, Mode::Welcome));
                    }
                    _ => {}
                }
            }
        }

        if let Mode::Message(text) = mode {
            frame = frame
                .resize(10, 50)
                .set_current_colors(style::Color::DarkYellow, style::Color::DarkYellow)
                .print_rectangle(
                    Point::new(1, 2),
                    45,
                    text.chars().filter(|c| *c == '\n').count() + 4,
                )
                .set_current_colors(style::Color::Yellow, style::Color::Reset);

            for (i, line) in text.split('\n').enumerate() {
                frame = frame.print_text(format!("{:^40}", line).as_str(), Point::new(i + 3, 5));
            }
            Ok((frame, Mode::Message(text)))
        } else {
            Ok((frame, Mode::Message("Fail".to_string())))
        }
    }
}
