use super::{wcombobox::WComboBox, Focus};
use crate::render::frame::{Frame, Point};
use crossterm::style;
use std::cmp::max;

const SCALE: usize = 60;

#[derive(Debug, Default, Clone)]
pub struct WScreen {
    pub number: usize,
    pub name: String,

    pub point: Point,

    pub focused: bool,
    pub combos: Vec<WComboBox>, // Make it private ??
}

impl Focus for WScreen {
    fn is_focus(&self) -> bool {
        self.focused
    }
    fn set_focus(&mut self, value: bool) {
        self.focused = value;
    }
}

impl WScreen {
    // pub fn new(number: usize, name: String, focused: bool) -> WScreen {
    pub fn new() -> WScreen {
        let mut screen = Self {
            number: 0,
            name: String::new(),
            focused: false,
            point: Point::new(0, 0),

            combos: vec![
                WComboBox::new(Point::new(4, 2), "Res", ""),
                WComboBox::new(Point::new(5, 2), "Freq", "Hz"),
                WComboBox::new(Point::new(6, 2), "Rot", "°"),
                WComboBox::new(Point::new(7, 2), "Scale", ""),
                WComboBox::new(Point::new(8, 2), "Disabled", ""),
            ],
        };

        // Init --
        screen.combos[0].focused = true;
        screen.combos[2].values.append(&mut vec![
            "0".to_string(),
            "90".to_string(),
            "180".to_string(),
            "270".to_string(),
            "flip 0".to_string(),
            "flip 90".to_string(),
            "flip 180".to_string(),
            "flip 270".to_string(),
        ]);
        screen.combos[3].values = (0..51)
            .skip(1)
            .map(|v| (v as f32 / 10.0).to_string())
            .collect::<Vec<String>>();
        screen.combos[4]
            .values
            .append(&mut vec!["true".to_string(), "false".to_string()]);

        screen
    }

    pub fn display_defaults(&mut self) {
        self.combos.iter_mut().for_each(|c| c.display_default());
    }

    pub fn scale_point(&mut self, point: Point) {
        self.point = point.scale(SCALE);
    }

    pub fn next_inside_focus(&mut self) {
        if let Some(current_combo) = self.combos.iter_mut().find(|c| c.focused) {
            current_combo.next();
        }
    }
    pub fn previous_inside_focus(&mut self) {
        if let Some(current_combo) = self.combos.iter_mut().find(|c| c.focused) {
            current_combo.previous();
        }
    }

    /// Space reclamed in the frame.
    pub fn space_reclaimed(&self) -> (usize, usize) {
        if let Some((length, width)) = self.current_length_width(SCALE) {
            (
                self.point.row + max(width / 2, 20), // Add some if frame is smaller than the text
                self.point.col + max(length, 20),
            )
        } else {
            (0, 0)
        }
    }

    /// Get the (length, width) according to the combox value and the orientation.
    fn current_length_width(&self, scale: usize) -> Option<(usize, usize)> {
        let size = self.combos[0].current_value_to_usize();
        if size.len() >= 2 {
            if self.combos[2].current_value().contains('9')
                || self.combos[2].current_value().contains('2')
            {
                Some((size[1] / scale, size[0] / scale))
            } else {
                Some((size[0] / scale, size[1] / scale))
            }
        } else {
            None
        }
    }

    pub fn draw(&self, mut frame: Frame) -> Frame {
        if let Some((length, width)) = self.current_length_width(SCALE) {
            frame = if self.focused {
                frame.set_current_colors(style::Color::Blue, style::Color::Reset)
            } else {
                frame.set_current_colors(style::Color::White, style::Color::Reset)
            }
            .print_rectangle(self.point, length, width / 2)
            .print_text(self.number.to_string().as_str(), self.point.up(1, 2))
            .print_text(self.name.as_str(), self.point.up(2, 2));

            for comb in self.combos.iter() {
                frame = comb.draw(frame, self.focused, self.point);
            }
        }

        frame
    }
}
