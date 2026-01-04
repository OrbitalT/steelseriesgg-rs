//! Mouse device support (Rival series).

use hidapi::HidDevice;
use crate::{Error, Result};
use crate::rgb::Color;
use super::{Device, DeviceInfo, DeviceType};

/// Trait for mouse-specific functionality.
pub trait Mouse: Device {
    /// Set the mouse LED color.
    fn set_color(&mut self, color: Color) -> Result<()>;

    /// Set DPI for a specific profile slot.
    fn set_dpi(&mut self, slot: u8, dpi: u16) -> Result<()>;

    /// Get current DPI.
    fn get_dpi(&self) -> Option<u16>;

    /// Set polling rate in Hz (125, 250, 500, 1000).
    fn set_polling_rate(&mut self, rate: u16) -> Result<()>;
}

/// Generic SteelSeries mouse implementation.
pub struct GenericMouse {
    info: DeviceInfo,
    device: Option<HidDevice>,
    current_dpi: Option<u16>,
}

impl GenericMouse {
    /// Create a new mouse instance.
    pub fn new(info: DeviceInfo, device: HidDevice) -> Self {
        Self {
            info,
            device: Some(device),
            current_dpi: None,
        }
    }

    /// Send a HID report to the mouse.
    fn send_report(&mut self, data: &[u8]) -> Result<()> {
        let device = self.device.as_mut().ok_or(Error::DeviceCommunication(
            "Device not connected".to_string(),
        ))?;

        // Pad to 64 bytes
        let mut report = vec![0u8; 64];
        report[..data.len().min(64)].copy_from_slice(&data[..data.len().min(64)]);

        device.write(&report)?;
        Ok(())
    }
}

impl Device for GenericMouse {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Mouse
    }

    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        self.device = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.device.is_some()
    }

    fn send_raw(&mut self, data: &[u8]) -> Result<()> {
        self.send_report(data)
    }

    fn receive_raw(&mut self, buf: &mut [u8]) -> Result<usize> {
        let device = self.device.as_mut().ok_or(Error::DeviceCommunication(
            "Device not connected".to_string(),
        ))?;

        let len = device.read(buf)?;
        Ok(len)
    }
}

impl Mouse for GenericMouse {
    fn set_color(&mut self, color: Color) -> Result<()> {
        // Rival color format: [0x05, 0x00, R, G, B]
        let data = [0x05, 0x00, color.r, color.g, color.b];
        self.send_report(&data)
    }

    fn set_dpi(&mut self, slot: u8, dpi: u16) -> Result<()> {
        // Clamp DPI to valid range (100-12000 typically)
        let dpi = dpi.clamp(100, 12000);
        let slot = slot.min(4); // Most mice have 5 DPI slots (0-4)

        // DPI command format varies by model
        // Generic: [0x03, slot, dpi_low, dpi_high]
        let data = [
            0x03,
            slot,
            (dpi & 0xFF) as u8,
            ((dpi >> 8) & 0xFF) as u8,
        ];

        self.send_report(&data)?;
        self.current_dpi = Some(dpi);
        Ok(())
    }

    fn get_dpi(&self) -> Option<u16> {
        self.current_dpi
    }

    fn set_polling_rate(&mut self, rate: u16) -> Result<()> {
        // Valid rates: 125, 250, 500, 1000 Hz
        let rate_code = match rate {
            125 => 0x04,
            250 => 0x03,
            500 => 0x02,
            1000 => 0x01,
            _ => return Err(Error::InvalidConfig("Invalid polling rate".to_string())),
        };

        let data = [0x04, rate_code];
        self.send_report(&data)
    }
}

/// Supported DPI values for Rival mice.
pub const RIVAL_DPI_VALUES: &[u16] = &[
    400, 800, 1200, 1600, 2000, 2400, 3200, 4000, 5000, 6000, 8000, 10000, 12000,
];
