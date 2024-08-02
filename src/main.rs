// use crossbeam::channel::bounded;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::{
    io::{self},
    thread::sleep,
    time::Duration,
};

use xenrandr::mode::{confirm::ModeConfirm, welcome::ModeWelcome, Mode};
use xenrandr::render::frame::Frame;

fn main() -> io::Result<()> {
    // Setup --
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // --
    let mut mode = Mode::Welcome;
    let mut mode_welcome = ModeWelcome::new();
    let mut mode_confirm = ModeConfirm::new();
    let mut frame = Frame::new();

    loop {
        match mode {
            Mode::Confirm => (frame, mode) = mode_confirm.mode_loop(frame)?,
            Mode::Welcome => (frame, mode) = mode_welcome.mode_loop(frame)?,
            _ => break,
        }

        frame = frame.render();
        sleep(Duration::from_millis(100));
    }

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
