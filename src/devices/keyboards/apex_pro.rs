use super::{GenericKeyboard, Keyboard};
use crate::Result;
use crate::devices::key_mapping::{KeyAddress, KeyId, KeyMapping};
use crate::devices::zone_mapping::{ZoneEffect, ZoneMapping};
use crate::devices::{Device, DeviceInfo, DeviceType};
use crate::rgb::{Color, PerKeyEffect};
use async_trait::async_trait;

/// Apex Pro Gen 3 specific implementation.
pub struct ApexProGen3 {
    inner: GenericKeyboard,
}

impl ApexProGen3 {
    /// Product ID for Apex Pro Gen 3.
    pub const PRODUCT_ID: u16 = 0x1640;

    /// Create from a generic keyboard.
    pub fn new(keyboard: GenericKeyboard) -> Self {
        Self { inner: keyboard }
    }

    /// Streaming Initialization for real-time matrix animation updates.
    pub fn stream_initialization(&mut self) -> Result<()> {
        let handshake = [0x09, 0x00, 0x03, 0x01, 0x00, 0x82, 0x02];
        self.send_raw(&handshake)
    }

    /// Stream Matrix Buffer for the 650-byte dynamic RGB canvas matrix.
    /// The daemon's internal key grid states will be transformed into this payload.
    /// Transmits using 678-byte packets containing a 649-byte payload.
    pub fn stream_matrix_buffer(&mut self, payload: &[u8; 650]) -> Result<()> {
        let mut data = vec![0u8; 678];
        // Ensure this targets Interface 0 (handled by `open_device_with_interface` during connection)
        data[0] = 0x09; // Assuming Report ID or SET_REPORT header
        // For now just copy the 650 byte payload into the packet
        data[1..651].copy_from_slice(payload);
        self.send_raw(&data)
    }

    /// Configure Dynamic Rapid Trigger Logic
    pub fn set_rapid_trigger(&mut self, key_index: u8, enable: bool, activation: u8, deactivation: u8) -> Result<()> {
        // Subtype 2b for actuation / rapid trigger
        let mut data = vec![0u8; 32]; // standard buffer
        data[0] = 0x06;
        data[1] = 0x2b;
        data[2] = key_index;
        
        let flags = if enable { 0x03 } else { 0x01 };
        data[3] = flags;
        
        // Boundaries: 0x01 (0.1mm) to 0x28 (4.0mm)
        data[4] = activation.clamp(1, 40);
        data[5] = deactivation.clamp(1, 40);

        self.send_raw(&data)
    }
}

// Delegate Device trait
impl Device for ApexProGen3 {
    fn info(&self) -> &DeviceInfo {
        self.inner.info()
    }

    fn device_type(&self) -> DeviceType {
        self.inner.device_type()
    }

    fn initialize(&mut self) -> Result<()> {
        self.inner.initialize()
    }

    fn close(&mut self) -> Result<()> {
        self.inner.close()
    }

    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    fn send_raw(&mut self, data: &[u8]) -> Result<()> {
        self.inner.send_raw(data)
    }

    fn receive_raw(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.inner.receive_raw(buf)
    }
}

// Delegate Keyboard trait
crate::impl_keyboard_with_delegation!(ApexProGen3, {
    async fn set_color(&mut self, color: Color) -> Result<()> {
        self.inner.set_color(color).await
    }

    async fn set_zone_colors(&mut self, colors: &[Color]) -> Result<()> {
        self.inner.set_zone_colors(colors).await
    }

    async fn set_key_color(&mut self, key_id: KeyId, color: Color) -> Result<()> {
        self.inner.set_key_color(key_id, color).await
    }

    async fn set_key_colors(&mut self, key_colors: &[(KeyId, Color)]) -> Result<()> {
        self.inner.set_key_colors(key_colors).await
    }

    fn read_actuation_point(&mut self) -> Result<u8> {
        self.inner.read_actuation_point()
    }

    fn set_actuation_point(&mut self, value: u8) -> Result<()> {
        let value = value.clamp(1, 40);
        // Generic actuation fallback or specific
        self.inner.set_actuation_point(value)
    }

    fn set_actuation_point_mm(&mut self, mm: f32) -> Result<()> {
        let val = (mm * 10.0).round() as u8;
        self.set_actuation_point(val)
    }
});
