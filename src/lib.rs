//! # SteelSeries GG for Linux
//!
//! A complete open-source replacement for SteelSeries GG on Linux.
//!
//! ## Features
//!
//! - **Device Control**: RGB lighting, DPI settings, and more for SteelSeries devices
//! - **Audio Mixer**: Multi-channel audio mixing (Game, Chat, Media, Aux, Mic)
//! - **GameSense**: HTTP API server for game integration and reactive lighting
//! - **Profiles**: Save and load device configurations
//!
//! ## Supported Devices
//!
//! - Keyboards: Apex Pro, Apex 3 TKL, and more
//! - Mice: Rival 3, Rival 5, Aerox series
//! - Headsets: Arctis Nova Pro, Arctis 7/9, and more

pub mod config;
pub mod devices;
pub mod error;
pub mod gamesense;
pub mod profiles;
pub mod rgb;

#[cfg(feature = "audio")]
pub mod audio;

pub use error::{Error, Result};

/// SteelSeries USB Vendor ID
pub const STEELSERIES_VENDOR_ID: u16 = 0x1038;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::devices::{Device, DeviceInfo, DeviceManager, DeviceType};
    pub use crate::error::{Error, Result};
    pub use crate::rgb::{Color, Effect, RgbController};
}
