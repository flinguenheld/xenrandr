use std::process::Command;

use crate::{render::frame::Point, widget::WScreen};

pub fn xrandr_read() -> Vec<WScreen> {
    let mut screens: Vec<WScreen> = Vec::new();

    let mut remove_that_juste_to_place_screens = 5;

    let xrandr = Command::new("sh")
        .arg("-c")
        .args(["xrandr"])
        .output()
        .expect("xrandr command failed.");

    for line in xrandr
        .stdout
        .iter()
        .map(|c| *c as char)
        .collect::<String>()
        .split('\n')
        .skip(1)
    {
        if !line.is_empty() {
            if line.starts_with("  ") {
                if let Some(resolution) = line.split_whitespace().next() {
                    if let Some(last_monitor) = screens.last_mut() {
                        last_monitor.combos[0].values.push(resolution.to_string());
                        if line.contains('+') {
                            last_monitor.combos[0].current_displayed =
                                last_monitor.combos[0].values.len() - 1;
                        }
                    }
                }
            } else if let Some(name) = line.split_whitespace().next() {
                screens.push(WScreen::new(
                    screens.len(),
                    name.to_string(),
                    Point::new(3, remove_that_juste_to_place_screens),
                    screens.is_empty(),
                ));
                remove_that_juste_to_place_screens += 20;
            }
        }
    }
    screens
}
// }
