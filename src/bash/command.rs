use crate::{render::frame::Point, widget::WScreen};
use std::process::Command;

pub fn xrandr_read() -> Vec<WScreen> {
    let mut screens: Vec<WScreen> = Vec::new();

    let xrandr = Command::new("sh")
        .arg("-c")
        .args(["xrandr"])
        .output()
        .expect("xrandr command failed.");

    let mut point = Point::new(2, 2);

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
                    screens.is_empty(),
                    point,
                ));

                point = point.up(0, 30);
            }
        }
    }

    // FIX: WRITE THAT FOR HYPRLAND AND GET THE CURRENT POSITIONS
    screens
}
// }
