use crate::{render::frame::Point, widget::WScreen};
use std::{cmp::max, process::Command};

pub fn hyprland_read() -> Vec<WScreen> {
    let mut screens: Vec<WScreen> = Vec::new();

    let cmd = Command::new("sh")
        .arg("-c")
        .args(["hyprctl monitors all | grep -E 'ID| at |scale|transform|Modes:'"])
        .output()
        .expect("hyprctl command failed.");

    // Example :
    // Monitor HDMI-A-1 (ID 0):
    // 3840x2160@59.99700 at 0x0
    // scale: 1.50
    // transform: 0
    // availableModes: 3840x2160@60.00Hz 3840x2160@60.00Hz ...

    let mut count = 0;
    let mut max_frequency = 0;

    for line in cmd
        .stdout
        .iter()
        .map(|c| *c as char)
        .collect::<String>()
        .split_whitespace()
    {
        if line.contains("Monitor") {
            screens.push(WScreen::new());
            add_frequencies(&mut screens, max_frequency);

            count = 0;
            max_frequency = 0;
        }

        if let Some(last) = screens.last_mut() {
            last.focused = true;
            match count {
                0 | 2 | 5 | 7 | 9 | 11 => {}
                1 => {
                    if let Some(name) = line.split_whitespace().next() {
                        last.name = name.to_string();
                    }
                }
                3 => {
                    // Number
                    if let Ok(num) = line
                        .chars()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse()
                    {
                        last.number = num;
                    }
                }
                4 => {
                    // Current resolution + Frequency
                    if let Some((res, frequency)) = line.split_once('@') {
                        last.combos[0].default = res.to_string();

                        // Round frequency
                        if let Ok(f) = frequency.parse::<f32>() {
                            let mut f = f.ceil() as u32;
                            while f % 10 != 0 {
                                f += 1;
                            }

                            last.combos[1].default = f.to_string();
                        }
                    }
                }
                6 => {
                    // Current position
                    if let Some((x, y)) = line.split_once('x') {
                        last.scale_point(Point::new(
                            y.parse::<usize>().unwrap_or(0),
                            x.parse::<usize>().unwrap_or(0),
                        ));
                    }
                }

                8 => {
                    // Current scale
                    last.combos[3].default = (line[..3]).to_string();
                }

                10 => {
                    // Current transform (rotation)
                    last.combos[2].default = last.combos[2]
                        .values
                        .get(line.parse::<usize>().unwrap_or(0))
                        .unwrap_or(&"".to_string())
                        .to_string();
                }

                _ => {
                    // All resolutions
                    if let Some((res, frequency)) = line.split_once('@') {
                        if !last.combos[0].values.contains(&res.to_string()) {
                            last.combos[0].values.push(res.to_string());
                        }

                        if let Ok(freq) = frequency[..2].parse::<u32>() {
                            max_frequency = max(freq, max_frequency);
                        }
                    }
                }
            }
        }

        count += 1;
    }

    add_frequencies(&mut screens, max_frequency);

    screens.iter_mut().for_each(|s| s.display_defaults());
    screens
}

/// Add frequencies from 10 to max into the last screen
pub fn add_frequencies(screens: &mut [WScreen], mut max_frequency: u32) {
    if let Some(last) = screens.last_mut() {
        while max_frequency % 10 != 0 {
            max_frequency += 1;
        }

        last.combos[1].values = (10..=max(max_frequency, 20))
            .step_by(5)
            .map(|v| v.to_string())
            .collect::<Vec<String>>();
    }
}

// pub fn xrandr_read() -> Vec<WScreen> {
//     let mut screens: Vec<WScreen> = Vec::new();

//     let xrandr = Command::new("sh")
//         .arg("-c")
//         .args(["xrandr"])
//         .output()
//         .expect("xrandr command failed.");

//     let mut point = Point::new(2, 2);

//     for line in xrandr
//         .stdout
//         .iter()
//         .map(|c| *c as char)
//         .collect::<String>()
//         .split('\n')
//         .skip(1)
//     {
//         if !line.is_empty() {
//             if line.starts_with("  ") {
//                 if let Some(resolution) = line.split_whitespace().next() {
//                     if let Some(last_monitor) = screens.last_mut() {
//                         last_monitor.combos[0].values.push(resolution.to_string());
//                         if line.contains('+') {
//                             last_monitor.combos[0].current_displayed =
//                                 last_monitor.combos[0].values.len() - 1;
//                         }
//                     }
//                 }
//             } else if let Some(_name) = line.split_whitespace().next() {
//                 screens.push(WScreen::new(
//                     // screens.len(),
//                     // name.to_string(),
//                     // screens.is_empty(),
//                     // point,
//                 ));

//                 point = point.up(0, 30);
//             }
//         }
//     }

// }
// }
