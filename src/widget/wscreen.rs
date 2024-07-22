use super::{wcombobox::WComboBox, Focus};
use crate::render::frame::{Frame, Point};
use crossterm::style;

#[derive(Debug, Default, Clone)]
pub struct WScreen {
    pub number: usize,
    pub name: String,

    point: Point,
    pub focused: bool,
    pub combos: Vec<WComboBox>,
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
    pub fn new(number: usize, name: String, point: Point, focused: bool) -> WScreen {
        let mut screen = Self {
            number,
            name,
            point,
            focused,
            combos: vec![
                WComboBox::new(point.up(4, 2)),
                WComboBox::new(point.up(5, 2)),
            ],
        };

        // Init --
        screen.combos[0].focused = true;
        screen.combos[1].values.append(&mut vec![
            "0 °".to_string(),
            "90 °".to_string(),
            "180 °".to_string(),
            "360 °".to_string(),
        ]);

        screen
    }

    pub fn next(&mut self) {
        if let Some(current_combo) = self.combos.iter_mut().find(|c| c.focused) {
            current_combo.next();
        }
    }
    pub fn previous(&mut self) {
        if let Some(current_combo) = self.combos.iter_mut().find(|c| c.focused) {
            current_combo.previous();
        }
    }

    pub fn draw(&self, mut frame: Frame) -> Frame {
        for c in self.combos.iter() {
            frame = c.draw(frame, self.focused);
        }

        if self.focused {
            frame.set_current_colors(style::Color::Blue, style::Color::Black)
        } else {
            frame.set_current_colors(style::Color::White, style::Color::Black)
        }
        .print_square(self.point, 8, 19)
        .print_text(self.number.to_string().as_str(), self.point.up(1, 2))
        .print_text(self.name.as_str(), self.point.up(2, 2))
    }
}
