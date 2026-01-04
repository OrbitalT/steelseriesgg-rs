//! Keyboard device support (Apex series).

pub mod apex;

use hidapi::HidDevice;
use crate::{Error, Result};
use crate::rgb::Color;
use super::{Device, DeviceInfo, DeviceType};

/// Trait for keyboard-specific functionality.
pub trait Keyboard: Device {
    /// Set the entire keyboard to a single color.
    fn set_color(&mut self, color: Color) -> Result<()>;

    /// Set colors for individual zones.
    fn set_zone_colors(&mut self, colors: &[Color]) -> Result<()>;

    /// Get the number of RGB zones.
    fn zone_count(&self) -> usize;

    /// Set keyboard brightness (0-100).
    fn set_brightness(&mut self, brightness: u8) -> Result<()>;

    /// Apply the current RGB settings.
    fn apply(&mut self) -> Result<()>;
}

/// Generic SteelSeries keyboard implementation.
pub struct GenericKeyboard {
    info: DeviceInfo,
    device: Option<HidDevice>,
    zone_count: usize,
}

impl GenericKeyboard {
    /// Create a new keyboard instance.
    pub fn new(info: DeviceInfo, device: HidDevice) -> Self {
        // Determine zone count based on product ID
        let zone_count = match info.product_id {
            0x1622 => 9,  // Apex 3 TKL - 10 zones
            0x161A => 10, // Apex 3 - 10 zones
            _ => 1,       // Default single zone
        };

        Self {
            info,
            device: Some(device),
            zone_count,
        }
    }

    /// Send a HID report to the keyboard.
    fn send_report(&mut self, data: &[u8]) -> Result<()> {
        let device = self.device.as_mut().ok_or(Error::DeviceCommunication(
            "Device not connected".to_string(),
        ))?;

        // Pad to 64 bytes with leading 0x00 for report ID
        let mut report = vec![0u8; 65];
        report[1..1 + data.len().min(64)].copy_from_slice(&data[..data.len().min(64)]);

        device.write(&report)?;
        Ok(())
    }
}

impl Device for GenericKeyboard {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Keyboard
    }

    fn initialize(&mut self) -> Result<()> {
        // Send initialization sequence if needed
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

impl Keyboard for GenericKeyboard {
    fn set_color(&mut self, color: Color) -> Result<()> {
        // Create color data for all zones
        let colors = vec![color; self.zone_count];
        self.set_zone_colors(&colors)
    }

    fn set_zone_colors(&mut self, colors: &[Color]) -> Result<()> {
        // Build the RGB packet
        // Format: [0x21, 0xFF, R, G, B, R, G, B, ...] for each zone
        let mut data = vec![0x21, 0xFF];

        for color in colors.iter().take(self.zone_count) {
            data.push(color.r);
            data.push(color.g);
            data.push(color.b);
        }

        // Pad remaining zones with zeros
        for _ in colors.len()..self.zone_count {
            data.extend_from_slice(&[0, 0, 0]);
        }

        self.send_report(&data)
    }

    fn zone_count(&self) -> usize {
        self.zone_count
    }

    fn set_brightness(&mut self, brightness: u8) -> Result<()> {
        // Clamp brightness to 0-100
        let brightness = brightness.min(100);

        // Send brightness command
        let data = [0x22, brightness];
        self.send_report(&data)
    }

    fn apply(&mut self) -> Result<()> {
        // Some keyboards need an explicit apply command
        Ok(())
    }
}
