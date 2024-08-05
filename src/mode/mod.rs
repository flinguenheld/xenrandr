pub mod confirm;
pub mod message;
pub mod welcome;

pub enum Mode {
    Welcome(bool),
    Confirm(bool),
    Message(String),
    Quit,
}

pub const HYPR_CONF: &str = ".config/hypr/hyprland.conf";
pub const HYPR_BAK: &str = ".config/hypr/.hyprland.bak";
