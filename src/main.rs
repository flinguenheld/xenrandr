// use crossbeam::channel::bounded;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::{
    io::{self},
    thread::sleep,
    time::Duration,
};

use xenrandr::mode::{confirm::ModeConfirm, message::ModeMessage, welcome::ModeWelcome, Mode};
use xenrandr::render::frame::Frame;

fn main() -> io::Result<()> {
    // Setup --
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(cursor::Hide)?;

    // --
    let mut mode = Mode::Welcome(true);
    let mut mode_welcome = ModeWelcome::new();
    let mut mode_confirm = ModeConfirm::new();
    let mut mode_message = ModeMessage::new();
    let mut frame = Frame::new();

    loop {
        match mode {
            Mode::Confirm(restart) => {
                (frame, mode) = mode_confirm.mode_loop(Mode::Confirm(restart), frame)?
            }
            Mode::Message(txt) => {
                (frame, mode) = mode_message.mode_loop(Mode::Message(txt), frame)?
            }
            Mode::Welcome(reload) => (frame, mode) = mode_welcome.mode_loop(frame, reload)?,
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
