use crossterm::{
    event::{self, Event, KeyCode},
    style,
};

use crate::render::frame::{Frame, Point};
use std::{
    io,
    time::{Duration, SystemTime},
};

use super::Mode;

pub struct ModeConfirm {
    active: bool,
    duration: Duration,
    start: SystemTime,
}

impl ModeConfirm {
    pub fn new() -> ModeConfirm {
        ModeConfirm {
            active: false,
            duration: Duration::from_secs(7),
            start: SystemTime::now(),
        }
    }

    pub fn mode_loop(&mut self, mut frame: Frame) -> io::Result<(Frame, Mode)> {
        // Start ?
        if !self.active {
            self.active = true;
            self.start = SystemTime::now();
        }

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc {} => {
                        return Ok((frame, Mode::Welcome));
                    }
                    KeyCode::Enter {} => {
                        return Ok((frame, Mode::Welcome));
                    }
                    _ => {}
                }
            }
        }

        if self.start.elapsed().unwrap() < self.duration {
            let time_left =
                (self.duration.as_secs() - self.start.elapsed().unwrap().as_secs()).to_string();

            frame = frame
                .resize(10, 50)
                .set_current_colors(style::Color::Blue, style::Color::Reset)
                .print_text("Press Enter to Confirm or Esc to cancel", Point::new(2, 4))
                .print_text("--", Point::new(4, 17))
                .print_text(time_left.as_str(), Point::new(4, 20))
                .print_text("--", Point::new(4, 22));
        } else {
            return Ok((frame, Mode::Quit));
        }

        Ok((frame, Mode::Confirm))
    }
}
