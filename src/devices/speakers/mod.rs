//! Speaker device definitions.

pub mod arena_7;

use super::{Device, DeviceInfo, DeviceType};
use crate::Result;
use async_trait::async_trait;

/// Base trait for all Speaker devices.
#[async_trait]
pub trait Speaker: Device {
    /// Set static single-color mode for zones.
    /// Zone Layout: Left Top (LT), Left Bottom (LB), Right Top (RT), and Right Bottom (RB).
    fn set_static_color(&mut self, colors: &[[u8; 3]; 4]) -> Result<()>;

    /// Set dynamic Prism ColorShift streaming mode.
    fn set_dynamic_colorshift(&mut self, colors: &[[u8; 3]; 4]) -> Result<()>;

    /// Set the equalizer bands (32Hz to 16kHz).
    fn set_equalizer(&mut self, bands: &[u8; 10]) -> Result<()>;
}

use std::sync::Arc;
use parking_lot::Mutex;

/// Generic speaker wrapper.
pub struct GenericSpeaker {
    info: DeviceInfo,
    device: Arc<Mutex<hidapi::HidDevice>>,
}

impl GenericSpeaker {
    /// Create a new generic speaker.
    pub fn new(info: DeviceInfo, device: hidapi::HidDevice) -> Self {
        Self {
            info,
            device: Arc::new(Mutex::new(device)),
        }
    }

    pub fn send_feature_report(&self, data: &[u8]) -> Result<()> {
        let device = self.device.lock();
        device.send_feature_report(data).map_err(crate::Error::from)?;
        Ok(())
    }
}

impl Device for GenericSpeaker {
    fn info(&self) -> &DeviceInfo {
        &self.info
    }

    fn device_type(&self) -> DeviceType {
        self.info.device_type
    }

    fn initialize(&mut self) -> Result<()> {
        Ok(())
    }

    fn close(&mut self) -> Result<()> {
        Ok(())
    }

    fn is_connected(&self) -> bool {
        // Basic check, depends on implementation
        true
    }

    fn send_raw(&mut self, data: &[u8]) -> Result<()> {
        let device = self.device.lock();
        device.write(data).map_err(crate::Error::from)?;
        Ok(())
    }


    fn receive_raw(&mut self, buf: &mut [u8]) -> Result<usize> {
        let device = self.device.lock();
        device.read(buf).map_err(crate::Error::from)
    }
}

#[async_trait]
impl Speaker for GenericSpeaker {
    fn set_static_color(&mut self, _colors: &[[u8; 3]; 4]) -> Result<()> {
        Err(crate::Error::DeviceCommunication("set_static_color not supported".into()))
    }

    fn set_dynamic_colorshift(&mut self, _colors: &[[u8; 3]; 4]) -> Result<()> {
        Err(crate::Error::DeviceCommunication("set_dynamic_colorshift not supported".into()))
    }

    fn set_equalizer(&mut self, _bands: &[u8; 10]) -> Result<()> {
        Err(crate::Error::DeviceCommunication("set_equalizer not supported".into()))
    }
}
