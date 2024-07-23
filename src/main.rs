// use crossbeam::channel::bounded;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::{
    io::{self},
    thread::sleep,
    time::Duration,
};

use xenrandr::mode::{welcome::ModeWelcome, Mode};
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
    let mut frame = Frame::new();

    loop {
        match mode {
            Mode::Welcome => (frame, mode) = mode_welcome.mode_loop(frame)?,
            _ => break,
        }

        frame = frame.render();
        sleep(Duration::from_millis(50));
    }

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
