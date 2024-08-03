pub mod confirm;
pub mod welcome;

pub enum Mode {
    Welcome,
    Confirm,
    Quit,
}

pub const HYPR_CONF: &str = ".config/hypr/hyprland.conf";
pub const HYPR_BAK: &str = ".config/hypr/.hyprland.bak";
