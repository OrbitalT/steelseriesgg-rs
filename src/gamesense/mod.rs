//! GameSense HTTP server for game integration.
//!
//! Implements a compatible GameSense API that games can connect to
//! for reactive RGB lighting and device feedback.

mod handlers;
mod server;

pub use handlers::*;
pub use server::GameSenseServer;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Game registration request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameMetadata {
    /// Game identifier (uppercase, no spaces).
    pub game: String,

    /// Display name for the game.
    pub game_display_name: String,

    /// Developer/publisher name.
    pub developer: String,

    /// Optional deinitialization timeout in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deinitialize_timer_length_ms: Option<u32>,
}

/// Game event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEvent {
    /// Game identifier.
    pub game: String,

    /// Event name.
    pub event: String,

    /// Event data.
    pub data: EventData,
}

/// Event data payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventData {
    /// Numeric value (0-100 typically).
    pub value: i32,

    /// Optional frame data for complex events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame: Option<HashMap<String, serde_json::Value>>,
}

/// Event handler binding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBinding {
    /// Game identifier.
    pub game: String,

    /// Event name.
    pub event: String,

    /// Minimum value.
    #[serde(default)]
    pub min_value: i32,

    /// Maximum value.
    #[serde(default = "default_max_value")]
    pub max_value: i32,

    /// Icon ID (0-40).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<u8>,

    /// Handlers for this event.
    pub handlers: Vec<Handler>,
}

fn default_max_value() -> i32 {
    100
}

/// Event handler definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "device-type")]
pub enum Handler {
    /// RGB zone handler.
    #[serde(rename = "rgb-per-key-zones")]
    RgbPerKeyZones {
        zone: String,
        color: ColorHandler,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<String>,
    },

    /// Keyboard handler.
    #[serde(rename = "keyboard")]
    Keyboard {
        zone: String,
        color: ColorHandler,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<String>,
    },

    /// Screen handler (OLED).
    #[serde(rename = "screened")]
    Screen {
        zone: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        datas: Option<Vec<ScreenData>>,
    },

    /// Tactile handler (vibration).
    #[serde(rename = "tactile")]
    Tactile {
        zone: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        mode: Option<String>,
    },
}

/// Color handler definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorHandler {
    /// Static color.
    Static { red: u8, green: u8, blue: u8 },

    /// Gradient between two colors.
    Gradient { gradient: GradientSpec },

    /// Range-based color selection.
    Range { color: Vec<RangeColor> },
}

/// Gradient specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradientSpec {
    pub zero: ColorSpec,
    pub hundred: ColorSpec,
}

/// Color specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSpec {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

/// Range-based color.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeColor {
    pub low: i32,
    pub high: i32,
    pub color: ColorSpec,
}

/// Screen data for OLED display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenData {
    /// Line number (0-indexed).
    pub line: u8,

    /// Text to display.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// Icon ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_id: Option<u8>,

    /// Whether to show progress bar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_progress_bar: Option<bool>,
}

/// Heartbeat request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    /// Game identifier.
    pub game: String,
}

/// Remove game request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveGame {
    /// Game identifier.
    pub game: String,
}

/// Remove event request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveEvent {
    /// Game identifier.
    pub game: String,

    /// Event name.
    pub event: String,
}

/// API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse {
    /// Whether the request succeeded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,

    /// Error code if failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// Error details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_text: Option<String>,
}

impl ApiResponse {
    /// Create a success response.
    pub fn success() -> Self {
        Self {
            success: Some(true),
            error: None,
            error_text: None,
        }
    }

    /// Create an error response.
    pub fn error(code: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            success: Some(false),
            error: Some(code.into()),
            error_text: Some(text.into()),
        }
    }
}
