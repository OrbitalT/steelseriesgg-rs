use super::{GenericSpeaker, Speaker};
use crate::Result;
use crate::devices::{Device, DeviceInfo, DeviceType};

/// Arena 7 specific implementation.
pub struct Arena7 {
    inner: GenericSpeaker,
}

impl Arena7 {
    /// Product ID for Arena 7.
    pub const PRODUCT_ID: u16 = 0x1a00;

    /// Create from a generic speaker.
    pub fn new(speaker: GenericSpeaker) -> Self {
        Self { inner: speaker }
    }
}

// Delegate Device trait
impl Device for Arena7 {
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

// Implement Speaker trait
#[async_trait::async_trait]
impl Speaker for Arena7 {
    fn set_static_color(&mut self, colors: &[[u8; 3]; 4]) -> Result<()> {
        let mut data = vec![0u8; 64];
        data[0] = 0x06; // Report ID
        data[1] = 0xa1; // Subtype a1

        // Zone Layout: LT, LB, RT, RB
        let mut offset = 2;
        for color in colors {
            data[offset] = color[0];     // Red
            data[offset + 1] = color[1]; // Green
            data[offset + 2] = color[2]; // Blue
            data[offset + 3] = 0x01;     // Effect (0x01)
            data[offset + 4] = 0x1e;     // Constant (0x1e)
            data[offset + 5] = 0x0a;     // Brightness (0x0a = max)
            // No trailing pad needed here, we just advance by 6
            offset += 6;
        }

        // Terminal footer
        data[26] = 0x0f;

        // Note: the rest of the buffer is 0x00 because we created it with vec![0u8; 64]

        self.inner.send_feature_report(&data)
    }

    fn set_dynamic_colorshift(&mut self, colors: &[[u8; 3]; 4]) -> Result<()> {
        let mut data = vec![0u8; 64];
        data[0] = 0x06; // Report ID
        data[1] = 0xa7; // Subtype a7
        data[2] = 0x0f; // Subtype a7 0f header

        let mut offset = 3;
        for color in colors {
            data[offset] = color[0];
            data[offset + 1] = color[1];
            data[offset + 2] = color[2];
            offset += 3;
        }

        // The rest are zeroed trailing bytes (handled by vec! initialization)

        self.inner.send_feature_report(&data)
    }

    fn set_equalizer(&mut self, bands: &[u8; 10]) -> Result<()> {
        let mut data = vec![0u8; 12];
        data[0] = 0x06; // Report ID
        data[1] = 0x33; // Subtype 33

        for i in 0..10 {
            data[2 + i] = bands[i];
        }

        self.inner.send_feature_report(&data)
    }
}
