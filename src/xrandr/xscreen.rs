use std::process::Command;

#[derive(Debug, Default, Clone)]
pub struct XScreen {
    pub number: u8,
    pub name: String,
    pub resolutions: Vec<String>,
    pub current_resolution: usize,
}

impl XScreen {
    pub fn new(number: u8, name: String) -> XScreen {
        Self {
            number,
            name,
            ..Default::default()
        }
    }
    pub fn current_res(&self) -> Option<String> {
        self.resolutions.get(self.current_resolution).cloned()
    }
}

#[derive(Debug, Default, Clone)]
pub struct XScreens {
    pub list: Vec<XScreen>,
}

impl XScreens {
    pub fn new() -> XScreens {
        Default::default()
    }

    pub fn refresh(&mut self) {
        self.list.clear();
        let mut number = 0;

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
                        if let Some(last_monitor) = self.list.last_mut() {
                            last_monitor.resolutions.push(resolution.to_string());
                            if line.contains('+') {
                                last_monitor.current_resolution =
                                    last_monitor.resolutions.len() - 1;
                            }
                        }
                    }
                } else if let Some(name) = line.split_whitespace().next() {
                    self.list.push(XScreen::new(number, name.to_string()));
                    number += 1;
                }
            }
        }
    }
}
