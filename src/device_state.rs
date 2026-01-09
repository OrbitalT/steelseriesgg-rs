//! Device state persistence for tracking current device settings.
//!
//! Since SteelSeries devices generally don't expose complete "read back current settings"
//! functionality, this module maintains a store of the last-applied settings keyed by
//! device identity (vendor_id, product_id, serial_number).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;
use crate::devices::DeviceInfo;
use crate::rgb::Effect;
use crate::{Error, Result};

/// Unique identifier for a device.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct DeviceId {
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: Option<String>,
}

impl From<&DeviceInfo> for DeviceId {
    fn from(info: &DeviceInfo) -> Self {
        Self {
            vendor_id: info.vendor_id,
            product_id: info.product_id,
            serial_number: info.serial_number.clone(),
        }
    }
}

/// Keyboard device state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct KeyboardState {
    /// Current RGB effect.
    pub effect: Effect,
    /// Current brightness (0-100).
    pub brightness: u8,
}

impl Default for KeyboardState {
    fn default() -> Self {
        Self {
            effect: Effect::Static {
                color: crate::rgb::Color::WHITE,
            },
            brightness: 100,
        }
    }
}

/// Headset device state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HeadsetState {
    /// Sidetone level (0-100).
    pub sidetone: u8,
    /// Microphone volume (0-100).
    pub mic_volume: u8,
    /// Microphone muted.
    pub mic_muted: bool,
    /// EQ preset name.
    pub eq_preset: String,
    /// Auto-off timeout in minutes.
    pub auto_off_minutes: u8,
}

impl Default for HeadsetState {
    fn default() -> Self {
        Self {
            sidetone: 50,
            mic_volume: 100,
            mic_muted: false,
            eq_preset: "Flat".to_string(),
            auto_off_minutes: 15,
        }
    }
}

/// Aggregate device state.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeviceState {
    pub keyboard: Option<KeyboardState>,
    pub headset: Option<HeadsetState>,
}

impl Default for DeviceState {
    fn default() -> Self {
        Self {
            keyboard: None,
            headset: None,
        }
    }
}

/// Manager for device state persistence.
pub struct DeviceStateStore {
    states: HashMap<DeviceId, DeviceState>,
    state_file: PathBuf,
}

impl DeviceStateStore {
    /// Create a new device state store.
    pub fn new() -> Result<Self> {
        let state_file = Config::config_dir()
            .ok_or_else(|| Error::InvalidConfig("could not determine config directory".to_string()))?
            .join("device_state.json");

        let mut store = Self {
            states: HashMap::new(),
            state_file,
        };

        // Load existing state if available
        if store.state_file.exists() {
            store.load()?;
        }

        Ok(store)
    }

    /// Load device states from disk.
    fn load(&mut self) -> Result<()> {
        let content = std::fs::read_to_string(&self.state_file)?;
        self.states = serde_json::from_str(&content)?;
        Ok(())
    }

    /// Save device states to disk.
    fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.states)?;
        std::fs::write(&self.state_file, content)?;
        Ok(())
    }

    /// Get the state for a device.
    pub fn get(&self, id: &DeviceId) -> Option<&DeviceState> {
        self.states.get(id)
    }

    /// Get mutable reference to device state.
    pub fn get_mut(&mut self, id: &DeviceId) -> Option<&mut DeviceState> {
        self.states.get_mut(id)
    }

    /// Get or create device state.
    pub fn get_or_create(&mut self, id: DeviceId) -> &mut DeviceState {
        self.states.entry(id).or_insert_with(DeviceState::default)
    }

    /// Update keyboard state for a device.
    pub fn update_keyboard(&mut self, id: DeviceId, keyboard: KeyboardState) -> Result<()> {
        let state = self.get_or_create(id);
        state.keyboard = Some(keyboard);
        self.save()
    }

    /// Update headset state for a device.
    pub fn update_headset(&mut self, id: DeviceId, headset: HeadsetState) -> Result<()> {
        let state = self.get_or_create(id);
        state.headset = Some(headset);
        self.save()
    }

    /// Update keyboard effect for a device.
    pub fn update_keyboard_effect(&mut self, id: DeviceId, effect: Effect) -> Result<()> {
        let state = self.get_or_create(id);
        if let Some(ref mut keyboard) = state.keyboard {
            keyboard.effect = effect;
        } else {
            state.keyboard = Some(KeyboardState {
                effect,
                ..Default::default()
            });
        }
        self.save()
    }

    /// Update keyboard brightness for a device.
    pub fn update_keyboard_brightness(&mut self, id: DeviceId, brightness: u8) -> Result<()> {
        let state = self.get_or_create(id);
        if let Some(ref mut keyboard) = state.keyboard {
            keyboard.brightness = brightness;
        } else {
            state.keyboard = Some(KeyboardState {
                brightness,
                ..Default::default()
            });
        }
        self.save()
    }

    /// List all devices with stored state.
    pub fn list_devices(&self) -> Vec<&DeviceId> {
        self.states.keys().collect()
    }
}
