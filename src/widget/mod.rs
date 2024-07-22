mod wcombobox;
mod wscreen;

pub use wscreen::WScreen;

pub trait Focus {
    fn is_focus(&self) -> bool;
    fn set_focus(&mut self, value: bool);
}

pub fn focus_next(widget: &mut Vec<impl Focus>) {
    if let Some(pos) = widget.iter().position(|ws| ws.is_focus()) {
        let len = widget.len();
        widget[pos].set_focus(false);
        widget[(pos + 1) % len].set_focus(true);
    }
}
pub fn focus_previous(widget: &mut Vec<impl Focus>) {
    if let Some(pos) = widget.iter().position(|ws| ws.is_focus()) {
        let len = widget.len();
        widget[pos].set_focus(false);
        match pos == 0 {
            true => widget[len - 1].set_focus(true),
            false => widget[pos - 1].set_focus(true),
        }
    }
}
