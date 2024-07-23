use super::Focus;
use crate::render::frame::{Frame, Point};
use crossterm::style;

#[derive(Debug, Default, Clone)]
pub struct WComboBox {
    point: Point,
    pub values: Vec<String>,
    pub current_displayed: usize,
    pub current_to_mark: usize,
    pub focused: bool,
}

impl Focus for WComboBox {
    fn is_focus(&self) -> bool {
        self.focused
    }
    fn set_focus(&mut self, value: bool) {
        self.focused = value;
    }
}

impl WComboBox {
    pub fn new(point: Point) -> WComboBox {
        Self {
            point,
            ..Default::default()
        }
    }

    pub fn next(&mut self) {
        self.current_displayed += 1;
        self.current_displayed %= self.values.len();
    }
    pub fn previous(&mut self) {
        match self.current_displayed {
            0 => self.current_displayed = self.values.len() - 1,
            _ => self.current_displayed -= 1,
        }
    }

    pub fn draw(&self, frame: Frame, is_owner_focused: bool, point: Point) -> Frame {
        let mut txt = format!(
            "❱ {} ",
            self.values
                .get(self.current_displayed)
                .cloned()
                .unwrap_or("fail".to_string())
        );
        if self.current_displayed == self.current_to_mark {
            txt.push_str("* ")
        }

        let (fore, back) = match (is_owner_focused, self.focused) {
            (true, true) => (style::Color::Black, style::Color::Blue),
            (false, true) => (style::Color::Black, style::Color::Grey),
            _ => (style::Color::White, style::Color::Reset),
        };

        frame
            .set_current_colors(fore, back)
            .print_text(txt.as_str(), self.point.up(point.row, point.col))
    }
}
