//! Device state persistence for tracking current device settings.
//!
//! Since SteelSeries devices generally don't expose complete "read back current settings"
//! functionality, this module maintains a store of the last-applied settings keyed by
//! device identity. We prefer stable identifiers (path/interface) when serial numbers
//! are missing to avoid collisions across multiple identical devices.

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
    #[serde(default)]
    pub interface_number: i32,
    pub serial_number: Option<String>,
    /// HID path (if available); optional for backward compatibility with older state files.
    #[serde(default)]
    pub path: Option<String>,
}

fn effects_equal(a: &Effect, b: &Effect) -> bool {
    use Effect::*;

    match (a, b) {
        (Static { color: c1 }, Static { color: c2 }) => c1 == c2,
        (
            Breathing {
                color: c1,
                speed: s1,
            },
            Breathing {
                color: c2,
                speed: s2,
            },
        ) => c1 == c2 && s1 == s2,
        (Spectrum { speed: s1 }, Spectrum { speed: s2 }) => s1 == s2,
        (
            Wave {
                colors: c1,
                speed: s1,
                direction: d1,
            },
            Wave {
                colors: c2,
                speed: s2,
                direction: d2,
            },
        ) => c1 == c2 && s1 == s2 && d1 == d2,
        (
            Reactive {
                color: c1,
                duration: d1,
            },
            Reactive {
                color: c2,
                duration: d2,
            },
        ) => c1 == c2 && d1 == d2,
        (Gradient { start: s1, end: e1 }, Gradient { start: s2, end: e2 }) => s1 == s2 && e1 == e2,
        (Custom { colors: c1 }, Custom { colors: c2 }) => c1 == c2,
        (Off, Off) => true,
        _ => false,
    }
}

fn keyboard_states_equal(a: &KeyboardState, b: &KeyboardState) -> bool {
    a.brightness == b.brightness && effects_equal(&a.effect, &b.effect)
}

fn headset_states_equal(a: &HeadsetState, b: &HeadsetState) -> bool {
    a.sidetone == b.sidetone
        && a.mic_volume == b.mic_volume
        && a.mic_muted == b.mic_muted
        && a.eq_preset == b.eq_preset
        && a.auto_off_minutes == b.auto_off_minutes
}

impl From<&DeviceInfo> for DeviceId {
    fn from(info: &DeviceInfo) -> Self {
        Self {
            vendor_id: info.vendor_id,
            product_id: info.product_id,
            interface_number: info.interface_number,
            serial_number: info.serial_number.clone(),
            path: Some(info.path.clone()),
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
            .ok_or_else(|| {
                Error::InvalidConfig("could not determine config directory".to_string())
            })?
            .join("device_state.json");

        if let Some(parent) = state_file.parent() {
            std::fs::create_dir_all(parent)?;
        }

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
        if let Some(state) = self.states.get(id) {
            return Some(state);
        }

        self.states
            .iter()
            .find(|(existing, _)| Self::id_loosely_matches(existing, id))
            .map(|(_, state)| state)
    }

    /// Get mutable reference to device state.
    pub fn get_mut(&mut self, id: &DeviceId) -> Option<&mut DeviceState> {
        if self.states.contains_key(id) {
            return self.states.get_mut(id);
        }

        let key = self
            .states
            .keys()
            .find(|existing| Self::id_loosely_matches(existing, id))
            .cloned();

        if let Some(key) = key {
            return self.states.get_mut(&key);
        }

        None
    }

    /// Get or create device state.
    pub fn get_or_create(&mut self, id: DeviceId) -> &mut DeviceState {
        // Prefer an exact match, but fall back to legacy identifiers (pre-path) to
        // retain previously saved state.
        if let Some(existing_key) = self
            .states
            .keys()
            .find(|existing| Self::id_loosely_matches(existing, &id))
            .cloned()
        {
            return self
                .states
                .get_mut(&existing_key)
                .expect("key just found exists");
        }

        self.states.entry(id).or_insert_with(DeviceState::default)
    }

    /// Update keyboard state for a device.
    pub fn update_keyboard(&mut self, id: DeviceId, keyboard: KeyboardState) -> Result<()> {
        let state = self.get_or_create(id);
        if state
            .keyboard
            .as_ref()
            .map(|existing| keyboard_states_equal(existing, &keyboard))
            .unwrap_or(false)
        {
            return Ok(());
        }

        state.keyboard = Some(keyboard);
        self.save()
    }

    /// Update headset state for a device.
    pub fn update_headset(&mut self, id: DeviceId, headset: HeadsetState) -> Result<()> {
        let state = self.get_or_create(id);
        if state
            .headset
            .as_ref()
            .map(|existing| headset_states_equal(existing, &headset))
            .unwrap_or(false)
        {
            return Ok(());
        }

        state.headset = Some(headset);
        self.save()
    }

    /// Update keyboard effect for a device.
    pub fn update_keyboard_effect(&mut self, id: DeviceId, effect: Effect) -> Result<()> {
        let state = self.get_or_create(id);
        if let Some(ref mut keyboard) = state.keyboard {
            if effects_equal(&keyboard.effect, &effect) {
                return Ok(());
            }
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
            if keyboard.brightness == brightness {
                return Ok(());
            }
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

    /// Compare device identifiers while tolerating missing path information to keep
    /// backward compatibility with previously persisted state.
    fn id_loosely_matches(existing: &DeviceId, candidate: &DeviceId) -> bool {
        let interface_matches = existing.interface_number == 0
            || candidate.interface_number == 0
            || existing.interface_number == candidate.interface_number;

        existing.vendor_id == candidate.vendor_id
            && existing.product_id == candidate.product_id
            && interface_matches
            && existing.serial_number == candidate.serial_number
    }
}
