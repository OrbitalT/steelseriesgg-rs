//! GameSense event handlers and presets.

use super::*;
use crate::rgb::Color;

/// Create a simple static color handler.
pub fn static_color_handler(zone: &str, color: Color) -> Handler {
    Handler::Keyboard {
        zone: zone.to_string(),
        color: ColorHandler::Static {
            red: color.r,
            green: color.g,
            blue: color.b,
        },
        mode: None,
    }
}

/// Create a health bar handler (red at low, green at high).
pub fn health_bar_handler(zone: &str) -> Handler {
    Handler::Keyboard {
        zone: zone.to_string(),
        color: ColorHandler::Gradient {
            gradient: GradientSpec {
                zero: ColorSpec { red: 255, green: 0, blue: 0 },
                hundred: ColorSpec { red: 0, green: 255, blue: 0 },
            },
        },
        mode: None,
    }
}

/// Create an ammo counter handler (yellow at low, white at high).
pub fn ammo_handler(zone: &str) -> Handler {
    Handler::Keyboard {
        zone: zone.to_string(),
        color: ColorHandler::Gradient {
            gradient: GradientSpec {
                zero: ColorSpec { red: 255, green: 200, blue: 0 },
                hundred: ColorSpec { red: 255, green: 255, blue: 255 },
            },
        },
        mode: None,
    }
}

/// Create a cooldown handler with range colors.
pub fn cooldown_handler(zone: &str) -> Handler {
    Handler::Keyboard {
        zone: zone.to_string(),
        color: ColorHandler::Range {
            color: vec![
                RangeColor {
                    low: 0,
                    high: 25,
                    color: ColorSpec { red: 255, green: 0, blue: 0 },
                },
                RangeColor {
                    low: 26,
                    high: 50,
                    color: ColorSpec { red: 255, green: 128, blue: 0 },
                },
                RangeColor {
                    low: 51,
                    high: 75,
                    color: ColorSpec { red: 255, green: 255, blue: 0 },
                },
                RangeColor {
                    low: 76,
                    high: 100,
                    color: ColorSpec { red: 0, green: 255, blue: 0 },
                },
            ],
        },
        mode: None,
    }
}

/// Common zone names.
pub mod zones {
    pub const FUNCTION_KEYS: &str = "function-keys";
    pub const NUMBER_KEYS: &str = "number-keys";
    pub const MAIN_KEYBOARD: &str = "main-keyboard";
    pub const KEYPAD: &str = "keypad";
    pub const ARROW_KEYS: &str = "arrow-keys";
    pub const NAV_CLUSTER: &str = "nav-cluster";
    pub const ALL: &str = "all";
    pub const LOGO: &str = "logo";
    pub const WHEEL: &str = "wheel";
}

/// Preset event bindings for common games.
pub mod presets {
    use super::*;

    /// Create a generic FPS game preset.
    pub fn fps_preset(game_id: &str) -> Vec<EventBinding> {
        vec![
            EventBinding {
                game: game_id.to_string(),
                event: "HEALTH".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: Some(1),
                handlers: vec![health_bar_handler(zones::FUNCTION_KEYS)],
            },
            EventBinding {
                game: game_id.to_string(),
                event: "AMMO".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: Some(2),
                handlers: vec![ammo_handler(zones::NUMBER_KEYS)],
            },
        ]
    }

    /// Create a MOBA game preset.
    pub fn moba_preset(game_id: &str) -> Vec<EventBinding> {
        vec![
            EventBinding {
                game: game_id.to_string(),
                event: "HEALTH".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: Some(1),
                handlers: vec![health_bar_handler(zones::FUNCTION_KEYS)],
            },
            EventBinding {
                game: game_id.to_string(),
                event: "MANA".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: Some(3),
                handlers: vec![Handler::Keyboard {
                    zone: zones::NUMBER_KEYS.to_string(),
                    color: ColorHandler::Gradient {
                        gradient: GradientSpec {
                            zero: ColorSpec { red: 0, green: 0, blue: 100 },
                            hundred: ColorSpec { red: 0, green: 100, blue: 255 },
                        },
                    },
                    mode: None,
                }],
            },
            EventBinding {
                game: game_id.to_string(),
                event: "ABILITY_Q".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: None,
                handlers: vec![cooldown_handler("q")],
            },
            EventBinding {
                game: game_id.to_string(),
                event: "ABILITY_W".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: None,
                handlers: vec![cooldown_handler("w")],
            },
            EventBinding {
                game: game_id.to_string(),
                event: "ABILITY_E".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: None,
                handlers: vec![cooldown_handler("e")],
            },
            EventBinding {
                game: game_id.to_string(),
                event: "ABILITY_R".to_string(),
                min_value: 0,
                max_value: 100,
                icon_id: None,
                handlers: vec![cooldown_handler("r")],
            },
        ]
    }
}
