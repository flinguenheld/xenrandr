pub mod confirm;
pub mod message;
pub mod welcome;

pub enum Mode {
    Welcome,
    Confirm,
    Message(String),
    Quit,
}

pub const HYPR_CONF: &str = ".config/hypr/hyprland.confsss";
pub const HYPR_BAK: &str = ".config/hypr/.hyprland.bak";
