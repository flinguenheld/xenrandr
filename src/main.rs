use crossbeam::channel::bounded;
use crossterm::{cursor, terminal, ExecutableCommand};
use std::{
    io::{self},
    thread::{sleep, spawn},
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

    // Force first frame
    Frame::new_frame().render(true);

    let (s, r) = bounded::<Frame>(1);
    let render_thread = spawn(move || {
        while let Ok(frame) = r.recv() {
            frame.render(false);
        }
    });

    // --
    let mut mode = Mode::Welcome;
    let mut mode_welcome = ModeWelcome::new();

    loop {
        let mut frame = Frame::new_frame();

        match mode {
            Mode::Welcome => (frame, mode) = mode_welcome.mode_loop(frame)?,
            _ => break,
        }

        let _ = s.send(frame);
        sleep(Duration::from_millis(20));
    }

    // Cleanup --
    drop(s);
    render_thread.join().unwrap();

    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
