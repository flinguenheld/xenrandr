use super::{wcombobox::WComboBox, Focus};
use crate::render::frame::{Frame, Point};
use crossterm::style;
use std::cmp::max;

const SCALE: usize = 100;

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
    pub fn new(number: usize, name: String, focused: bool, point: Point) -> WScreen {
        let mut screen = Self {
            number,
            name,
            focused,
            point,

            combos: vec![
                WComboBox::new(Point::new(4, 2)),
                WComboBox::new(Point::new(5, 2)),
            ],
        };

        // Init --
        screen.combos[0].focused = true;
        screen.combos[1].values.append(&mut vec![
            "0 °".to_string(),
            "90 °".to_string(),
            "180 °".to_string(),
            "270 °".to_string(),
        ]);

        screen
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
        let (length, width) = self.current_length_width(SCALE);
        (
            self.point.row + max(width / 2, 20), // Add some if frame is smaller than the text
            self.point.col + max(length, 10),
        )
    }

    pub fn draw(&self, mut frame: Frame) -> Frame {
        let (length, width) = self.current_length_width(SCALE);

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
        frame
    }

    /// Get the (length, width) according to the combox value and the orientation.
    fn current_length_width(&self, scale: usize) -> (usize, usize) {
        let size = self.combos[0].current_value_to_usize();

        if self.combos[1].current_value().contains('9')
            || self.combos[1].current_value().contains('2')
        {
            (size[1] / scale, size[0] / scale)
        } else {
            (size[0] / scale, size[1] / scale)
        }
    }
}
