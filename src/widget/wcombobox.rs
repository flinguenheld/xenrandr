use super::Focus;
use crate::render::frame::{Frame, Point};
use crossterm::style;

#[derive(Debug, Default, Clone)]
pub struct WComboBox {
    pub current_displayed: usize,
    pub default: String, // Value to display with a star
    pub focused: bool,
    point: Point,
    prefix: String,
    suffix: String,
    pub values: Vec<String>,
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
    pub fn new(point: Point, prefix: &str, suffix: &str) -> WComboBox {
        Self {
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
            point,
            ..Default::default()
        }
    }

    pub fn display_default(&mut self) {
        self.current_displayed = self
            .values
            .iter()
            .position(|v| *v == self.default)
            .unwrap_or(0);
    }

    pub fn current_value(&self) -> String {
        self.values
            .get(self.current_displayed)
            .unwrap_or(&String::new())
            .clone()
    }
    pub fn current_value_to_usize(&self) -> Vec<usize> {
        let mut numbers: Vec<usize> = Vec::new();
        for txt in self.current_value().split(|c: char| !c.is_numeric()) {
            if let Ok(num) = txt.parse::<usize>() {
                numbers.push(num);
            }
        }
        numbers
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
        let val = self
            .values
            .get(self.current_displayed)
            .cloned()
            .unwrap_or("fail".to_string());

        let mut txt = format!("{} ❱ {} {}", self.prefix, val, self.suffix);
        // dbg!(&self.default);
        if val == self.default {
            txt.push_str(" *")
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
