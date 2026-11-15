//! SigmaOS-acceleration: hardware acceleration model.
//!
//! Real implementation: GPU, compute units, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Compute device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputeDevice {
    pub id: String,
    pub name: String,
    pub device_type: String,
    pub capability: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccelerationError {
    EmptyName,
    DuplicateDevice(String),
}

impl fmt::Display for AccelerationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "device name must not be empty"),
            Self::DuplicateDevice(id) => write!(f, "duplicate device {:?}", id),
        }
    }
}

impl std::error::Error for AccelerationError {}

/// Hardware acceleration manager.
pub struct HardwareAcceleration {
    pub name: String,
    devices: BTreeMap<String, ComputeDevice>,
}

impl HardwareAcceleration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            devices: BTreeMap::new(),
        }
    }

    pub fn add_device(&mut self, device: ComputeDevice) -> Result<(), AccelerationError> {
        if device.name.trim().is_empty() { return Err(AccelerationError::EmptyName); }
        let id = device.id.clone();
        if self.devices.contains_key(&id) {
            return Err(AccelerationError::DuplicateDevice(id.clone()));
        }
        self.devices.insert(id, device);
        Ok(())
    }

    pub fn device_count(&self) -> usize { self.devices.len() }

    pub fn get_device(&self, id: &str) -> Option<&ComputeDevice> { self.devices.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acceleration_create_aaa() {
        let a = HardwareAcceleration::new("My Acceleration");
        assert_eq!(a.name, "My Acceleration");
        assert_eq!(a.device_count(), 0);
    }

    #[test]
    fn test_add_device_aaa() {
        let mut a = HardwareAcceleration::new("My Acceleration");
        let d = ComputeDevice { id: "d1".into(), name: "gpu0".into(), device_type: "gpu".into(), capability: "cuda".into() };
        a.add_device(d).unwrap();
        assert_eq!(a.device_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut a = HardwareAcceleration::new("My Acceleration");
        let d = ComputeDevice { id: "x".into(), name: "".into(), device_type: "a".into(), capability: "b".into() };
        assert!(a.add_device(d).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut a = HardwareAcceleration::new("My Acceleration");
        let d = ComputeDevice { id: "x".into(), name: "X".into(), device_type: "a".into(), capability: "b".into() };
        a.add_device(d).unwrap();
        let d2 = ComputeDevice { id: "x".into(), name: "X".into(), device_type: "a".into(), capability: "b".into() };
        assert!(a.add_device(d2).is_err());
    }
}
