use crate::{
    render::frame::{Frame, Point},
    xrandr::XScreen,
};

#[derive(Debug, Default)]
pub struct WScreen {
    point: Point,
}

impl WScreen {
    pub fn new(point: Point) -> WScreen {
        Self { point }
    }

    pub fn draw(&self, frame: Frame, xscreen: &XScreen) -> Frame {
        frame
            .print_square(self.point, 8, 15)
            .print_text(xscreen.number.to_string().as_str(), self.point.up(1, 2))
            .print_text(xscreen.name.as_str(), self.point.up(2, 2))
            .print_text(xscreen.current_res().unwrap().as_str(), self.point.up(3, 2))
    }
}
